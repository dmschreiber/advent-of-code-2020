use std::time::{Instant};
// use rayon::prelude::*;
use std::collections::HashMap;

fn check_permutations(nums: &Vec<i32>, start : i32, my_rating: i32, cached_permutations: &mut HashMap<i32,i64>) -> i64 {
   let vec = vec![start, start+1, start+2];
 
  return 
    vec.iter().map(|n|     
    if nums.contains(&n) {
      if n + 3 == my_rating { 
        1 as i64
      } else if cached_permutations.get(n) != None {
        *cached_permutations.get(n).unwrap()
      } else {
        let tmp = check_permutations(nums, n+1, my_rating, cached_permutations);
        cached_permutations.insert(*n,tmp);
        tmp
      }
    } else {
      0
    }
    ).sum::<i64>();
  
}

pub fn solve() {
  let lines: Vec<String> = include_str!("../inputs/dec10.txt").lines().map(|s| (&*s).to_string() ).collect();
  let start = Instant::now();

  let mut nums : Vec<i32> = lines.iter().map(|line| line.parse::<i32>().unwrap()).collect();

  nums.sort();

  // solve logic
  let my_rating = nums.iter().max().unwrap() + 3;
  let mut last = 0;
  let mut diff;
  let mut one_joltage = 0;
  let mut three_joltage = 0;

  for n in &nums {
    // println!("joltage {} max {} with {}/{}", n, my_rating,one_joltage, three_joltage);
    diff = n - last;
    if diff == 1 {
      one_joltage += 1;
    } else if diff == 3 {
      three_joltage += 1;
    }
    last = *n;
    
    if *n == my_rating {
      break;
    }
  }

  three_joltage += 1;
  println!("one joltage={}, three joltage={}, product {}", one_joltage, three_joltage, one_joltage*three_joltage);

  let mut cached_permutations = <HashMap<i32,i64>>::new();

  let num_arrangements = check_permutations(&nums, 1, my_rating, &mut cached_permutations);
  println!("Day 10 num arrangements {} in {:?}", num_arrangements, start.elapsed());
}