use holo_config::{Config, Seed};

use docopt::Docopt;
use failure::Error;
use serde::*;
use sha2::{Digest, Sha512Trunc256};

use std::fs::File;
use std::path::PathBuf;
use std::{env, io};

const USAGE: &'static str = "
Usage: holo-config --email EMAIL --password STRING [--seed-from PATH]
       holo-config --help

Creates Holo config file that contains seed and admin email/password.

Options:
  --email EMAIL      HoloPort admin email address
  --password STRING  HoloPort admin password
  --seed-from PATH   Use SHA-512 hash of given file truncated to 256 bits as seed
";

#[derive(Deserialize)]
struct Args {
    flag_email: String,
    flag_password: String,
    flag_seed_from: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args().into_iter()).deserialize())
        .unwrap_or_else(|e| e.exit());

    let maybe_seed = match args.flag_seed_from {
        None => None,
        Some(path) => {
            let mut hasher = Sha512Trunc256::new();
            let mut file = File::open(path)?;
            io::copy(&mut file, &mut hasher)?;

            let seed: Seed = hasher.result().into();
            Some(seed)
        }
    };

    let config = Config::new(args.flag_email, args.flag_password, maybe_seed)?;
    println!("{}", serde_json::to_string_pretty(&config)?);

    Ok(())
}
