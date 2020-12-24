#[cfg(test)]
mod tests {
  use std::collections::LinkedList;

  #[test]
  pub fn dec23_prod() {
    let value = super::solve_part1("364289715".to_string(),100,false);
    println!("Day 23 part 1 {}", value);
    assert!(98645732==value);

  }
  #[test]
  pub fn dec23_test() {
    assert!(25467389==super::get_return_value(vec![3,8,9,1,2,5,4,6,7]));
    assert!(6==super::get_destination(&vec![3,8,9,1,2,5,4,6,7],5));
    assert!(1==super::get_destination(&vec![3,8,9,1,2,5,4,6,7],9));
    assert!(0==super::get_destination(&vec![3,8,9,1,2,5,4,6,7],4));
    assert!(1==super::get_destination(&vec![8,9,5,4,6,7],4));

    let mut l = LinkedList::<u32>::new();
    for n in vec![3,8,9,1,2,5,4,6,7] {
      l.push_back(n);
    }
    assert!(6==super::get_destination_list(&l, 5));
    assert!(1==super::get_destination_list(&l, 9));
    assert!(0==super::get_destination_list(&l, 4));

    let mut l2 = LinkedList::<u32>::new();
    for n in vec![8,9,5,4,6,7] {
      l2.push_back(n);
    }
    assert!(1==super::get_destination_list(&l2,4));

    // assert!(25467389==super::solve_part1("389125467".to_string(),100));

    assert!(92658374==super::solve_part1("389125467".to_string(),10,false));
    assert!(67384529==super::solve_part1("389125467".to_string(),100,false));
      
    // assert!(149245887792==super::solve_part2("389125467".to_string(),1000));
    // assert!(149245887792==super::solve_part2("389125467".to_string(),10000000));
    // assert!(149245887792==super::solve_part2("389125467".to_string(),1000000));
    assert!(149245887792==super::solve_part1("389125467".to_string(),10000000,true)); // actual data
  }  
}

use std::fs;
use std::iter;
use regex::Regex;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::time::{Instant};
// use sha1::{Sha256, Digest};

fn adjust_cups(cup_vec : &Vec<u32>, cup : u32) -> Vec<u32> {
  let c = find(&cup_vec,cup);
  let mut v = cup_vec[c..].to_vec();
  v.append(&mut cup_vec[..c].to_vec());
  return v;
}
pub fn get_return_value(cup_vec : Vec<u32>) -> u64 {
  // let cup_vec = cups.as_bytes().iter().map(|c| (*c - b'0') as u32).collect::<Vec<u32>>();
  let one_cup = cup_vec.iter().position(|&r| r == 1).unwrap();
  let mut first_cup_vec = cup_vec[one_cup+1..].to_vec();
  let mut second_cup_vec = cup_vec[..one_cup].to_vec();
  first_cup_vec.append(&mut second_cup_vec);
  // println!("return value{:?}", first_cup_vec);
  return first_cup_vec.iter().fold(0, |acc, n| 10 * acc + *n as u64);
}
fn get_destination_list(cup_list : &LinkedList<u32>, target : u32) -> usize {
  let mut running_target = target - 1;
  let mut dest = None;
  let min = cup_list.iter().map(|n| *n).min().unwrap();
  let max = cup_list.iter().map(|n| *n).max().unwrap();
  // println!("{:?} min {} max {}", cup_list.iter().map(|n| *n).collect::<Vec<u32>>(), min, max);

   while dest == None {
     if cup_list.contains(&running_target) {
      //  println!("running target {} in list", running_target);
      for (i,number) in cup_list.iter().enumerate() {
        if running_target == *number {
          dest = Some(i);
        }
      }
     } else {
      //  println!("running target {} not in list", running_target);
       dest = None;
     }
//    dest = cup_vec.iter().position(|&r| r == running_target);
    if running_target < min {
      running_target = max;
    } else {
      running_target = running_target - 1;
    }
    // println!("new running_target {}", running_target);
  }

  dest.unwrap()

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

fn adjust_cup_list(cup_list : &mut LinkedList<u32>, current : u32) {
  let mut position = 0;

  // println!("Adjust cup_list start ({}) - {:?}", current, cup_list.iter().map(|n| *n).collect::<Vec<u32>>());

  for (index,a) in cup_list.iter().enumerate() {
    if *a == current {
      position = index;
      break;
    }
  }
  if position != 0 {
    let split = cup_list.split_off(position);
    for n in split.iter().rev() {
      cup_list.push_front(*n);
    }
  }
  // println!("Adjust cup_list end ({}) {:?}",current, cup_list.iter().map(|n| *n).collect::<Vec<u32>>());

}

fn find_in_list(cup_list : &LinkedList<u32>, target : u32) -> usize {
  return cup_list.iter().position(|&r| r == target).unwrap();
}

fn find(cup_vec : &Vec<u32>, target : u32) -> usize {
  return cup_vec.iter().position(|&r| r == target).unwrap();

}

pub const MAX_CUPS : u32 = 1000000;

pub fn solve_part1(cups : String, moves : u32, part_two : bool) -> u64 {
  let mut cup_vec = cups.as_bytes().iter().map(|c| (*c - b'0') as u32).collect::<Vec<u32>>();
  if part_two {
    let max = cup_vec.iter().map(|n| *n).max().unwrap();
    let mut cup_rest = (max+1..MAX_CUPS+1).collect::<Vec<u32>>();
    cup_vec.append(&mut cup_rest);
    assert!(cup_vec.len()==MAX_CUPS as usize);
  }
  let mut cup_list = LinkedList::<u32>::new();

  for n in cup_vec {
    cup_list.push_back(n);
  }

  let mut current = cup_list.front().unwrap().clone();

  for i in 0..moves {
    let start = Instant::now();
    adjust_cup_list(&mut cup_list, current);
    if !part_two {
      println!("Begin move {}\n cups {:?} \n current ({}) {:?}", i+1, cup_list.iter().map(|n| *n).collect::<Vec<u32>>(), current, start.elapsed());
    } else {
      if i+1 % 1000 == 0 {
        println!("Move {}", i);
      }
    }

    // cup_vec = adjust_cups(&cup_vec, *current);
    
    let mut back_of_list = cup_list.split_off(4);
    back_of_list.push_front(cup_list.pop_front().unwrap()); // new list without three items

    let d = get_destination_list(&back_of_list, current); // index of destination
    if !part_two {
      let v = cup_list.iter().map(|n| *n).collect::<Vec<u32>>();
      let v2 = back_of_list.iter().map(|n| *n).collect::<Vec<u32>>();
      println!("pick up: {:?}", v);
      println!("Destination {}", v2[d]);
    }
    let mut new_back_of_list = back_of_list.split_off(d+1); 

    if !part_two {
      let v = new_back_of_list.iter().map(|n| *n).collect::<Vec<u32>>();
      let v2 = back_of_list.iter().map(|n| *n).collect::<Vec<u32>>();
      let v3 = cup_list.iter().map(|n| *n).collect::<Vec<u32>>();
      println!("assemble {:?} {:?} {:?}", v2, v3, v);
    }
    
    back_of_list.append(&mut cup_list); // append removed cups
    back_of_list.append(&mut new_back_of_list); // append previous back of list

    // back_of_list should now be:
    // cup_vec.splice(d+1..d+1, removed.iter().cloned());

    cup_list = back_of_list;
    if !part_two {
      let v = cup_list.iter().map(|n| *n).collect::<Vec<u32>>();
      println!("new list looks like: {:?}", v);
    }

    let current_index = find_in_list(&cup_list, current);
    if !part_two {
      println!("current_index {}", current_index);
    }

    // let current_index = find(&cup_vec, *current);
    if current_index == cup_list.len() - 1 {
      if !part_two {
        println!("current_index last in the list");
      }
      current = cup_list.front().unwrap().clone();
    } else {
      for (index,n) in cup_list.iter().enumerate() {
        if index == current_index + 1 {
            if !part_two {
              println!("current_index now {} with value {}", index, *n);
            }
            current = *n;
        }
      }
    }
    if part_two {
      if i+1 % 1000 == 0 {
        println!("Move {} in {:?}", i+1, start.elapsed());
      }
    }

  }
  // cup_vec = adjust_cups(&cup_vec, 1);
  adjust_cup_list(&mut cup_list, 1);
  let cup_vec = cup_list.iter().map(|n| *n).collect::<Vec<u32>>();
  if part_two { return cup_vec[1] as u64 *cup_vec[2] as u64; }
  else { return get_return_value(cup_vec) };
}

