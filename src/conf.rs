pub(crate) mod basic_config;
pub(crate) mod error;
pub(crate) mod wrapper_config;

use super::*;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use toml;

pub use self::basic_config::BasicConfig;
pub use self::error::ConfigError;
pub use self::wrapper_config::WrapperConfig;

use crate::environment::{Environment, Environment::*};
use std::collections::HashMap;

const CONFIG_FILENAME: &str = "config/conf.toml";
pub type Result<T> = ::std::result::Result<T, ConfigError>;
