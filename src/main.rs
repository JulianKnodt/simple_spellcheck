use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BTreeSet;
use std::str;
use std::env;

const DICT_FILE: &str = "/usr/share/dict/words";
fn main() {
  // need to read in file to spell check

  let file = File::open(DICT_FILE).unwrap();
  let mut word_set = BTreeSet::new();
  let r = BufReader::new(file);
  r.lines().for_each(|w| {
    word_set.insert(w.unwrap());
  });

  for arg in env::args().skip(1) {
    let file = match File::open(arg.clone()) {
      Ok(s) => s,
      Err(e) => {
        println!("! Could not open {}, {}, skipping...", arg, e);
        break;
      },
    };
    BufReader::new(file)
      .lines()
      .enumerate()
      .for_each(|(i, s)|
        s.unwrap()
          .split_whitespace()
          .filter_map(|word| {
            let trimmed = word.trim_matches(|c: char| !c.is_alphanumeric()).to_string();
            if word_set.contains(&trimmed.to_lowercase()) || trimmed.is_empty() {
              None
            } else {
              Some(trimmed)
            }
          })
          .for_each(|not_found| println!("{}, line {}:{}", arg, i, not_found))
      )
  }
}
