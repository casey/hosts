use crate::common::*;

#[derive(Deserialize)]
pub(crate) struct Config {
  common: Option<Profile>,
  profiles: BTreeMap<String, Profile>,
}

impl Config {
  pub(crate) fn load() -> Result<Config, Error> {
    let path = PathBuf::from("/Users/rodarmor/.hosts");
    let text = fs::read_to_string(&path).context(error::ConfigRead { path: &path })?;
    let config = serde_yaml::from_str(&text).context(error::ConfigParse { path: &path })?;
    Ok(config)
  }

  pub(crate) fn profile(&self, name: &str) -> Result<Profile, Error> {
    if let Some(profile) = self.profiles.get(name) {
      return if let Some(common) = &self.common {
        Ok(common.merge(profile))
      } else {
        Ok(profile.clone())
      };
    }

    Err(Error::ProfileUnknown {
      name: name.to_owned(),
    })
  }
}
