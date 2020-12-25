#[cfg(test)]
mod tests {

  #[test]
  pub fn dec25_prod() {
    let value = super::solve_part1(15733400,6408062);
    println!("Day 25 part 1 {}", value);

  }
  #[test]
  pub fn dec25_test() {
    
    let val : u32= 7;
    let i = 11;
    assert!(super::find_loop_size_from_key(val.pow(i) % super::SECRET )==i);

    assert!(super::find_loop_size_from_key(49)==2);
    assert!(14897079==super::solve_part1(5764801,17807724));
    println!("another way {:8}", super::another_way(49));
    println!("another way {:8}", super::another_way(5764801));
    println!("another way {:8}", super::another_way(17807724));
  }
}

use std::collections::HashMap;
const SECRET : u32 = 20201227;

fn mod_inv(a: i64, module: i64) -> i64 {
  let mut mn = (module, a);
  let mut xy = (0, 1);
 
  while mn.1 != 0 {
    xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
    mn = (mn.1, mn.0 % mn.1);
  }
 
  while xy.0 < 0 {
    xy.0 += module;
  }
  xy.0
}
use std::convert::TryFrom;

fn another_way(key : u32) -> u32 {
  let mi =  mod_inv(7 as i64, SECRET as i64);

  let mut x : i64 = key as i64 * mi % SECRET as i64;
  let mut loop_count = 2;
  loop {
    // println!("{} {}={}*{} mod {}", i, x, key, mi, SECRET);
    x = x * mi % SECRET as i64;
    if x == 1 {
      break;
    }
    loop_count = loop_count + 1;
  }

  return loop_count; // u32::try_from(x).unwrap();
}

fn find_loop_size_from_key(key : u32) -> u32 {
  let mut loop_size = 1;

  loop {
    println!("{:8}", (0..loop_size).fold(1, |acc,_n| (acc * 7) % SECRET));
    if (0..loop_size).fold(1, |acc,_n| (acc * 7) % SECRET) == key {
      println!("{:8}", (0..loop_size+1).fold(1, |acc,_n| (acc * 7) % SECRET));
      return loop_size;
    }
    loop_size = loop_size + 1;
  }
}

pub fn solve_part1(key1 : u32, key2 : u32) -> u128 {

  let loop_size1 = another_way(key1);
  let loop_size2 = another_way(key2);
  let public_key1 = (0..loop_size1).fold(1, |acc,_n| (acc * 7) % SECRET);
  assert!(public_key1 == key1);
  let encryption_key1 = (0..loop_size2).fold(1, |acc,_n| (acc * key1 as u128) % SECRET as u128);

  let public_key2 = (0..loop_size2).fold(1, |acc,_n| (acc * 7) % SECRET);
  assert!(public_key2 == key2);
  let encryption_key2 = (0..loop_size1).fold(1, |acc,_n| (acc * key2 as u128) % SECRET as u128);
  assert!(encryption_key1 == encryption_key2);
  return encryption_key1;
}


