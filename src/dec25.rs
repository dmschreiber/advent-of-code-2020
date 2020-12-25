#[cfg(test)]
mod tests {

  #[test]
  pub fn dec25_prod() {
    let value = super::solve_part1(15733400,6408062);
    println!("Day 25 part 1 {}", value);

  }
  #[test]
  pub fn dec25_test() {
    assert!(14897079==super::solve_part1(5764801,17807724));

  }
}

use std::collections::HashMap;
const SECRET : u32 = 20201227;

fn find_loop_size_from_key(key : u32) -> u32 {
  let mut loop_size = 1;
  loop {
    println!("Test {} result is {}", loop_size, (0..loop_size).fold(1, |acc,n| (acc * 7) % SECRET));
    if (0..loop_size).fold(1, |acc,n| (acc * 7) % SECRET) == key {
      return loop_size;
    }
    loop_size = loop_size + 1;
  }
}

pub fn solve_part1(key1 : u32, key2 : u32) -> u128 {
  let retval = 0;

  let val = 1;
  let loop_size1 = find_loop_size_from_key(key1);
  let loop_size2 = find_loop_size_from_key(key2);
  let public_key1 = (0..loop_size1).fold(1, |acc,n| (acc * 7) % SECRET);
  assert!(public_key1 == key1);
  let encryption_key1 = (0..loop_size2).fold(1, |acc,n| (acc * key1 as u128) % SECRET as u128);

  let public_key2 = (0..loop_size2).fold(1, |acc,n| (acc * 7) % SECRET);
  assert!(public_key2 == key2);
  let encryption_key2 = (0..loop_size1).fold(1, |acc,n| (acc * key2 as u128) % SECRET as u128);
  assert!(encryption_key1 == encryption_key2);
  return encryption_key1;
}


