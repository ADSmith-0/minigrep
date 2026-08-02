use std::error::Error;
use std::{env, fs, process};

use minigrep::{Case, search};

fn main() {
  let args = Args::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing your arguments: {err}");
    process::exit(1);
  });

  if let Err(e) = run(args) {
    eprintln!("Application Error: {e}");
    process::exit(1);
  };
}

struct Args {
  query: String,
  file_path: String,
  case: Case,
}

impl Args {
  fn new(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
    args.next();

    let Some(query) = args.next() else {
      return Err("Missing search query");
    };

    let Some(file_path) = args.next() else {
      return Err("Missing file path");
    };

    let ignore_case = get_ignore_case(args.next());

    Ok(Args {
      query,
      file_path,
      case: ignore_case,
    })
  }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(args.file_path)?;

  let results = search(&args.query, &contents, &args.case);

  for line in results {
    println!("{line}");
  }

  Ok(())
}

fn get_ignore_case(arg: Option<String>) -> Case {
  const IGNORE_FLAG: &str = "-i";

  if let Some(x) = arg {
    if x.eq(IGNORE_FLAG) {
      return Case::Insensitive;
    };
    return Case::Sensitive;
  };

  let Ok(env_var) = env::var("IGNORE_CASE") else {
    return Case::Sensitive;
  };

  let value: i32 = env_var.parse().unwrap_or(0);

  if value > 0 {
    return Case::Insensitive;
  }
  return Case::Sensitive;
}
