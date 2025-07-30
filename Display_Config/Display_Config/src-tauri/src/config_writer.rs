use crate::config_parser::ConfigEntry;
use std::io::Write;
use std::{fs, io, path::PathBuf};

pub fn write_config_to_file(path: PathBuf, entries: &[ConfigEntry]) -> io::Result<()> {
    let file = fs::File::create(path)?;
    write_config(file, entries)
}

pub fn write_config<W: Write>(mut writer: W, entries: &[ConfigEntry]) -> io::Result<()> {
    for (index, entry) in entries.iter().enumerate() {
        writeln!(writer, "{}", entry.entry_type)?;
        writeln!(writer, "{{")?;
        for (key, value) in &entry.entries {
            writeln!(writer, "\t{key} \t= {value};")?;
        }

        write!(writer, "}};")?;
        if index < entries.len() - 1 {
            writeln!(writer, "\n")?; // Add a newline between entries
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config_parser::parse_config_str;

    use super::*;
    #[test]
    pub fn test_write() {
        let test_data = r#"player
{
	bone_config_file 	= "data/characters/plrbnecnf_2.txt";
	mesh_config_file 	= "data/characters/keith/plrmshcnf.txt";
};

player
{
	bone_config_file 	= "data/characters/plrbnecnf_1.txt";
	mesh_config_file 	= "data/characters/akiko/plrmshcnf.txt";
};"#;

        let config = parse_config_str(test_data).unwrap();
        let mut buffer = Vec::new();
        write_config(&mut buffer, &config).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, test_data);
    }

    #[test]
    pub fn test_write_with_spaces() {
        let test_data = r#"player
{
	bone_config_file 	= "data/characters/plrbnecnf_2.txt";
	mesh_config_file 	= "data/characters/keith/plrmshcnf.txt";
};

player
{
	bone_config_file 	= "data/characters/plrbnecnf_1.txt";
	mesh_config_file 	= "data/characters/akiko/plrmshcnf.txt";
};"#;

        let config = parse_config_str(test_data).unwrap();
        let mut buffer = Vec::new();
        write_config(&mut buffer, &config).unwrap();

        let output = String::from_utf8(buffer).unwrap();

        assert_eq!(output, test_data);
    }
}
