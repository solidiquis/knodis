use clap::ValueEnum;
use std::{
    convert::AsRef,
    fmt::{self, Display},
};

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Environment {
    #[default]
    Development,
    Production,
    Staging,
}

macro_rules! environment_string_rep {
    ($i:ident) => {
        match $i {
            Self::Development => "development",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    };
}

impl AsRef<str> for Environment {
    fn as_ref(&self) -> &str {
        environment_string_rep!(self)
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rep = environment_string_rep!(self);
        write!(f, "{rep}")
    }
}
