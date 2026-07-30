pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut matches = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      matches.push(line);
    }
  }

  return matches;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let content = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, content));
  }
}
