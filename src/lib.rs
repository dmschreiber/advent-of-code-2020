use std::fs;

pub fn read_input(filename: String) -> Vec<String> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines: Vec<String> = contents.lines().map(|s| (&*s).to_string() ).collect();

  lines
}
