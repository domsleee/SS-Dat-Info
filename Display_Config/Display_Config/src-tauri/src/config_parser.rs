use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigEntry {
    pub entry_type: String,
    pub entries: Vec<(String, String)>,
}

pub fn parse_config<P: AsRef<Path>>(path: P) -> io::Result<Vec<ConfigEntry>> {
    let content = fs::read_to_string(path)?;
    parse_config_str(&content)
}
pub fn parse_config_str(content: &str) -> io::Result<Vec<ConfigEntry>> {
    let mut result = Vec::new();
    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Extract entry type
        let entry_type = if line.ends_with('{') {
            line.trim_end_matches('{').trim().to_string()
        } else {
            // peek next non-empty line for '{'
            let entry_type = line.to_string();
            loop {
                match lines.peek() {
                    Some(next_line) if next_line.trim().is_empty() => {
                        lines.next();
                        continue;
                    }
                    Some(next_line) if next_line.trim() == "{" => {
                        lines.next();
                        break;
                    }
                    Some(unexpected_line) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Expected '{{', but found '{}'", unexpected_line.trim()),
                        ));
                    }
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Unexpected end of file while looking for '{'",
                        ));
                    }
                }
            }
            entry_type
        };

        let mut entries = Vec::new();

        for line in lines.by_ref() {
            let line = line.trim();

            if line.starts_with('}') {
                break;
            }

            if line.is_empty() || !line.contains('=') {
                continue;
            }

            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() >= 2 {
                let key = parts[0].trim().to_string();

                let joined_value = parts[1..].join("=");
                let value_part = joined_value.trim();
                let value = if let Some(stripped) = value_part.strip_suffix(';') {
                    stripped.trim()
                } else {
                    value_part
                };

                let value = value.to_string();
                entries.push((key, value));
            }
        }

        result.push(ConfigEntry {
            entry_type,
            entries,
        });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let test_data = r#"
            player
            {
                bone_config_file = "data/characters/plrbnecnf_2.txt";
                mesh_config_file = "data/characters/keith/plrmshcnf.txt";
            };
    
            player
            {
                bone_config_file = "data/characters/plrbnecnf_1.txt";
                mesh_config_file = "data/characters/akiko/plrmshcnf.txt";
            };"#;

        let config: Vec<ConfigEntry> = parse_config_str(test_data).unwrap();

        assert_eq!(config.len(), 2);

        assert_eq!(config[0].entry_type, "player");
        assert_eq!(
            config[0].entries[0],
            (
                "bone_config_file".to_string(),
                "\"data/characters/plrbnecnf_2.txt\"".to_string()
            )
        );
        assert_eq!(
            config[0].entries[1],
            (
                "mesh_config_file".to_string(),
                "\"data/characters/keith/plrmshcnf.txt\"".to_string()
            )
        );

        assert_eq!(config[1].entry_type, "player");
        assert_eq!(
            config[1].entries[0],
            (
                "bone_config_file".to_string(),
                "\"data/characters/plrbnecnf_1.txt\"".to_string()
            )
        );
        assert_eq!(
            config[1].entries[1],
            (
                "mesh_config_file".to_string(),
                "\"data/characters/akiko/plrmshcnf.txt\"".to_string()
            )
        );
    }

    #[test]
    fn test_parse_invalid_config_returns_error() {
        let test_data = r#"
        player
{
	bone_config_file 	= "data/characters/plrbnecnf_2.txt";
	mesh_config_file 	= "data/characters/bulk_30/plrmshcnf.txt";
	player_config_file 	= "data/characters/bulk_30/plrcnf.txt";
};
	player_config_file 	= "data/characters/bulk_29/plrcnf.txt";
};

player
{
	bone_config_file 	= "data/characters/plrbnecnf_2.txt";
	mesh_config_file 	= "data/characters/bulk_30/plrmshcnf.txt";
	player_config_file 	= "data/characters/bulk_30/plrcnf.txt";
};"#;
        let config = parse_config_str(test_data);
        assert!(config.is_err(), "Expected error, but got: {config:?}");
    }
}
