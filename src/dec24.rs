#[cfg(test)]
mod tests {

  #[test]
  pub fn dec24_prod() {
    let value = super::solve_part1("./inputs/dec24.txt".to_string());
    println!("Day 24 part 1 {}", value);

  }
  #[test]
  pub fn dec24_test() {
    assert!((0,0)==super::get_coordinates("nwwswee".to_string()));
    assert!(10==super::solve_part1("./inputs/dec24-test.txt".to_string()));
  }
}

use std::collections::HashMap;
use std::fs;

fn get_coordinates(direction : String) -> (i32,i32) {
  let start = (0,0);
  let mut c = start;
  let mut remaining_dir = direction.clone();
  let directions = vec!["ne", "nw", "sw", "se", "e", "w"];
  let dir_offset = vec![(-1,1), (-1,-1), (1,-1), (1,1), (0,2), (0,-2)];

  while remaining_dir.len() > 0 {
    for (index,d) in directions.iter().enumerate() {
      if remaining_dir.starts_with(d) {
        let offset = dir_offset[index];
        c.0 = c.0 + offset.0;
        c.1 = c.1 + offset.1;
        remaining_dir = remaining_dir[d.len()..].to_string();
      }
    }
  }
  return c;
}
pub struct Tile {
  is_black : bool,
}

pub fn solve_part1(filename : String) -> u64 {
  let mut retval = 0;
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();

  let mut tiles = HashMap::<(i32,i32),Tile>::new();
  for line in lines {
    let coordinates = get_coordinates(line);
    if let Some(t) = tiles.get_mut(&coordinates) {
      if t.is_black { t.is_black = false; } else { t.is_black = true; }
    } else {
      tiles.insert(coordinates,Tile { is_black : true });
    }
  }
  retval = tiles.values().fold(0,|acc,n| if n.is_black { acc + 1 } else { acc } );
  retval
}