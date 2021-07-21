use std::str::FromStr;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(structopt, derive(structopt::StructOpt))]
pub enum Mode {
    /// Checks all tracked issues. More efficient than running the track macros during build time,
    /// as it can asynchronously obtain all information.
    Pipe,
    /// Runs checks during build time. `Level::Warn` corresponds to build warnings, `Level::Error`
    /// to build errors.
    Emit(Level),
    /// Performs no actions after parsing the attribute.
    Noop,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pipe" => Ok(Mode::Pipe),
            "Emit(Warn)" => Ok(Mode::Emit(Level::Warn)),
            "Emit(Error)" => Ok(Mode::Emit(Level::Error)),
            _ => Err("unrecognized Mode"),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Emit(Level::Warn)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Level {
    Warn,
    Error,
}
