use crate::common::*;

#[derive(Clone)]
pub(crate) struct Hosts {
  records: Vec<Record>,
}

impl Hosts {
  pub(crate) fn new(records: Vec<Record>) -> Hosts {
    Hosts { records }
  }

  pub(crate) fn records(&self) -> &[Record] {
    &self.records
  }
}

impl Display for Hosts {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    for record in self.records {
      writeln!(f, "{}", record)?
    }

    Ok(())
  }
}

impl FromStr for Hosts {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let parser = crate::parser::HostsParser::new();

    let hosts = parser.parse(text)?;

    Ok(hosts)
  }
}
