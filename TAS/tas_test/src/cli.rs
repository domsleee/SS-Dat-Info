//! Flag parsing shared by every mode.
//!
//! One parser so every mode rejects what it does not understand: an unknown
//! flag, a missing value or a non-numeric value is an error (exit 2), never a
//! silently substituted default.

use std::fmt::Display;
use std::str::FromStr;

pub struct Spec {
    name: &'static str,
    alias: Option<&'static str>,
    values: usize,
}

/// A flag that takes one value, e.g. `--iterations 5` / `-n 5`.
pub const fn flag(name: &'static str, alias: Option<&'static str>) -> Spec {
    Spec {
        name,
        alias,
        values: 1,
    }
}

/// A flag that takes no value, e.g. `--verbose`.
pub const fn switch(name: &'static str, alias: Option<&'static str>) -> Spec {
    Spec {
        name,
        alias,
        values: 0,
    }
}

/// A flag that takes two values, e.g. `--at X Y`.
pub const fn pair(name: &'static str) -> Spec {
    Spec {
        name,
        alias: None,
        values: 2,
    }
}

#[derive(Debug, Default)]
pub struct Flags {
    given: Vec<(&'static str, Vec<String>)>,
    pub positional: Vec<String>,
}

/// Parse `args` against `specs`, allowing up to `max_positional` bare arguments.
pub fn parse(args: &[String], specs: &[Spec], max_positional: usize) -> Result<Flags, String> {
    let mut flags = Flags::default();
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        match specs
            .iter()
            .find(|s| s.name == arg || s.alias == Some(arg))
        {
            Some(spec) => {
                let Some(values) = args.get(i + 1..i + 1 + spec.values) else {
                    return Err(format!(
                        "{} needs {} value{}",
                        spec.name,
                        spec.values,
                        if spec.values == 1 { "" } else { "s" }
                    ));
                };
                flags.given.push((spec.name, values.to_vec()));
                i += 1 + spec.values;
            }
            None if arg.starts_with('-') && arg.len() > 1 => {
                return Err(format!("unknown flag '{arg}'"));
            }
            None => {
                if flags.positional.len() >= max_positional {
                    return Err(format!("unexpected argument '{arg}'"));
                }
                flags.positional.push(arg.to_string());
                i += 1;
            }
        }
    }
    Ok(flags)
}

impl Flags {
    pub fn is_set(&self, name: &str) -> bool {
        self.given.iter().any(|(n, _)| *n == name)
    }

    /// The last value given for `name` (later occurrences win).
    pub fn value(&self, name: &str) -> Option<&str> {
        self.values(name).and_then(|v| v.first()).map(String::as_str)
    }

    pub fn values(&self, name: &str) -> Option<&[String]> {
        self.given
            .iter()
            .rev()
            .find(|(n, _)| *n == name)
            .map(|(_, v)| v.as_slice())
    }

    /// Numeric flag with a default when absent; a non-numeric value is an error.
    pub fn num<T: FromStr>(&self, name: &str, default: T) -> Result<T, String> {
        match self.value(name) {
            None => Ok(default),
            Some(raw) => parse_num(name, raw),
        }
    }

    /// Numeric positional argument, `None` when absent.
    pub fn positional_num<T: FromStr>(&self, index: usize) -> Result<Option<T>, String> {
        match self.positional.get(index) {
            None => Ok(None),
            Some(raw) => parse_num(&format!("argument {}", index + 1), raw).map(Some),
        }
    }
}

fn parse_num<T: FromStr>(what: &str, raw: &str) -> Result<T, String> {
    raw.parse()
        .map_err(|_| format!("{what}: expected a number, got '{raw}'"))
}

/// Report a command-line problem and exit 2.
pub fn usage_error(message: impl Display) -> ! {
    eprintln!("ERROR: {message}");
    std::process::exit(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    const SPECS: &[Spec] = &[
        flag("--iterations", Some("-n")),
        switch("--verbose", Some("-v")),
        pair("--at"),
    ];

    #[test]
    fn unknown_flag_is_an_error_naming_the_flag() {
        let err = parse(&args(&["--iteration", "5"]), SPECS, 0).unwrap_err();
        assert!(err.contains("--iteration"), "{err}");
    }

    #[test]
    fn non_numeric_value_is_an_error_not_a_default() {
        let flags = parse(&args(&["--iterations", "abc"]), SPECS, 0).unwrap();
        let err = flags.num::<u32>("--iterations", 5).unwrap_err();
        assert!(err.contains("--iterations") && err.contains("abc"), "{err}");
        let flags = parse(&args(&["xyz"]), SPECS, 1).unwrap();
        assert!(flags.positional_num::<u32>(0).is_err());
    }

    #[test]
    fn aliases_switches_defaults_and_pairs() {
        let flags = parse(&args(&["-n", "7", "-v", "--at", "10", "20"]), SPECS, 0).unwrap();
        assert_eq!(flags.num("--iterations", 5u32).unwrap(), 7);
        assert!(flags.is_set("--verbose"));
        assert_eq!(flags.values("--at").unwrap(), &["10", "20"]);
        let flags = parse(&args(&[]), SPECS, 0).unwrap();
        assert_eq!(flags.num("--iterations", 5u32).unwrap(), 5);
        assert!(!flags.is_set("--verbose"));
        assert_eq!(flags.positional_num::<u32>(0).unwrap(), None);
    }

    #[test]
    fn missing_values_and_extra_positionals_are_errors() {
        assert!(parse(&args(&["--iterations"]), SPECS, 0).is_err());
        assert!(parse(&args(&["--at", "1"]), SPECS, 0).is_err());
        assert!(parse(&args(&["a", "b"]), SPECS, 1).is_err());
        let flags = parse(&args(&["a", "12"]), SPECS, 2).unwrap();
        assert_eq!(flags.positional, ["a", "12"]);
        assert_eq!(flags.positional_num::<u64>(1).unwrap(), Some(12));
    }
}
