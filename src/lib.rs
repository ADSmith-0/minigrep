pub enum Case {
  Sensitive,
  Insensitive,
}

pub fn search<'a>(query: &str, contents: &'a str, case: &Case) -> impl Iterator<Item = &'a str> {
  contents.lines().filter(move |line| match case {
    Case::Insensitive => line.to_lowercase().contains(&query.to_lowercase()),
    Case::Sensitive => line.contains(query),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, content, &Case::Sensitive).collect::<Vec<&str>>()
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search(query, content, &Case::Insensitive).collect::<Vec<&str>>()
    );
  }
}
