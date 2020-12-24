#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  #[test]
  pub fn dec23_prod() {
    let value = super::solve_part1("364289715".to_string(),100,false);
    println!("Day 23 part 1 {}", value);
    assert!(98645732==value);
    let value = super::solve_part1("364289715".to_string(),10000000,true);
    println!("Day 23 part 2 {}", value);

  }
  #[test]
  pub fn dec23_test() {
    let mut cup_map = HashMap::<u32,super::Cup>::new();
    let v = vec![3,8,9,1,2,5,4,6,7];
    let mut last_n = 7 as u32;
    for n in v {
      cup_map.insert(last_n,super::Cup { label : last_n, right : n});
      last_n = n;
    }
    assert!(25467389==super::get_return_value_map(&cup_map));

    assert!(92658374==super::solve_part1("389125467".to_string(),10,false));
    assert!(67384529==super::solve_part1("389125467".to_string(),100,false));
      
    assert!(149245887792==super::solve_part1("389125467".to_string(),10000000,true)); // actual data
  }  
}

use std::collections::HashMap;

pub fn get_return_value_map(cup_map : &HashMap<u32,Cup>) -> u64 {
  let cup_one = cup_map.get(&1).unwrap();

  let mut next  = cup_one.right; 
  let mut acc : u64 = 0;
  for _i in 0..8 {
    if next == 1 {
      break;
    }
    acc = acc * 10 + next as u64;
    next = cup_map.get(&next).unwrap().right;
  }
  acc
}


pub const MAX_CUPS : u32 = 1000000;

#[derive(Debug,Clone)]
pub struct Cup {
  label : u32,
  right : u32,
}

fn pick_up_cups_map(cups : &mut HashMap<u32,Cup>, current : u32) -> Vec<Cup> {
  let mut v = vec![];

  let c1 = cups.get(&current).unwrap().right;
  let c2 = cups.get(&c1).unwrap().right;
  let c3 = cups.get(&c2).unwrap().right;
  let c3_o = cups.get(&c3).unwrap();

  cups.get_mut(&current).unwrap().right = c3_o.right;

  v.push(cups.get(&c1).unwrap().clone());
  v.push(cups.get(&c2).unwrap().clone());
  v.push(cups.get(&c3).unwrap().clone());

  cups.remove(&c1);
  cups.remove(&c2);
  cups.remove(&c3);

  v
}

fn deposit_cups(cups: &mut HashMap<u32,Cup>, dest_cup : u32, new_cups : Vec<Cup>) {
  let dest = cups.get_mut(&dest_cup).unwrap();
  let saved_right = dest.right;
  dest.right = new_cups[0].label;
  cups.insert(new_cups[0].label, new_cups[0].clone());
  cups.insert(new_cups[1].label, new_cups[1].clone());
  cups.insert(new_cups[2].label, Cup { label : new_cups[2].label, right : saved_right});
}

#[allow(dead_code)]
pub fn show_cups (cup_map : &HashMap<u32,Cup>, current : u32 ) -> String {
  let c = cup_map.get(&current).unwrap();
  let mut next = c.right;
  let mut retval;
  
  retval = format!("({})", c.label);
  
  let mut index = 0;
  while next != c.label && index < 15 {
    retval = format!("{} {}", retval, next);
    next = cup_map.get(&next).unwrap().right;
    index = index + 1;
  }
  retval
}

pub fn solve_part1(cups : String, moves : u32, part_two : bool) -> u64 {
  let mut cup_vec = cups.as_bytes().iter().map(|c| (*c - b'0') as u32).collect::<Vec<u32>>();
  if part_two {
    let max = cup_vec.iter().map(|n| *n).max().unwrap();
    let mut cup_rest = (max+1..MAX_CUPS+1).collect::<Vec<u32>>();
    cup_vec.append(&mut cup_rest);
    assert!(cup_vec.len()==MAX_CUPS as usize);
  }

  let mut cup_map = HashMap::new();

  let max_n = cup_vec.len();
  let mut last_n = cup_vec[cup_vec.len()-1] as u32;
  let mut current = cup_vec[0].clone();
  for n in cup_vec {
    // cup_list.push_back(n);
    cup_map.insert(last_n,Cup { label : last_n, right : n});
    last_n = n;
  }

  for _i in 0..moves {

    if !part_two {
      // println!("Begin move {}\n cups {:?} \n {:?}", i+1, show_cups(&cup_map,current), start.elapsed());
    } 
    

    let pick_up = pick_up_cups_map(&mut cup_map,current);
    if !part_two {
      // println!("Pick-up cups {:?} \n {:?}", pick_up, start.elapsed());
    } 

    let mut dest_cup = current - 1;
    while !cup_map.contains_key(&dest_cup) {
      if dest_cup == 0 { dest_cup = max_n as u32; }
      else {
        dest_cup = dest_cup - 1;
      }
    }
    if !part_two {
      // println!("Destination {} \n {:?}", dest_cup, start.elapsed());
    } 
    deposit_cups(&mut cup_map, dest_cup,pick_up);
    current = cup_map.get(&current).unwrap().right;
  }

  let current_o = cup_map.get(&1).unwrap();
  let current_r1 = cup_map.get(&current_o.right).unwrap();
  let current_r2 = cup_map.get(&current_r1.right).unwrap();

  if part_two { return current_r1.label as u64 * current_r2.label as u64; }
  else { return get_return_value_map(&cup_map) };
}

