use std::collections::HashMap;

fn solve_part1(nums: &Vec<i32>) -> i32 {
  let mut retval = 0;

  return retval;
}

fn solve_part2(nums: &Vec<i32>) -> i32 {
  let mut retval = 0;

  return retval;
}

pub fn solve() {
  let lines: Vec<String> = include_str!("../inputs/dec11.txt").lines().map(|s| (&*s).to_string() ).collect();

  let mut nums : Vec<i32> = lines.iter().map(|line| line.parse::<i32>().unwrap()).collect();

  // nums.sort();

  // solve logic
  solve_part1(&nums);
  solve_part2(&nums);
}

