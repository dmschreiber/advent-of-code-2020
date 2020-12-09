
pub fn create_structure(lines: &Vec<String>) -> Vec<i64> {

  let nums : Vec<i64> = lines.iter().map(|line| line.parse::<i64>().unwrap()).collect();

  nums
}

fn is_in_list(nums: &[i64], num: i64) -> bool {
  for n in nums {
    for m in nums {
      if n != m {
        if num == n+m {
          return true
        }
      }
    }
  } 
  return false
}

pub fn solve(nums: &Vec<i64>) -> i64 {
  let mut retval = 0;
  for i in 25..nums.len() {

    let answer = is_in_list(&nums[i-25..i],nums[i]);

    if !answer {
      println!("Didn't find numbers that sum {}",nums[i]);
      retval = nums[i];
      break;
    }
  }
  retval
}

fn find(nums:&Vec<i64>, which: usize, target: i64) -> bool {

  for i in 0..nums.len() {
    let sum = nums[which..which+i].iter().sum::<i64>();

    if sum > target {
      return false
    } else if sum == target {
      let min = nums[which..which+i].iter().min();
      let max = nums[which..which+i].iter().max();

      println!("found {} with min {}, max {} and sum {}", target, min.unwrap(), max.unwrap(), min.unwrap()+max.unwrap() );
      return true;
    }
  }
  false
}
pub fn solve_part2(nums: &Vec<i64>, answer: i64) {
  // let answer = 105950735;
  for i in 0..nums.len() {
    if find(nums,i,answer) {
      break;
    }
  }
}