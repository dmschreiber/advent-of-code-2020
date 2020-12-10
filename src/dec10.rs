use std::time::{Instant};


fn check_adapters(nums: &Vec<i64>, my_rating:i64, which_skip: i64) -> bool {
  let start = Instant::now();
  let mut last = 0;
  let mut diff = 0;

  for n in nums {
    if which_skip != *n {
      // println!("joltage {} max {} with {}/{}", n, my_rating,one_joltage, three_joltage);
      diff = *n - last;
      if diff<= 3 {
        // noop
      } else {
        return false;
      }
      last = *n;
      
      if *n + 3 == my_rating {
        return true;
      }
    }
  }
  // println!("check_adapters in {:?}", start.elapsed());

  return false;
}

fn copy_vec(nums: &Vec<i64>) -> Vec<i64> {
  let mut new_nums = <Vec<i64>>::new();

  for n in nums {
    new_nums.push(*n);
  }
  return new_nums;
}

fn check_permutations(nums: &Vec<i64>, start : i64, my_rating: i64, lvl:&str) -> i32 {

  let mut num_arrangements = 0;
  let mut i = 0;

  for n in nums {
    if nums.len() == 31 {
        println!("vector worked removing {} of {}", i, nums.len());
    }
    
    if check_adapters(&nums, my_rating,*n) {
      // println!("vector worked removing {} of {}", i, nums.len());
      let mut new_nums = copy_vec(nums);
      new_nums.remove(i);
      num_arrangements += 1;
      num_arrangements += check_permutations(&new_nums, my_rating, &format!("-{}",lvl));
    }
    i = i + 1;
  }
  // println!("{}Finished level {}", lvl, nums.len());
  return num_arrangements;
}

pub fn solve() {
  let lines: Vec<String> = include_str!("../inputs/dec10-test.txt").lines().map(|s| (&*s).to_string() ).collect();

  let mut nums : Vec<i64> = lines.iter().map(|line| line.parse::<i64>().unwrap()).collect();

  nums.sort();

  // solve logic
  let my_rating = nums.iter().max().unwrap() + 3;
  let mut last = 0;
  let mut diff = 0;
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
  // one_joltage += 1;
  three_joltage += 1;
  println!("one {}, three {}, product {}", one_joltage, three_joltage, one_joltage*three_joltage);

  let num_arrangements = check_permutations(&nums, my_rating,"");
  println!("test num arrangements is 19208 vs {}", num_arrangements);
}