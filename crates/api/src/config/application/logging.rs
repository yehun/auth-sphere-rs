use std::fmt::Display;
use std::path::PathBuf;
use serde::Deserialize;

fn convert_level(level: &str) -> tracing::Level {
    match level {
        "TRACE" => tracing::Level::TRACE,
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        "ERROR" => tracing::Level::ERROR,
        _ => tracing::Level::DEBUG,
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Logging {
    pub level: Option<String>,
    pub console: Option<LoggingConsole>,
    pub file: Option<LoggingFile>
}

impl Logging {
    pub fn get_level(&self) -> tracing::Level {
        self.level.clone()
            .map(|x| convert_level(&x))
            .unwrap_or(tracing::Level::TRACE)
    }
    pub fn get_level_with_console(&self) -> tracing::Level {
        let level = self.console.clone()
            .map(|x| x.get_level())
            .unwrap_or(None);
        level.map(|x| x).unwrap_or(self.get_level())
    }
    pub fn get_level_with_file(&self) -> tracing::Level {
        let level = self.file.clone()
            .map(|x| x.get_level())
            .unwrap_or(None);
        level.map(|x| x).unwrap_or(self.get_level())
    }
}

impl Display for Logging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(console) = &self.console {
            write!(f, "{}\n", console)?;
        }
        if let Some(file) = &self.file {
            write!(f, "{}", file)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LoggingConsole {
    pub level: Option<String>,
}

impl Display for LoggingConsole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logging.console.level: {}", self.level.clone().unwrap_or("none".to_string()))
    }
}


impl LoggingConsole {
    pub fn get_level(&self) -> Option<tracing::Level> {
        self.level.clone().map(|x| convert_level(&x))
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LoggingFile {
    pub level: Option<String>,
    pub path: PathBuf,
    pub prefix: String,
}

impl LoggingFile {
    pub fn get_level(&self) -> Option<tracing::Level> {
        self.level.clone().map(|x| convert_level(&x))
    }
}

impl Display for LoggingFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logging.file.level: {}\n", self.level.clone().unwrap_or("none".to_string()))?;
        write!(f, "logging.file.path: {}\n", self.path.clone().to_str().unwrap_or(""))?;
        write!(f, "logging.file.prefix: {}", self.prefix.clone())
    }
}
