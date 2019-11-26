use std::path::PathBuf;
use structopt::StructOpt;

use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result::Result;

use cli_parsers::parse_bash;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "parse_bash_history",
  about = "Simple cli app to parse bash histories and output their parts"
)]
struct Opts {
  path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
  let opts: Opts = Opts::from_args();

  let file = File::open(opts.path)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let cmd = parse_bash(&line?)?;
    println!("{:?}", cmd);
  }

  Ok(())
}
