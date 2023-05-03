use anyhow::{Context, Result, bail};
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

// TODO: also support variables without = as seperator
fn parse_config_line(line: &str, path: &Path, line_number: usize) -> Result<(String, String)> {
    let mut parts = line.splitn(2, |c: char| c == '=');
    let name = parts.next()
        .with_context(|| format!("Failed to parse variable name on line {} while reading config file `{}`", line_number, path.display()))?
        .trim()
        .to_string();
    let value = parts.next()
        .with_context(|| format!("Failed to parse variable value on line {} while reading config file `{}`", line_number, path.display()))?
        .trim()
        .to_string();
    // strip optional comments of the value
    let value = value.splitn(2, |c: char| c == '#')
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    Ok((name, value))
}

#[derive(Debug)]
pub(crate) struct Config {
    name: String,
    log_level: u8,
    connect_to: HashSet<String>,
    port: Option<u16>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            log_level: 0,
            connect_to: HashSet::new(),
            port: None,
        }
    }
}

fn check_name(val: &str, path: &Path, line_number: usize) -> Result<()> {
    if val.is_empty() {
        bail!("Empty name on line {} while reading config file `{}`", line_number, path.display());
    }
    // is alphanum or _
    if !val.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        bail!("Invalid name '{}' on line {} while reading config file `{}`", val, line_number, path.display());
    }

    Ok(())
}


fn validate_config_line(config: &mut Config, name: &str, value: &str, path: &Path, line_number: usize) -> Result<()> {
    match name {
        "Name" => {
            check_name(value, path, line_number)?;
            config.name = value.to_string();
        },
        "LogLevel" => {
            let level = value.parse::<u8>()
                .with_context(|| format!("Failed to parse LogLevel on line {} while reading config file `{}`", line_number, path.display()))?;
            config.log_level = level;
        },
        "ConnectTo" => {
            config.connect_to.insert(value.to_string());
        },
        "Port" => {
            let port = value.parse::<u16>()
                .with_context(|| format!("Failed to parse Port on line {} while reading config file `{}`", line_number, path.display()))?;
            config.port = Some(port);
        },
        _ => {
            bail!("Unknown variable `{}` on line {} while reading config file `{}`", name, line_number, path.display());
        }
    }
    Ok(())
}


struct ConfigLine {
    name: String,
    value: String,
    line_number: usize,
}

fn read_config_file(path: &Path) -> Result<Vec<ConfigLine>> {
    // iterate line wise over the file
    let file = File::open(path).with_context(|| format!("could not read config file `{}`", path.display()))?;
    let reader = BufReader::new(file);
    let mut config = Vec::new();
    for (number, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("could not read config file `{}`", path.display()))?;
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let (key, value) = parse_config_line(&line, path, number)?;
        config.push(ConfigLine {
            name: key,
            value,
            line_number: number,
        });
    }
    Ok(config)
}

pub(crate) fn read_server_config(path: &Path) -> Result<Config> {
    let lines = read_config_file(path)?;
    let mut config = Config::default();
    for line in lines {
        validate_config_line(&mut config, &line.name, &line.value, path, line.line_number)?;
    }
    if config.name.is_empty() {
        bail!("No name specified in config file `{}`", path.display());
    }
    Ok(config)
}


#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_read_server_config() {
        let config = NamedTempFile::new().unwrap();
        config.as_file().write_all(r"
        Name = test
        # comment
        LogLevel = 3
        ConnectTo = host1
        ConnectTo = host2 # some comment

        ".as_bytes()).unwrap();
        let config = read_server_config(config.path()).unwrap();
        assert_eq!(config.log_level, 3);
        assert_eq!(config.connect_to.len(), 2);
        assert!(config.connect_to.contains("host1"));
        assert!(config.connect_to.contains("host2"));
    }
}
