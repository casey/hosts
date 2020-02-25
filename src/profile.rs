use crate::common::*;

#[serde(transparent)]
#[derive(Deserialize, Clone)]
pub(crate) struct Profile {
  #[serde(with = "serde_with::rust::display_fromstr")]
  hosts: Hosts,
}

impl Profile {
  pub(crate) fn merge(&self, other: &Profile) -> Profile {
    Profile {
      hosts: self
        .hosts
        .records()
        .iter()
        .chain(other.hosts.records())
        .collect(),
    }
  }

  pub(crate) fn text(&self) -> &str {
    &self.text
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn merge() {
    let a = Profile {
      text: "\n\na\n\n".to_owned(),
    };

    let b = Profile {
      text: "\n\nb\n\n".to_owned(),
    };

    let c = a.merge(&b);
    assert_eq!(c.text, "a\nb\n");
  }
}
