use crate::common::*;

mod common;
mod config;
mod error;
mod hosts;
mod opt;
mod profile;
mod record;

lalrpop_mod!(parser);

fn main() -> Result<(), Error> {
  Opt::from_args().run()
}
