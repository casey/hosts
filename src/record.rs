use crate::common::*;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub(crate) struct Record {
  address: String,      // TODO: switch to Ipv4/ipv6 address
  domain: String,       // TODO: Switch to structured host
  aliases: Vec<String>, // TODO: switch to structured
}

impl Record {
  pub(crate) fn address(&self) -> &str {
    &self.address
  }

  pub(crate) fn aliases(&self) -> &[String] {
    &self.aliases
  }
}
