use std::fs;
use regex::Regex;

pub fn read_input(filename: String) -> Vec<String> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let passports: Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string() ).collect();

  passports
}

// const fields: [&str;7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const FIELDS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
lazy_static! {
  static ref YEAR_REGEX: Regex = Regex::new(r"^\d{4}$").unwrap();
  static ref HEIGHT_REGEX: Regex = Regex::new(r"^\d{1,3}(cm|in)$").unwrap();
  static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  static ref PASSPORTNUMBER_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
}

// byr ^\d{4}$ and >=1920 <=2002
pub fn validate_birthyear(s: &str) -> bool {
  let  retval;

  // let y = Regex::new(r"^\d{4}$").unwrap();
  if YEAR_REGEX.is_match(s) {
    let year = s.parse::<i32>().unwrap();
    if year >= 1920 && year <= 2002 {
      retval = true;
    } else {
      retval = false;
    }
  } else {
    retval = false;
  }

  retval
}

// iyr ^same and >= 2010 <=2020
pub fn validate_issueyear(s: &str) -> bool {
  let retval;

  if YEAR_REGEX.is_match(s) {
    let year = s.parse::<i32>().unwrap();
    if year >= 2010 && year <= 2020 {
      retval = true;
    } else {
      retval = false;
    }
  } else {
    retval = false;
  }

  retval
}

// eyr ^same and >= 2020 <=2030
pub fn validate_expireyear(s: &str) -> bool {
  let  retval;

  if YEAR_REGEX.is_match(s) {
    let year = s.parse::<i32>().unwrap();
    if year >= 2020 && year <= 2030 {
      retval = true;
    } else {
      retval = false;
    }
  } else {
    retval = false;
  }

  retval
}

// hgt ^\d{1,3}(cm|in)$ cm 150-193 incl; in 59-76 incl
pub fn validate_height(s: &str) -> bool {
  let retval;

  if HEIGHT_REGEX.is_match(s) {
    let value = &s[..s.len()-2];
    let units = &s[s.len()-2..];
    let height = value.parse::<i32>().unwrap();
    // println!("value {} and units {}", value, units);
    if units == "in" {
      retval = (height >= 59) && (height <= 76);
    } else if units == "cm" {
      retval = (height >= 150) && (height <= 193);
    } else {
      retval = false;
    }
  } else {
    retval = false;
  }

  retval
}
// hcl ^#(0-9a-f){6}
pub fn validate_haircolor(s: &str) -> bool {
  let retval;

  if COLOR_REGEX.is_match(s) {
    retval = true;
  } else {
    retval = false;
  }

  retval

}
// ecl ^(amb|blu|brn|gry|grn|hzl|oth)
pub fn validate_eyecolor(s: &str) -> bool {
  let  retval;
  if s == "amb" || s=="blu" || s == "brn" || s=="gry" || s=="grn" || s=="hzl" || s=="oth" {
    retval = true;
  } else {
    retval = false;
  }
  retval
}
// pid ^d{9}$
pub fn validate_pid(s: &str) -> bool {
  let  retval;

  if PASSPORTNUMBER_REGEX.is_match(s) {
    retval = true;
  } else {
    retval = false;
  }

  retval

}

fn validate_passport(p:&String) -> bool {

  for field in p.split_ascii_whitespace() {
    let key_value = field.split(":").collect::<Vec<&str>>();
    let key = key_value[0];
    let value = key_value[1];
    // println!("current validity is {} - Checking {} of {}", valid, key, value);
    if key == "byr" {
      if !validate_birthyear(value) { return false; }
    } else if key == "iyr" {
      if !validate_issueyear(value) { return false; }
    } else if key == "eyr" {
      if !validate_expireyear(value) { return false; }
    } else if key == "hgt" {
      if !validate_height(value) { return false; }
    } else if key == "hcl" {
      if !validate_haircolor(value) { return false; }
    } else if key == "ecl" {
      if !validate_eyecolor(value) { return false; }
    } else if key == "pid" {
      if !validate_pid(value) { return false; }
    } else if key == "cid" {
      // ignore
    } else {
      println!("Problem {}", key);
    }
  }

  true
}

pub fn solve(passports: &Vec<String>, validate: bool) {
  let mut passport_count = 0;

  for p in passports {
    let mut field_count = 0;
    for field in FIELDS.iter() {
      if p.find(field) != None {
        field_count += 1;
      }
    }
    if FIELDS.len() == field_count {
      if validate && validate_passport(p) {
        passport_count += 1;
      } 

      if !validate {
        passport_count += 1;
      }
    }

  }
  println!("valid passports with validation {} is {}", validate, passport_count);
}