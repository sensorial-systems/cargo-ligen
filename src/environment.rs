mod arguments;
pub use arguments::*;

use crate::Error;
use std::{env, path::PathBuf};
use std::convert::TryFrom;
use ligen_core::generator::Arguments;


#[derive(Debug, Clone)]
pub struct RawArguments {
    pub values: Vec<String>
}

impl RawArguments {
    pub fn parse() -> Self {
        let values = std::env::args().skip(2).collect();
        Self { values }
    }

    pub fn find_pair<S: AsRef<str>>(&self, key: S) -> Option<String> {
        let mut iter = self.values.iter();
        while let Some(argument) = iter.next() {
            if key.as_ref() == *argument {
                return iter.next().cloned()
            }
        }
        None
    }

    pub fn find<S: AsRef<str>>(&self, key: S) -> Option<&String> {
        self.values.iter().find(|argument| key.as_ref() == *argument)
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub current_dir: PathBuf,
    pub raw_arguments: RawArguments,
    pub arguments: Arguments
}

impl Environment {
    pub fn parse() -> Result<Self, Error> {
        let current_dir = env::current_dir()?;
        let raw_arguments = RawArguments::parse();
        let arguments = Arguments::try_from(raw_arguments.clone())?;
        Ok(Self { current_dir, raw_arguments, arguments })
    }
}
