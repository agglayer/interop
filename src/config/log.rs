use std::path::PathBuf;

use serde::{Deserialize, Deserializer};
use tracing_subscriber::fmt::writer::BoxMakeWriter;

/// The log configuration.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Log {
    #[serde(default)]
    pub(crate) level: LogLevel,
    pub(crate) outputs: Vec<LogOutput>,
}

/// The log level.
#[derive(Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    /// Get the log level as a string.
    ///
    /// This is used to set the `RUST_LOG` environment variable.
    pub(crate) fn as_str(&self) -> &str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Fatal => "fatal",
        }
    }
}

/// The log output.
///
/// This can be either `stdout`, `stderr`, or a file path.
///
/// The [`Deserialize`] implementation allows for the configuration file to
/// specify the output location as a string, which is then parsed into the
/// appropriate enum variant. If the string is not recognized to be either
/// `stdout` or `stderr`, it is assumed to be a file path.
#[derive(Debug, Clone, Default)]
pub(crate) enum LogOutput {
    #[default]
    Stdout,
    Stderr,
    File(PathBuf),
}

impl<'de> Deserialize<'de> for LogOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // If the string is not recognized to be either `stdout` or `stderr`,
        // it is assumed to be a file path.
        match s.as_str() {
            "stdout" => Ok(LogOutput::Stdout),
            "stderr" => Ok(LogOutput::Stderr),
            _ => Ok(LogOutput::File(PathBuf::from(s))),
        }
    }
}

impl LogOutput {
    /// Get a [`BoxMakeWriter`] for the log output.
    ///
    /// This can be used to plug the log output into the tracing subscriber.
    pub(crate) fn as_make_writer(&self) -> BoxMakeWriter {
        match self {
            LogOutput::Stdout => BoxMakeWriter::new(std::io::stdout),
            LogOutput::Stderr => BoxMakeWriter::new(std::io::stderr),
            LogOutput::File(path) => {
                let appender = tracing_appender::rolling::never(".", path);
                BoxMakeWriter::new(appender)
            }
        }
    }
}
