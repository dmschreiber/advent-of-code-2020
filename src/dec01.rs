use std::fs;

pub fn read_input(filename: String) -> Vec<i32> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");
  let nums : Vec<i32> = contents.lines().map(|line| line.parse::<i32>().unwrap()).collect();
  println!("{:?}", nums);
  nums
}

pub fn solve_for_one(nums: Vec<i32>, num: i32) -> bool {
  // guess the middle and adjust until we find it's complement or fail

}

pub fn solve(nums: Vec<i32>) {
  // sort
  // loop through each numbrr
  // call solve for one

}