use crate::common::*;

#[derive(Deserialize, Debug, PartialEq, Default)]
pub(crate) struct Config {
  #[serde(default)]
  common: Option<Profile>,
  #[serde(default)]
  profiles: Option<BTreeMap<String, Profile>>,
}

impl Config {
  pub(crate) fn load() -> Result<Config> {
    let path = PathBuf::from("/Users/rodarmor/.hosts");
    let text = fs::read_to_string(&path).context(error::ConfigRead { path: &path })?;
    let config = Self::parse(&text, &path)?;
    Ok(config)
  }

  fn parse(text: &str, path: impl AsRef<Path>) -> Result<Config> {
    let path = path.as_ref();
    let documents = YamlLoader::load_from_str(text).context(error::ConfigParse { path })?;
    if documents.is_empty() {
      return Ok(Config::default());
    }
    serde_yaml::from_str(&text).context(error::ConfigDeserialize { path })
  }

  pub(crate) fn profile(&self, name: &str) -> Result<Profile, Error> {
    if let Some(profiles) = self.profiles.as_ref() {
      if let Some(profile) = profiles.get(name) {
        return if let Some(common) = &self.common {
          Ok(common.merge(profile))
        } else {
          Ok(profile.clone())
        };
      }
    }

    Err(Error::ProfileUnknown {
      name: name.to_owned(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn case(text: &str, want: Config) {
    let have = Config::parse(text, "<TEST CONFIG>").unwrap();
    assert_eq!(have, want);
  }

  #[test]
  fn empty() {
    case("", Config::default());
  }

  #[test]
  fn empty_common() {
    case("common:", Config::default());
  }

  #[test]
  fn empty_profiles() {
    case("profiles:", Config::default());
  }
}
