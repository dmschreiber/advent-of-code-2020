use std::time::{Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

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
        cached_permutations.entry(*n).or_insert(tmp);
        tmp
      }
    } else {
      0
    }
    ).sum::<i64>();
  
}

#[allow(dead_code)]
fn split_chunk_thread(nums: &Vec<i32>, my_rating: i32) {
  let cached_permutations = Arc::new(Mutex::new(<HashMap<i32,i64>>::new()));
  let chunks = 3;
  let chunk = my_rating/5 as i32;
  
  let mut handles = Vec::with_capacity(chunks);
  for which  in 1..=chunks as i32 {
      let cached_permutations = Arc::clone(&cached_permutations);
      let new_nums = nums.clone();

      handles.push(thread::spawn( move || {
        let start = Instant::now();
        // The shared state can only be accessed once the lock is held.
          // Our non-atomic increment is safe because we're the only thread
          // which can access the shared state when the lock is held.
          //
          // We unwrap() the return value to assert that we are not expecting
          // threads to ever fail while holding the lock.
          let nums = new_nums.clone();
          let mut cached_permutations2 = <HashMap<i32,i64>>::new();
          check_permutations(&nums, my_rating-which*chunk, my_rating, &mut cached_permutations2);

          let mut cached_permutations = cached_permutations.lock().unwrap();
          cached_permutations.extend(&cached_permutations2); 
          println!("thread {} cached permutations is {} in {:?}", which, cached_permutations.keys().len(), start.elapsed());
          
      }));
  }

  // Wait for other threads to finish.
  for handle in handles {
    handle.join().unwrap();
  }


  let cached_permutations = Arc::clone(&cached_permutations);
  let _cached_permutations = cached_permutations.lock().unwrap();
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
  println!("Day 10 part 1 one joltage={}, three joltage={}, product {} in {:?}", one_joltage, three_joltage, one_joltage*three_joltage, start.elapsed());

  // part 2

  let start = Instant::now();
  let mut cached_permutations =  <HashMap<i32,i64>>::new();

  let num_arrangements = check_permutations(&nums, 1, my_rating, &mut cached_permutations);

  println!("Day 10 part 2 num arrangements {} in {:?}", num_arrangements, start.elapsed());
}