use std::collections::HashMap;
use std::cmp;

pub fn solve() {
  let lines: Vec<String> = include_str!("../inputs/dec12.txt").lines().map(|s| (&*s).to_string() ).collect();

  let mut map = <HashMap<(i32,i32),u8>>::new();
}