use crate::common::*;

#[serde(transparent)]
#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub(crate) struct Profile {
  records: Vec<Record>,
}

impl Profile {
  fn insert(&mut self, domain: String, record: Record) {
    self.records.insert(domain, record);
  }

  pub(crate) fn merge(&self, other: &Profile) -> Profile {
    let mut profile = self.clone();

    profile.records.extend(other.records.clone());

    profile
  }

  pub(crate) fn text(&self) -> String {
    let mut text = String::new();

    for (domain, record) in &self.records {
      text.push_str(record.address());
      text.push('\t');
      text.push_str(&domain);
      for alias in record.aliases() {
        text.push('\t');
        text.push_str(&alias);
      }
      text.push('\n');
    }

    text
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn merge() {
    let a = serde_yaml::from_str::<Profile>(
      "
      a.com:
        address: 0.0.0.0
        aliases:
        - www.a.com
      b.com:
        address: 0.0.0.0
        aliases:
        - www.b.com
      ",
    )
    .unwrap();

    let b = serde_yaml::from_str::<Profile>(
      "
      b.com:
        address: 1.1.1.1
        aliases:
        - b.net
      ",
    )
    .unwrap();

    let have = a.merge(&b);

    let want = serde_yaml::from_str::<Profile>(
      "
      a.com:
        address: 0.0.0.0
        aliases:
        - www.a.com
      b.com:
        address: 1.1.1.1
        aliases:
        - b.net
      ",
    )
    .unwrap();

    assert_eq!(have, want);
  }
}
