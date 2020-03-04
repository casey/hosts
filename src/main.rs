use crate::common::*;

mod common;
mod config;
mod error;
mod opt;
mod profile;
mod record;

fn main() -> Result<(), Error> {
  Opt::from_args().run()
}
