use crate::common::*;

pub(crate) struct Record {
  address: String,      // TODO: switch to Ipv4/ipv6 address
  host: String,         // TODO: Switch to domain
  aliases: Vec<String>, // todo: switch to structured
}

impl Display for Record {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}\t{}", self.address, self.host)?;

    for alias in &self.aliases {
      write!(f, " {}", alias)?;
    }

    writeln!(f)?;

    Ok(())
  }
}
