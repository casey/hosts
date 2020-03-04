use crate::common::*;

#[snafu(visibility(pub(crate)))]
#[derive(Snafu, Debug)]
pub(crate) enum Error {
  ConfigRead {
    source: io::Error,
    path: PathBuf,
  },
  ConfigParse {
    source: yaml_rust::ScanError,
    path: PathBuf,
  },
  ConfigDeserialize {
    source: serde_yaml::Error,
    path: PathBuf,
  },
  HostsWrite {
    source: io::Error,
    path: PathBuf,
  },
  ProfileUnknown {
    name: String,
  },
}
