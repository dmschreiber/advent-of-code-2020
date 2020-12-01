use std::fs;

pub fn read_input(filename: String) -> Vec<i32> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");
  let nums : Vec<i32> = contents.lines().map(|line| line.parse::<i32>().unwrap()).collect();
  nums
}

pub fn guess_hi_lo(nums: Vec<i32>, num: i32, hi: i32, lo: i32) -> i32 {
  // println!("num {}, low {}, hi {}", num, lo, hi);

  // let lo_index = lo as usize;
  // let hi_index = hi as usize;
  // let subset = &nums[lo_index..hi_index];
  // println!();
  // println!("Searching {:?}", subset);

  if nums[lo as usize] == num {
    return num;
  } else if nums[hi as usize] == num {
    return num;
  } else if (hi - lo) <= 1 {
    // println!("not found {}", num);
    return 0;
  }  else {
    let mid = (lo+hi)/2;
    if nums[mid as usize] >= num {
      return guess_hi_lo(nums, num, mid, lo);
    } else {
      return guess_hi_lo(nums,num, hi, mid);
    }
  }
}

pub fn solve_for_one(nums: Vec<i32>, num: i32) -> bool {
  // guess the middle and adjust until we find it's complement or fail
  let lo = 0;
  let hi = nums.len() as i32 - 1;
  if num < nums[0] { println!("not found {}", num); return false; }
  if num > nums[hi as usize] { println!("not found {}", num); return false; }

  let result = guess_hi_lo(nums, num, hi, lo);
  result == num
}

pub fn solve(nums: Vec<i32>) {
  // loop through each number
  for n in nums.iter() {
    // call solve for one
    // println!("SOLVING FOR {}", n);
    let m = 2020-n;
    if solve_for_one(nums.clone(),m) {
      println!("SOLUTION FOUND {},{} = {}", n, m, n*(2020-n));
      break;
    }
  }

  pub fn solve_part2(nums: Vec<i32>) {

  }

}