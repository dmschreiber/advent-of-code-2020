#[cfg(test)]
mod tests {

  #[test]
  pub fn dec24_prod() {
    let value = super::solve_part1("./inputs/dec24.txt".to_string());
    assert!(479==value);
    println!("Day 24 part 1 {}", value);
    let value = super::solve_part2("./inputs/dec24.txt".to_string(),100);
    println!("Day 24 part 2 {}", value);

  }
  #[test]
  pub fn dec24_test() {
    assert!((0,0)==super::get_coordinates("nwwswee".to_string()));
    assert!(10==super::solve_part1("./inputs/dec24-test.txt".to_string()));
    assert!(2208==super::solve_part2("./inputs/dec24-test.txt".to_string(),100));
  }
}

use std::collections::HashMap;
use std::fs;
const DIRECTIONS: [&'static str; 6] = ["ne", "nw", "sw", "se", "e", "w"];
const DIR_OFFSET : [(i32,i32); 6] = [(-1,1), (-1,-1), (1,-1), (1,1), (0,2), (0,-2)];

fn get_coordinates(direction : String) -> (i32,i32) {
  let start = (0,0);
  let mut c = start;
  let mut remaining_dir = direction.clone();
  // let directions = vec!["ne", "nw", "sw", "se", "e", "w"];
  // let dir_offset = vec![(-1,1), (-1,-1), (1,-1), (1,1), (0,2), (0,-2)];

  while remaining_dir.len() > 0 {
    for (index,d) in DIRECTIONS.iter().enumerate() {
      if remaining_dir.starts_with(d) {
        let offset = DIR_OFFSET[index];
        c.0 = c.0 + offset.0;
        c.1 = c.1 + offset.1;
        remaining_dir = remaining_dir[d.len()..].to_string();
      }
    }
  }
  return c;
}
#[derive(Debug,Copy,Clone)]
pub struct Tile {
  row : i32,
  col : i32,
  is_black : bool,
}

pub fn solve_part1(filename : String) -> u64 {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();

  let mut tiles = HashMap::<(i32,i32),Tile>::new();
  for line in lines {
    let coordinates = get_coordinates(line);
    if let Some(t) = tiles.get_mut(&coordinates) {
      if t.is_black { t.is_black = false; } else { t.is_black = true; }
    } else {
      tiles.insert(coordinates,Tile { is_black : true, row : coordinates.0, col : coordinates.1 });
    }
  }
  let retval = tiles.values().fold(0,|acc,n| if n.is_black { acc + 1 } else { acc } );
  retval
}

fn print_tiles(tiles : &HashMap<(i32,i32),Tile>) {
  let max_col = tiles.keys().map(|(_a,b)| *b ).max().unwrap();
  let max_row = tiles.keys().map(|(a,_b)| *a ).max().unwrap();
  let min_col = tiles.keys().map(|(_a,b)| *b ).min().unwrap();
  let min_row = tiles.keys().map(|(a,_b)| *a ).min().unwrap();

  // print!("  ");
  // for c in min_col..=max_col {
  //   if (c>=0) {
  //     print!("{:1}", c);
  //   } else {
  //     print!(" ");
  //   }
  // }
  println!();
  for r in min_row..=max_row {
    print!("{:2}", r);
    for c in min_col..=max_col {
      if let Some(t) = tiles.get(&(r,c)) {
        if t.is_black {
          print!("O");
        } else {
          print!(" ");
        }
      }
      else {
        print!(" ");
      }
    }
    println!();
  }
}

fn flip_tiles(tiles : &mut HashMap<(i32,i32),Tile>) {
  let old_tiles = tiles.clone();
  // print_tiles(&tiles);

  let max_col = tiles.keys().map(|(_a,b)| *b ).max().unwrap();
  let max_row = tiles.keys().map(|(a,_b)| *a ).max().unwrap();
  let min_col = tiles.keys().map(|(_a,b)| *b ).min().unwrap();
  let min_row = tiles.keys().map(|(a,_b)| *a ).min().unwrap();

  for r in min_row-1..=max_row+1 {
    for c in min_col-2..=max_col+2 {
      let mut neighbor_count = 0;
      for d in DIR_OFFSET.iter() {
        let new_r = r + d.0;
        let new_c = c + d.1;
  
        if let Some(n_t) = old_tiles.get(&(new_r,new_c)) {
          if n_t.is_black { neighbor_count += 1; }
        }
      }
      //     Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
      // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
      let was_black;
      if let Some(t) = old_tiles.get(&(r,c)) {
        was_black = t.is_black;
      } else {
        was_black = false;
      }
      if was_black && (neighbor_count == 0 || neighbor_count > 2) {
        tiles.get_mut(&(r,c)).unwrap().is_black = false;
      } 
      if !was_black && neighbor_count == 2 {
        if let Some(t) = tiles.get_mut(&(r,c)) {
          t.is_black = true;
        } else {
          tiles.insert((r,c), Tile{ is_black : true, row : r, col : c });
        }
      } 

    }
  }

  for t in old_tiles.values() {
    let mut neighbor_count = 0;
    for d in DIR_OFFSET.iter() {
      let new_r = t.row + d.0;
      let new_c = t.col + d.1;

      if let Some(n_t) = old_tiles.get(&(new_r,new_c)) {
        if n_t.is_black { neighbor_count = neighbor_count + 1; }
      }
    }
  }

}

pub fn solve_part2(filename : String, days : u32) -> u64 {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();

  let mut tiles = HashMap::<(i32,i32),Tile>::new();
  for line in lines {
    let coordinates = get_coordinates(line);
    if let Some(t) = tiles.get_mut(&coordinates) {
      if t.is_black { t.is_black = false; } else { t.is_black = true; }
    } else {
      tiles.insert(coordinates,Tile { is_black : true , row : coordinates.0, col : coordinates.1 });
    }
  }

  let mut retval = 0;
  for day in 0..days {
    flip_tiles(&mut tiles);
    retval = tiles.values().fold(0,|acc,n| if n.is_black { acc + 1 } else { acc } );
    if day == days-1 {
      println!("Day {} - {} tiles", day+1, retval);
    }
  }
  print_tiles(&tiles);

  retval
}