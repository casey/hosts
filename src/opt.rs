use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  Switch { profile: String },
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    match self {
      Self::Switch { profile } => {
        let config = Config::load()?;

        let profile = config.profile(&profile)?;

        let hosts = PathBuf::from("/etc/hosts");

        fs::write(&hosts, profile.text()).context(error::HostsWrite { path: hosts })?;

        Ok(())
      }
    }
  }
}
