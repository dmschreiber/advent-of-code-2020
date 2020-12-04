use std::fs;

pub fn read_input(filename: String) -> Vec<String> {
  let mut passports = Vec::<String>::new();

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let splits = contents.split("\n\n");

  for split in splits {
    let s = String::from(split);
    passports.push(s);
  }
  // collect::<Vec<String>>();

  passports
}

// const fields: [&str;7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const FIELDS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn solve(passports: &Vec<String>) {
  let mut passport_count = 0;

  for p in passports {
    let mut field_count = 0;
    for field in FIELDS.iter() {
      if let Some(i) = p.find(field)  {
        field_count += 1;
        // println!("{}",i);
      }
    }
    if FIELDS.len() == field_count {
      passport_count += 1;
    }

  }
  println!("valid passports {}", passport_count);
}