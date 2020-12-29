#[cfg(test)]
mod tests {

  #[test]
  pub fn dec20_prod() {
    assert!(30425930368573==super::solve_part1("./inputs/dec20.txt".to_string()));
    assert!(2453==super::solve_part2("./inputs/dec20.txt".to_string()));

  }
  #[test]
  pub fn dec20_test() {
      assert!(20899048083289==super::solve_part1("./inputs/dec20-test.txt".to_string()));
      super::solve_part2("./inputs/dec20-test.txt".to_string());
  }  
}

use std::fs;
use regex::Regex;
use std::time::{Instant};
use std::collections::HashMap;
use std::convert::TryInto;
use colored::*;

lazy_static! {
  static ref TILE_ID_REGEX: Regex = Regex::new(r"^Tile (\d+):$").unwrap();

}

#[derive(Debug,Clone,PartialEq)]
pub struct Tile {
  id : u32,
  lines : Vec<String>,
  sides : Vec<u32>,
  rev_sides : Vec<u32>,
  rotation : Option<usize>,
}

impl Tile {
  fn reverse (&self, side : u32) -> u32 {
    if self.sides.contains(&side) {
      let index = self.sides.iter().position(|k| *k == side).unwrap();
      return self.rev_sides[index];
    } else {
      let index = self.rev_sides.iter().position(|k| *k == side).unwrap();
      return self.sides[index];
    }
  }

  #[allow(dead_code)]
  fn print_with_border(&self, rotated : bool) {
    for row in 0..10 {
      for col in 0..10 {
        if rotated {
          print!("{}", self.get_row_col_with_border(row, col));
        } else {
          print!("{}", self.lines[row].as_bytes()[col] as char);
        }
      }
      println!();
    }
  }

  fn all_rotations(&self) -> Vec<Vec<u32>> {
    let mut v = Vec::new();

    let mut basic = vec![self.sides[0],self.sides[1],self.sides[2],self.sides[3]];
    // 0 degree; 90 degree; 180 degree; 270 degree rotation
    v.push(basic.clone());

    for _i in 0..3 {
      basic = vec![self.reverse(basic[3]),basic[0],self.reverse(basic[1]), basic[2]];
      v.push(basic.clone());
    }

    // flip on vertical axis then 0 degree; 90 degree; 180 degree; 270 degree rotation
    basic = vec![self.rev_sides[0],self.sides[3],self.rev_sides[2],self.sides[1]];
    v.push(basic.clone());
    for _i in 0..3 {
      basic = vec![self.reverse(basic[3]),basic[0],self.reverse(basic[1]), basic[2]];
      v.push(basic.clone());
    }

    // flip on horizontal axis then 0 degree; 90 degree; 180 degree; 270 degree rotation
    basic = vec![self.sides[2],self.rev_sides[1],self.sides[0],self.rev_sides[3]];
    v.push(basic.clone());
    for _i in 0..3 {
      basic = vec![self.reverse(basic[3]),basic[0],self.reverse(basic[1]), basic[2]];
      v.push(basic.clone());
    }

    v
  }
  fn get_row_col_with_border(&self, row : usize, col : usize) -> char {
    let mut rotate_row = row;
    let mut rotate_col = col;

    if let Some(r) = self.rotation  {
      let (r,c) = rotate_coordinates(row, col, 10, r);
      rotate_row = r;
      rotate_col = c;

    }

    let retval = self.lines[rotate_row].as_bytes()[rotate_col] as char;
    return retval;

  }

  fn get_row_col(&self, row : usize, col : usize) -> char {

    let mut rotate_row = row;
    let mut rotate_col = col;

    if let Some(r) = self.rotation  {
      let (r,c) = rotate_coordinates(row, col, 8, r);
      rotate_row = r;
      rotate_col = c;

    }

    let retval = self.lines[rotate_row+1].as_bytes()[rotate_col+1] as char;
    return retval;
  }
  fn find_rotation(&self, sides : Vec<Option<usize>>) -> Option<usize> {
    
    for (i,r) in self.all_rotations().iter().enumerate() {
      let mut side_match = true;
      for (side_index,s) in sides.iter().enumerate() {
        if let Some(side_val) = s {
          if *side_val != r[side_index] as usize {
            side_match = false;
          }
        }
      }
      if side_match {
        return Some(i);
      }
    }
    return None;
  }

}

fn rotate_coordinates (row : usize, col : usize, size : usize, rotation : usize) -> (usize, usize) {
  let mut rotate_row = row;
  let mut rotate_col = col;
  let r = rotation;

  if r == 0 {
  // no nothing
  } else if r >= 1 && r <= 3 {
    for _i in 0..r {
      let save_row = rotate_row;
      rotate_row = size-rotate_col-1;
      rotate_col = save_row;
    }
  } else if r >= 4 && r <= 7 {
    rotate_col = size - rotate_col - 1;
    for _i in 0..r-4 {
      let save_col = rotate_col;
      rotate_col = size-rotate_row-1;
      rotate_row = save_col;
    }
  } else if r >= 8 && r <= 11 {
    rotate_row = size - rotate_row - 1;
    for _i in 0..r-8 {
      let save_col = rotate_col;
      rotate_col = size-rotate_row-1;
      rotate_row = save_col;
    }
  }
  
  return (rotate_row, rotate_col);

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

        return Tile{ id : tile_id, rotation : None,
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


pub fn solve_part1(filename : String) -> u64 {
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
    let mut borders = std::collections::HashMap::<u32,Tile>::new();
    let mut inner = std::collections::HashMap::<u32,Tile>::new();
    for t in &things {
      // println!("Trying {}", t.id);
      for (_r_index,rotation) in t.all_rotations().iter().enumerate() {
        let mut unique_sides = 0;
        for (_index,s) in rotation.iter().enumerate() {
          let c = count_matches(&things,t.id,*s);
          // println!("Tile {} rotation {:?} {} side index {} with value {} is {} others", t.id, rotation, _r_index, _index, *s, c);
          if c == 0 {
            unique_sides += 1;
          }
        }
        if unique_sides == 2 {
          // println!("{:?} rotation {:?} has {} unique sides", t, rotation, unique_sides);
          corners.insert(t.id, t.clone());
        } else if unique_sides == 1 {
          borders.insert(t.id, t.clone());
        } else {
          inner.insert(t.id, t.clone());
        }
      }
    }

    let mut retval : u64 = 1;
    for c in corners.values() {
      // println!("{}", c.id );
      retval = retval * c.id as u64;
    }
    println!("{}={} in {:?}", corners.keys().map(|id| format!("{}",id) ).collect::<Vec<String>>().join("x"), retval, start.elapsed());
    retval
}

pub fn build_border(corners : &Vec<Tile>, borders : &Vec<Tile>, width : u32) -> HashMap<(u32,u32),Tile> {
  let mut retval = std::collections::HashMap::<(u32,u32),Tile>::new();
  let mut borders_vec : Vec<Tile> = borders.clone(); // borders.values().cloned().collect();
  let mut corners_vec : Vec<Tile> = corners.clone(); // values().cloned().collect();

  // println!("borders {:?}", borders_vec);
  // SPECIAL CASE for 2x2; all corners no borders
  if width == 2 {
    let mut t = corners_vec[0].clone();
    let index = corners_vec.iter().position(|x| *x == t).unwrap();
    corners_vec.remove(index);

    for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
      let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&corners_vec, t.id, *s)).collect();
      if v == vec![0,1,1,0] {
        t.rotation = Some(my_rotation);
        retval.insert((0,0),t.clone());

        let t1 = get_match(&corners_vec, t.id, rotation_sides[1]).unwrap().clone();
        let index = corners_vec.iter().position(|x| *x == t1).unwrap();
        corners_vec.remove(index);
        retval.insert((0,1),t1);

        let t2 = get_match(&corners_vec, t.id, rotation_sides[2]).unwrap().clone();
        let index = corners_vec.iter().position(|x| *x == t2).unwrap();
        corners_vec.remove(index);
        retval.insert((1,0),t2);

        break;
      }
    }

    retval.insert((1,1),corners_vec[0].clone());
    return retval;
  }

  // GENERAL CASE for widthxwidth where width>2
  for row in 0..width {
    for col in 0..width {
      // println!("Trying {} {}", row, col);

      if row == 0 && col == 0 {
        let mut t = corners_vec[0].clone();
        let index = corners_vec.iter().position(|x| *x == t).unwrap();
        corners_vec.remove(index);

        let t_id = t.id;
        // println!("Tile: {}", t_id);
        for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();

          if v == vec![0,1,1,0] {
            if t.rotation != None {
              // println!("already set my rotation for {} in size {}", t.id, width);
            } else {
              t.rotation = Some(my_rotation);
            }
            retval.insert((row,col),t.clone());
          }
        }
          let rotation_sides = &t.all_rotations()[t.rotation.unwrap()];
          let t1 = get_match(&borders_vec, t_id, rotation_sides[1]).unwrap().clone();
          let index = borders_vec.iter().position(|x| *x == t1).unwrap();
          borders_vec.remove(index);
          retval.insert((0,1),t1);

          let t2 = get_match(&borders_vec, t_id, rotation_sides[2]).unwrap().clone();
          let index = borders_vec.iter().position(|x| *x == t2).unwrap();
          borders_vec.remove(index);
          retval.insert((1,0),t2);
    

      } else if row == 0 && col == width-2 {
        let west_n = retval.get(&(row,col-1)).unwrap().clone();
        let mut t = retval.get_mut(&(row,col)).unwrap();
        let t_id = t.clone().id;
        for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&corners_vec, t_id, *s)).collect();

          let match_west_side = west_n.all_rotations()[west_n.rotation.unwrap()][1];
          if v == vec![0,1,0,0] && rotation_sides[3] == match_west_side {
          // if v == vec![0,1,0,0] {
            t.rotation = Some(my_rotation);
            let t1 = get_match(&corners_vec, t.id, rotation_sides[1]).unwrap().clone();
            let index = corners_vec.iter().position(|x| *x == t1).unwrap();
            corners_vec.remove(index);
            retval.insert((row,col+1),t1);
            break;

          }          
        }

      } else if row == 0 && col == width-1 { // UR corner
        let west_n = retval.get(&(row,col-1)).unwrap().clone();
        let mut t = retval.get_mut(&(row,col)).unwrap();
        for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();
          let match_west_side = west_n.all_rotations()[west_n.rotation.unwrap()][1];
          if v == vec![0,0,1,0] && rotation_sides[3] == match_west_side {
          // if v == vec![0,0,1,0] {
            t.rotation = Some(my_rotation);
            let t1 = get_match(&borders_vec, t.id, rotation_sides[2]).unwrap().clone();
            let index = borders_vec.iter().position(|x| *x == t1).unwrap();
            borders_vec.remove(index);
            retval.insert((row+1,col),t1);

            break;
          }
        }

      } else if row == width-2 && col == 0 {
        let north_n = retval.get(&(row-1,col)).unwrap().clone();
        let mut t = retval.get_mut(&(row,col)).unwrap();
        for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&corners_vec, t.id, *s)).collect();
          let match_north_side = north_n.all_rotations()[north_n.rotation.unwrap()][2];

          if v == vec![0,0,1,0] && rotation_sides[0] == match_north_side {
          // if v == vec![0,0,1,0] {
            t.rotation = Some(my_rotation);
            let t1 = get_match(&corners_vec, t.id, rotation_sides[2]).unwrap().clone();
            let index = corners_vec.iter().position(|x| *x == t1).unwrap();
            corners_vec.remove(index);
            retval.insert((row+1,col),t1);

            break;
          }
        }

      } else if row == width-1 && col == 0 { // LL corner
        let north_n = retval.get(&(row-1,col)).unwrap().clone();
        let mut t = retval.get_mut(&(row,col)).unwrap();
        for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();
          let match_north_side = north_n.all_rotations()[north_n.rotation.unwrap()][2];
          if v == vec![0,1,0,0] && rotation_sides[0] == match_north_side {
          // if v == vec![0,1,0,0] {
            t.rotation = Some(my_rotation);
            let t1 = get_match(&borders_vec, t.id, rotation_sides[1]).unwrap().clone();
            let index = borders_vec.iter().position(|x| *x == t1).unwrap();
            borders_vec.remove(index);
            retval.insert((row,col+1),t1);

            break;
          }
        }

      } else if row == width-1 && col == width-2 { // second from lower right corner
        let west_n = retval.get(&(row,col-1)).unwrap().clone();
        let mut t = retval.get_mut(&(row,col)).unwrap();
        for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&corners_vec, t.id, *s)).collect();
          let match_west_side = west_n.all_rotations()[west_n.rotation.unwrap()][1];
          if v == vec![0,1,0,0] && rotation_sides[3] == match_west_side {
          // if v == vec![0,1,0,0] {
            t.rotation = Some(my_rotation);
            let t1 = get_match(&corners_vec, t.id, rotation_sides[1]).unwrap().clone();
            let index = corners_vec.iter().position(|x| *x == t1).unwrap();
            corners_vec.remove(index);
            retval.insert((row,col+1),t1);

            break;
          }
        }

      } else if col == 0 || col == width-1 {
        let north_n = retval.get(&(row-1,col)).unwrap().clone();
          // let north_n = retval.get(&(row-1,col)).unwrap().clone();
          if retval.get(&(row+1,col)) == None {
            let mut t = retval.get_mut(&(row,col)).unwrap();
            for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
              let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();

              let match_north_side;
              if north_n.rotation == None {
                match_north_side = rotation_sides[0];
              } else {
                match_north_side = north_n.all_rotations()[north_n.rotation.unwrap()][2];
              }
              if v == vec![0,0,1,0] && rotation_sides[0] == match_north_side {
              // if v == vec![0,0,1,0] {
                t.rotation = Some(my_rotation);
                let t1 = get_match(&borders_vec, t.id, rotation_sides[2]).unwrap().clone();
                let index = borders_vec.iter().position(|x| *x == t1).unwrap();
                borders_vec.remove(index);
                retval.insert((row+1,col),t1);
    
                break;
              }
            }  
          }
        
      }  else if col > 0 && row > 0 && col < width-1 && row < width-1 {
        // println!("NO LOGIC - INSIDE");
        // retval.get(&(row,col)).unwrap();

      } else if row == 0 || row == width-1 {
        let west_n = retval.get(&(row,col-1)).unwrap().clone();
        if retval.get(&(row,col+1)) == None {
          let mut t = retval.get_mut(&(row,col)).unwrap();
          for (my_rotation,rotation_sides) in t.all_rotations().iter().enumerate() {
            let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();
            let match_west_side = west_n.all_rotations()[west_n.rotation.unwrap()][1];
            if v == vec![0,1,0,0] && rotation_sides[3] == match_west_side {
              t.rotation = Some(my_rotation);
  
              let t1 = get_match(&borders_vec, t.id, rotation_sides[1]).unwrap().clone();
              let index = borders_vec.iter().position(|x| *x == t1).unwrap();
              borders_vec.remove(index);
              retval.insert((row,col+1),t1);
  
              break;
            }
          }
  
        }

      } else {
        println!("SHOULD BE DOING SOMETHING");
      }
    }     
  }

  retval
}


pub fn count_matches(things : &Vec<Tile>, skip_id : u32, side : u32) -> u32 {
  let mut retval = 0;

  for t in things {
    if t.id != skip_id {
      for rotation_sides in t.all_rotations() {
        // let rotation_sides = &t.sides;
        let mut side_count = 0;
        for s in rotation_sides {
          if s == side { side_count += 1; }
        }
        retval = std::cmp::max(retval,side_count);
      }
    }
  }
  
  retval
}

pub fn get_match(things : &Vec<Tile>, skip_id : u32, side : u32) -> Option<Tile> {

  for t in things {
  if t.id != skip_id {
    for rotation_sides in t.all_rotations() {
      for s in rotation_sides {
        if s == side {
          return Some(t.clone());
        }
      }
    }
  }
}
None
}

pub fn border (things : &Vec<Tile>, corner_hint : Option<u32>) -> HashMap<(u32,u32),Tile> {
  let mut corners = vec![]; // std::collections::HashMap::<u32,Tile>::new();
  let mut borders = vec![]; // std::collections::HashMap::<u32,Tile>::new();
  let mut inner = vec![];

  // sort tiles into corner, borders and inner
  for t in things {
    for (_r_index,rotation) in t.all_rotations().iter().enumerate() {
      let mut unique_sides = 0;
      for (_index,s) in rotation.iter().enumerate() {
        let c = count_matches(&things,t.id,*s);
        if c == 0 {
          unique_sides += 1;
        }
      }
      if unique_sides == 2 {
        if let Some(id) = corner_hint { // use my hint to put the right tile first == upper left
          if id == t.id {
            corners.insert(0,t.clone());
          } else {
            corners.push(t.clone());
          }
        } else {
          corners.push(t.clone());
        }
        corners.dedup();
      } else if unique_sides == 1 {
        borders.push(t.clone());
        borders.dedup();
      } else {
        inner.push(t.clone());
        inner.dedup();
      }
    }
  }

  
  let size = (things.len() as f64).sqrt() as u32;
  let mut grid = build_border(&corners, &borders, size);
  let mut next_corner_hint = None;

  // if I have more than corners, figure out which tile should be the inner upper left
  // and pass it as my corner hint
  if size > 2 { 
    let mut upper_left_corner = vec![];
    // let mut upper_left_rotation;
    let mut which = 0;

    let top = grid.get(&(0,1)).unwrap().clone();
    let left = grid.get(&(1,0)).unwrap().clone();
    upper_left_corner.push(top.clone());
    upper_left_corner.push(left.clone());

    for (index,t) in inner.iter().enumerate() {
      for (_r_index,rotation) in t.all_rotations().iter().enumerate() {
        let mut unique_matches = 0;
        for (_index,s) in rotation.iter().enumerate() {
          let c = count_matches(&upper_left_corner, t.id, *s);
          if c == 1 {
            unique_matches += 1;
          }
        }
        if unique_matches == 2 {
            next_corner_hint = Some(t.id);
            // upper_left_rotation = Some(_r_index); // find_rotation(vec![top,None,None,left])
            which = index;
            break;
        }
      }
    }
    let a = top.all_rotations()[top.rotation.unwrap()][2];
    let b = left.all_rotations()[left.rotation.unwrap()][1];
    inner[which].rotation = inner[which].find_rotation(vec![Some(a.try_into().unwrap()),None,None,Some(b.try_into().unwrap())]); 
    grid.insert((1,1),inner[which].clone());
  }

  
  
  // if I have inner tiles, copy my inner grid into my outer grid before I return it
  if inner.len() > 1 {
    // build the border with my inner tiles (all over again)
    let sub_grid = border(&inner,next_corner_hint);
    for row in 0..size-2 {
      for col in 0..size-2 {
        // println!("Getting {} {} from subgrid", row, col);
        if let Some(t) = sub_grid.get(&(row,col)) {
          grid.insert((row+1,col+1),t.clone());

        }
      }
    }
  } else if inner.len() == 1 {
    let t = inner[0].clone();
    grid.insert((size/2,size/2),t);
  }
  return grid;
}

pub fn solve_part2(filename : String) -> u32 {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let tiles : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
  let mut things = Vec::new();
  let size = (tiles.len() as f64).sqrt() as u32;

  for tile in tiles {
    let lines : Vec<String> = tile.lines().map(|s| (&*s).to_string()).collect();
    things.push(make_tile(lines));
  }

  // call to make the successive borders
  let mut grid = border(&things,Some(1439));

  for grid_row in 0..size {
    for grid_col in 0..size {
      if !(grid_row == 0 && grid_col == 0) {
        let mut northern_side = 0;
        if let Some(t) = grid.get_mut(&(grid_row-1,grid_col)) {
          if let Some(r) = t.rotation {
            let v = t.all_rotations()[r].clone();
            northern_side = v[2];
          }
        }


        let mut eastern_side = 0;
        if let Some(t) = grid.get_mut(&(grid_row,grid_col+1)) {
          if let Some(r) = t.rotation {
            let v = t.all_rotations()[r].clone();
            eastern_side = v[3];
          }
        }


        if let Some(t) = grid.get_mut(&(grid_row,grid_col)) {
            for (i,v) in t.all_rotations().iter().enumerate() {
              // let v = t.all_rotations()[r].clone();
              let mut matches = true;
              let mut match_count = 0;
              if northern_side != 0 && northern_side != v[0] {
                matches = false;
              } else if northern_side == v[0] {
                match_count = match_count + 1;
              }

              if eastern_side != 0 && eastern_side != v[1] {
                matches = false;
              } else if eastern_side == v[1] {
                match_count = match_count + 1;
              }
              
              if let Some(r) = t.rotation {
                if matches && match_count > 0 && i != r {
                  // println!("Tile {} at ({},{}) found new rotation {} with {} matches", t.id, grid_row, grid_col, i, match_count);
                  // println!("old rotation {:?}, new rotation {:?}", t.all_rotations()[r], t.all_rotations()[i]);
                  t.rotation = Some(i);
                }
              } else if match_count > 0 {
                // println!("Tile {} at ({},{}) found rotation {} with {} matches", t.id, grid_row, grid_col, i, match_count);
                // println!("new rotation {}", i);
                t.rotation = Some(i);
              }
          }
        }
      }
    }
  }

// println!("{}", t.lines.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n"));
for grid_row in 0..size*10 {
  for grid_col in 0..size*10 {
    let (r,c) = (grid_row,grid_col);
    if let Some(t) = grid.get(&((r/10).try_into().unwrap(),(c/10).try_into().unwrap())) {
      print!("{}", t.get_row_col_with_border((r % 10).try_into().unwrap(),(c % 10).try_into().unwrap()));
    } else {
      print!(" ");
    }    
    if c % 10 == 9 { print!(" ")};
  }
  println!();
  if grid_row % 10 == 9 {println!(); }

}

// the following is the actual eventual image
let sea_monster : Vec<Vec<char>> = vec![
  "                  O ".as_bytes().iter().map(|b| *b as char).collect(),
  "\\    /\\    /\\    OO>".as_bytes().iter().map(|b| *b as char).collect(),
  " \\  /  \\  /  \\  /   ".as_bytes().iter().map(|b| *b as char).collect() 
];
let monster_width = sea_monster[0].len();
let monster_height = sea_monster.len();

// searching
let mut monsters = vec![];
let mut monster_map = HashMap::new();

let mut my_rotation = 0;
for rotation in 0..12 {
  my_rotation = rotation;
  for grid_row  in 0..size*8-monster_height as u32 {
    for grid_col in 0..size*8-monster_width as u32 {
      let (r,c) = rotate_coordinates(grid_row.try_into().unwrap(), grid_col.try_into().unwrap(), size as usize*8, rotation);
      // let (r,c) = (grid_row,grid_col);

      // search for the monster
      let mut monster_match = 0;
      let mut mmap = HashMap::new();

      for (sm_row,sm_line) in sea_monster.iter().enumerate() {
        for (sm_col,sm) in sm_line.iter().enumerate() {
          let (effective_row,effective_col) = rotate_coordinates((grid_row+sm_row as u32).try_into().unwrap(), (grid_col+sm_col as u32).try_into().unwrap(), size as usize*8, rotation);
          if effective_row <= (size*8).try_into().unwrap() && effective_col <= (size*8).try_into().unwrap() { // { panic!("effective row is too big {} vs {}", effective_row, size*8)};
            if let Some(t) = grid.get(&((effective_row/8).try_into().unwrap(),(effective_col/8).try_into().unwrap())) {
              let pixel = t.get_row_col((effective_row % 8).try_into().unwrap(),(effective_col % 8).try_into().unwrap());
              if *sm != ' ' && pixel == '#' {
                monster_match = monster_match + 1;
                mmap.insert((effective_row,effective_col),sea_monster[sm_row][sm_col]);
              }
            }        
          } // check if in bounds  
        }
      }
      if monster_match == 15 { 
        monsters.push((r,c)); 
        monster_map.extend(mmap);
      }      
    }
  }
  if monsters.len() > 0 { println!("rotation is {}", my_rotation); break; }
}

let mut hash_count = 0;
for grid_row in 0..size*8 {
    for grid_col in 0..size*8 {
      let (r,c) = rotate_coordinates(grid_row.try_into().unwrap(), grid_col.try_into().unwrap(), 8 as usize * size as usize, my_rotation);
      if let Some(t) = grid.get(&((r/8).try_into().unwrap(),(c/8).try_into().unwrap())) {
        let pixel = t.get_row_col((r % 8).try_into().unwrap(),(c % 8).try_into().unwrap());
        if pixel == '#' { hash_count = hash_count + 1; }

        if let Some(m) = monster_map.get(&(r,c)) {
            print!("{}", m.to_string().purple());
        } else {
          print!("{}", pixel);
        }
      } else {
        print!(" ");
      }   
    }
    println!();

  }
  println!("Monsters {:?}", monsters.len());

  println!("Rough water count {}", hash_count - monsters.len()*15);
  return hash_count as u32 - monsters.len() as u32 *15;
}