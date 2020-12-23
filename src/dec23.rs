#[cfg(test)]
mod tests {

  #[test]
  pub fn dec23_prod() {
    println!("Day 23 part 1 {}", super::solve_part1("364289715".to_string(),100));
    assert!(98645732==super::solve_part1("364289715".to_string(),100));

  }
  #[test]
  pub fn dec23_test() {
    assert!(25467389==super::get_return_value(vec![3,8,9,1,2,5,4,6,7]));
    assert!(6==super::get_destination(&vec![3,8,9,1,2,5,4,6,7],5));
    assert!(1==super::get_destination(&vec![3,8,9,1,2,5,4,6,7],9));
    assert!(0==super::get_destination(&vec![3,8,9,1,2,5,4,6,7],4));
    assert!(1==super::get_destination(&vec![8,9,5,4,6,7],4));

    // assert!(25467389==super::solve_part1("389125467".to_string(),100));

    assert!(92658374==super::solve_part1("389125467".to_string(),10));
    assert!(67384529==super::solve_part1("389125467".to_string(),100));
      
    // assert!(149245887792==super::solve_part2("389125467".to_string(),1000));
    // assert!(149245887792==super::solve_part2("389125467".to_string(),10000000));
    // assert!(149245887792==super::solve_part2("389125467".to_string(),1000000));
    // assert!(149245887792==super::solve_part2("389125467".to_string(),10000000)); // actual data
  }  
}

use std::fs;
use std::iter;
use regex::Regex;
use std::collections::HashMap;
use std::time::{Instant};
// use sha1::{Sha256, Digest};

fn adjust_cups(cup_vec : &Vec<u32>, cup : u32) -> Vec<u32> {
  let c = find(&cup_vec,cup);
  let mut v = cup_vec[c..].to_vec();
  v.append(&mut cup_vec[..c].to_vec());
  return v;
}
pub fn get_return_value(cup_vec : Vec<u32>) -> u32 {
  // let cup_vec = cups.as_bytes().iter().map(|c| (*c - b'0') as u32).collect::<Vec<u32>>();
  let one_cup = cup_vec.iter().position(|&r| r == 1).unwrap();
  let mut first_cup_vec = cup_vec[one_cup+1..].to_vec();
  let mut second_cup_vec = cup_vec[..one_cup].to_vec();
  first_cup_vec.append(&mut second_cup_vec);
  // println!("return value{:?}", first_cup_vec);
  return first_cup_vec.iter().fold(0, |acc, n| 10 * acc + *n);
}
fn get_destination(cup_vec : &Vec<u32>, target : u32) -> usize {
  let mut running_target = target - 1;
  let mut dest = None;
  let min = cup_vec.iter().map(|n| *n).min().unwrap();
  let max = cup_vec.iter().map(|n| *n).max().unwrap();

   while dest == None {
    //  println!("min {}, max {}, dest {:?}, running_target {}", min, max, dest, running_target);
    dest = cup_vec.iter().position(|&r| r == running_target);
    if running_target < min {
      running_target = max;
    } else {
      running_target = running_target - 1;
    }
  }
  // println!("RETURNING min {}, max {}, dest {:?}, running_target {}", min, max, dest, running_target);

  dest.unwrap()
}


fn get_destination_with_ignore(cup_vec : &Vec<u32>, target : u32, ignore : &Vec<u32>) -> usize {
  let mut running_target = target - 1;
  let mut dest = None;
  let min = cup_vec.iter().map(|n| *n).min().unwrap();
  let max = cup_vec.iter().map(|n| *n).max().unwrap();

   while dest == None {
    //  println!("min {}, max {}, dest {:?}, running_target {}", min, max, dest, running_target);
    if !ignore.contains(&running_target) {
      dest = cup_vec.iter().position(|&r| r == running_target);
    }
    if running_target < min {
      running_target = max;
    } else {
      running_target = running_target - 1;
    }
  }
  // println!("RETURNING min {}, max {}, dest {:?}, running_target {}", min, max, dest, running_target);

  dest.unwrap()
}

fn find(cup_vec : &Vec<u32>, target : u32) -> usize {
  return cup_vec.iter().position(|&r| r == target).unwrap();

}
pub fn solve_part1(cups : String, moves : u32) -> u32 {
  let mut cup_vec = cups.as_bytes().iter().map(|c| (*c - b'0') as u32).collect::<Vec<u32>>();

  let mut current = cup_vec[0];

  for i in 0..moves {
    let start = Instant::now();

    cup_vec = adjust_cups(&cup_vec, current);
    // let subset = cup_vec[1..4].to_vec().clone();

    let pick_up = &cup_vec[1..4].to_vec().clone();    
    // let removed : Vec<_> = cup_vec.splice(1..4, vec![]).collect();

    let d = get_destination_with_ignore(&cup_vec,current,&pick_up);
    println!("destination is {}", cup_vec[d]); // add to ignore pick_up
    // cup_vec[1..d].iter().map()
    for pos in 1..=d-3 {
      cup_vec.get_mut(pos);
      cup_vec[pos] = cup_vec[pos+3];
    }
    cup_vec[d-2] = pick_up[0];
    cup_vec[d-1] = pick_up[1];
    cup_vec[d] = pick_up[2];

    // cup_vec.splice(d+1..d+1, removed.iter().cloned());

    let current_index = find(&cup_vec, current);
    if current_index == cup_vec.len() - 1 {
      current = cup_vec[0];
    } else {
      current = cup_vec[current_index+1];
    }
    println!("After move {} - {:?} current ({}) {:?}", i, cup_vec, current, start.elapsed());


  }
  cup_vec = adjust_cups(&cup_vec, 1);
  return get_return_value(cup_vec);
}

// PART 2 *****
// 
// 
pub const MAX_CUPS : u32 = 100000;

pub fn solve_part2(cups : String, moves : u32) -> u64 {
  let mut cup_vec = Vec::with_capacity(MAX_CUPS as usize);
  cup_vec.append(&mut cups.as_bytes().iter().map(|c| (*c - b'0') as u32).collect::<Vec<u32>>());
  let max = cup_vec.iter().map(|n| *n).max().unwrap();
  let mut cup_rest = (max+1..MAX_CUPS+1).collect::<Vec<u32>>();
  cup_vec.append(&mut cup_rest);
  assert!(cup_vec.len()==MAX_CUPS as usize);

  let mut current = cup_vec[0];
  let first_pair = (cup_vec[1], cup_vec[2]);
  for i in 0..moves {
    // current = play_round(&mut cup_vec, current);
    let start = Instant::now();

    cup_vec = adjust_cups(&cup_vec, current);
    // println!("Before move {} - {:?}...{:?}", i, &cup_vec[..15], &cup_vec[cup_vec.len()-15..]);
    if  (i+1) % 100 == 0 {
      // println!("Adjust cups {:?} <-- desire <100ns", start.elapsed());
    }

    if true {
      let removed : Vec<_> = cup_vec.splice(1..4, vec![]).collect();
      let d = get_destination(&cup_vec,current);
      // println!("destination is {}", cup_vec[d]); // add to ignore pick_up
      cup_vec.splice(d+1..d+1, removed.iter().cloned());

    } else {
      let pick_up = &cup_vec[1..4].to_vec().clone();    

      let d = get_destination_with_ignore(&cup_vec,current,&pick_up);

      for pos in 1..=d-3 {
        cup_vec.get_mut(pos);
        cup_vec[pos] = cup_vec[pos+3];
      }
      cup_vec[d-2] = pick_up[0];
      cup_vec[d-1] = pick_up[1];
      cup_vec[d] = pick_up[2];
    }

    if  (i+1) % 100 == 0 {
      // println!("Adjust splice {:?} <-- desire <100ns", start.elapsed());
    }

    let current_index = find(&cup_vec, current);
    if current_index == cup_vec.len() - 1 {
      current = cup_vec[0];
    } else {
      current = cup_vec[current_index+1];
    }
    // println!("After move {} - {:?} current ({})", i, cup_vec, current);
    if first_pair ==  (cup_vec[1], cup_vec[2]) {
      println!("COMPARE {:?} and {:?} AT {:?}", i+1, first_pair, (cup_vec[1], cup_vec[2]));
    }

    if  (i+1) % 100 == 0 {
      // println!("After move {} - {:?}, {} - {:?} <-- desire <100ns", i+1, cup_vec[1], cup_vec[2], start.elapsed());
    }
  }
  cup_vec = adjust_cups(&cup_vec, 1);
  return cup_vec[1] as u64 *cup_vec[2] as u64;
}