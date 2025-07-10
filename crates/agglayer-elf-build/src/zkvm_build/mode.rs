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

impl Mode {
    pub const DEFAULT_ENV_VAR: &str = "AGGLAYER_ELF_BUILD";

    pub fn from_env_var(var: &str) -> anyhow::Result<Self> {
        println!("cargo::rerun-if-env-changed={var}");
        match std::env::var(var) {
            Ok(mode_str) => mode_str.parse().context("Parsing mode env var"),
            Err(std::env::VarError::NotPresent) => Ok(Self::default()),
            Err(err) => anyhow::bail!("Malformed mode (from {var}): {err}"),
        }
    }

    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_env_var(Self::DEFAULT_ENV_VAR)
    }
}

impl std::str::FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.trim() {
            "build" => Ok(Self::Build),
            "refresh" | "update" => Ok(Self::Refresh),
            "" | "cached" => Ok(Self::Cached),
            mode_str => anyhow::bail!("Unrecognized mode {mode_str:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[rstest::rstest]
    #[case("", Ok(Mode::Cached))]
    #[case("-", Err(()))]
    #[case("cached", Ok(Mode::Cached))]
    #[case("build", Ok(Mode::Build))]
    #[case("refresh", Ok(Mode::Refresh))]
    #[case("update", Ok(Mode::Refresh))]
    #[case("what", Err(()))]
    fn config_parsing(#[case] s: &str, #[case] expected_mode: Result<Mode, ()>) {
        let mode: Result<Mode, ()> = s.parse().map_err(|_| ());
        assert_eq!(mode, expected_mode);
    }
}
