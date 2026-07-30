use std::error::Error;
use std::{env, fs, process};

use minigrep::{search, search_case_insensitive};

fn main() {
  let args: Vec<String> = env::args().collect();

  let args = Args::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing your arguments: {err}");
    process::exit(1);
  });

  if let Err(e) = run(args) {
    println!("Application Error: {e}");
    process::exit(1);
  };
}

struct Args<'a> {
  query: &'a str,
  file_path: &'a str,
  ignore_case: bool,
}

impl Args<'_> {
  fn new<'a>(args: &'a Vec<String>) -> Result<Args<'a>, &'static str> {
    let query = match args.get(1) {
      Some(a) => a,
      None => return Err("Missing search query"),
    };
    let file_path = match args.get(2) {
      Some(a) => a,
      None => return Err("Missing file path"),
    };

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Args {
      query,
      file_path,
      ignore_case,
    })
  }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(args.file_path)?;

  let results = if args.ignore_case {
    search_case_insensitive(&args.query, &contents)
  } else {
    search(&args.query, &contents)
  };

  for line in results {
    println!("{line}");
  }

  Ok(())
}
