use anyhow::Context;

/// Zkvm ELF build mode.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Mode {
    /// Build the zkvm program without updating the cached binary.
    Build,

    /// Build the zkvm program and update the cached binary.
    Refresh,

    /// Use the cached zkvm program binary.
    #[default]
    Cached,
}

impl std::str::FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "build" => Ok(Self::Build),
            "refresh" | "update" => Ok(Self::Refresh),
            "cached" => Ok(Self::Cached),
            mode_str => anyhow::bail!("Unrecognized mode {mode_str:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
    pub mode: Mode,
    pub extra_args: Vec<String>,
}

/// Dynamic builder configuration.
impl Config {
    pub const DEFAULT_ENV_VAR: &str = "AGGLAYER_BUILD";

    pub fn from_env_var(var: &str) -> anyhow::Result<Self> {
        println!("cargo::rerun-if-env-changed={var}");
        match std::env::var(var) {
            Ok(config_str) => config_str.parse().context("Parsing build env var"),
            Err(err) => match err {
                std::env::VarError::NotPresent => Ok(Self::default()),
                err => anyhow::bail!("Malformed env var {var}: {err}"),
            },
        }
    }

    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_env_var(Self::DEFAULT_ENV_VAR)
    }
}

impl std::str::FromStr for Config {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = shell_words::split(s)?.into_iter();
        match words.next() {
            None => Ok(Self::default()),
            Some(mode_str) => {
                let mode = mode_str.parse()?;
                let extra_args = words.collect();
                Ok(Self { mode, extra_args })
            }
        }
    }
}

impl From<Mode> for Config {
    fn from(mode: Mode) -> Self {
        let extra_args = Default::default();
        Self { mode, extra_args }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[rstest::rstest]
    #[case("", Mode::Cached)]
    #[case("cached", Mode::Cached)]
    #[case("build", Mode::Build)]
    #[case("refresh", Mode::Refresh)]
    #[case("update", Mode::Refresh)]
    #[case(
        "update --features=foo,bar",
        Config {
            mode: Mode::Refresh,
            extra_args: vec![String::from("--features=foo,bar")],
        },
    )]
    fn config_parsing_happy_path(#[case] s: &str, #[case] expected_config: impl Into<Config>) {
        let config: Config = s.parse().expect("parsing ok");
        assert_eq!(config, expected_config.into());
    }
}
