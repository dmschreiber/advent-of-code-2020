#[cfg(test)]
mod tests {

  #[test]
  pub fn dec20_prod() {
    super::solve("./inputs/dec20.txt".to_string());

  }
  #[test]
  pub fn dec20_test() {
      super::solve("./inputs/dec20-test.txt".to_string());
  }  
}

use std::fs;
use regex::Regex;
use std::time::{Instant};

lazy_static! {
  static ref TILE_ID_REGEX: Regex = Regex::new(r"^Tile (\d+):$").unwrap();

}

#[derive(Debug,Clone,PartialEq)]
pub struct Tile {
  id : u32,
  lines : Vec<String>,
  sides : Vec<u32>,
  rev_sides : Vec<u32>,
}

impl Tile {
  fn all_rotations(&self) -> Vec<Vec<u32>> {
    let mut v = Vec::new();

    v.push(vec![self.sides[0],self.sides[1],self.sides[2],self.sides[3]]);
    v.push(vec![self.rev_sides[3],self.sides[0],self.rev_sides[1],self.sides[2]]);
    v.push(vec![self.rev_sides[2],self.rev_sides[3],self.rev_sides[0], self.rev_sides[1]]);
    v.push(vec![self.sides[1],self.rev_sides[2],self.sides[3], self.rev_sides[0]]);

    v.push(vec![self.sides[0],self.sides[3],self.sides[2],self.sides[1]]);
    v.push(vec![self.rev_sides[1],self.sides[0],self.rev_sides[3],self.sides[2]]);
    v.push(vec![self.rev_sides[2],self.rev_sides[1],self.rev_sides[0], self.rev_sides[3]]);
    v.push(vec![self.sides[3],self.rev_sides[2],self.sides[1], self.rev_sides[0]]);

    v.push(vec![self.sides[2],self.rev_sides[1],self.sides[0],self.rev_sides[3]]);
    v.push(vec![self.sides[3],self.sides[2],self.sides[1],self.sides[0]]);
    v.push(vec![self.rev_sides[0],self.sides[3],self.rev_sides[2], self.sides[1]]);
    v.push(vec![self.rev_sides[1],self.rev_sides[0],self.rev_sides[3], self.rev_sides[2]]);

    v
  }
}

pub fn make_tile(lines : Vec<String>) -> Tile {
  let mut left_string = "".to_string();
  let mut right_string = "".to_string();

  let mut tile_id= 0;
  let mut top : u32 = 0;
  let mut top_rev : u32 = 0;
  let bottom : u32;
  let bottom_rev : u32;
  let left : u32;
  let left_rev : u32;
  let right : u32;
  let right_rev : u32;

  for (index,line) in lines.iter().enumerate() {
    if line.len() == 0 { break; }

    if let Some(args) = TILE_ID_REGEX.captures(line) {
      tile_id = args[1].parse::<u32>().unwrap();
    } else {
      if index == 1 {
        top = u32::from_str_radix(&line.replace("#","1").replace(".","0"),2).unwrap();
        top_rev = u32::from_str_radix(&line.replace("#","1").replace(".","0").chars().rev().collect::<String>(),2).unwrap();

        left_string = line[..1].to_string();
        right_string = line[9..=9].to_string();
      } else if index == 10 {
        bottom = u32::from_str_radix(&line.replace("#","1").replace(".","0"),2).unwrap();
        bottom_rev = u32::from_str_radix(&line.replace("#","1").replace(".","0").chars().rev().collect::<String>(),2).unwrap();

        left_string = format!("{}{}", left_string, &line[..1].to_string());
        right_string = format!("{}{}", right_string.clone(), &line[9..=9].to_string());

        left = u32::from_str_radix(&left_string.replace("#","1").replace(".","0"),2).unwrap();
        left_rev = u32::from_str_radix(&left_string.replace("#","1").replace(".","0").chars().rev().collect::<String>(),2).unwrap();

        right = u32::from_str_radix(&right_string.replace("#","1").replace(".","0"),2).unwrap();
        right_rev = u32::from_str_radix(&right_string.replace("#","1").replace(".","0").chars().rev().collect::<String>(),2).unwrap();

        return Tile{ id : tile_id, 
            lines: lines[1..].to_vec(),
            sides : vec![top, right, bottom, left] , 
            rev_sides : vec![top_rev, right_rev, bottom_rev, left_rev] };

      } else {
        left_string = format!("{}{}", left_string, &line[..1].to_string());
        right_string = format!("{}{}", right_string.clone(), &line[9..=9].to_string());

      }
    }
  }
  panic!("never returned a tile");
}

pub fn solve(filename : String) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  
    let tiles : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
    
    let mut things = Vec::new();

    for tile in tiles {
      // println!("{}", tile);
      let lines : Vec<String> = tile.lines().map(|s| (&*s).to_string()).collect();
      things.push(make_tile(lines));
    }

    // println!("{:?}", things);
    let start = Instant::now();

    let mut corners = std::collections::HashMap::<u32,Tile>::new();
    for t in &things {
      println!("Trying {}", t.id);
      for (r_index,rotation) in t.all_rotations().iter().enumerate() {
        let mut unique_sides = 0;
        for (index,s) in rotation.iter().enumerate() {
          let c = count_matches(&things,t.id,*s);
          // println!("Tile {} rotation {:?} {} side index {} with value {} is {} others", t.id, rotation, r_index, index, *s, c);
          if c == 0 {
            unique_sides += 1;
          }
        }
        if unique_sides == 2 {
          // println!("{:?} rotation {:?} has {} unique sides", t, rotation, unique_sides);
          corners.insert(t.id, t.clone());
        }
      }
    }

    let mut retval : u64 = 1;
    for c in corners.values() {
      println!("{}", c.id);
      retval = retval * c.id as u64;
    }
    println!("{}={} in {:?}", corners.keys().map(|id| format!("{}",id) ).collect::<Vec<String>>().join("x"), retval, start.elapsed());
  }

pub fn count_matches(things : &Vec<Tile>, skip_id : u32, side : u32) -> u32 {
  let mut retval = 0;

  for t in things {
    if t.id != skip_id {
      for rotation_sides in t.all_rotations() {
        for s in rotation_sides {
          if s == side { retval += 1; }
        }
      }
    }
  }
  
  retval
}