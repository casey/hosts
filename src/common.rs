pub(crate) use std::{
  collections::{BTreeMap, BTreeSet},
  fmt::{self, Display, Formatter},
  fs, io,
  path::{Path, PathBuf},
};

pub(crate) use crate::error;
pub(crate) use serde::Deserialize;
pub(crate) use serde_with::rust::unwrap_or_skip;
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use structopt::StructOpt;
pub(crate) use yaml_rust::YamlLoader;

pub(crate) use crate::{config::Config, error::Error, opt::Opt, profile::Profile, record::Record};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
