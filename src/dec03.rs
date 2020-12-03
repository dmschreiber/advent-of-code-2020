use std::collections::HashMap;
use std::fs;

pub fn display_map(map: HashMap<(u32,u32),u8>) {
  let mut max_row = 0;
  let mut max_col = 0;

  for k in map.keys() {
    let (a,b) = k;
    if *a > max_row { max_row = *a; }
    if *b > max_col { max_col = *b; }
  }
  for row in 0..=max_row {
    for col in 0..=max_col {
      let b = map.get(&(row,col));
      if let Some(i) = b {
        print!("{}", *i as char);
      }
    }
    println!();
  }
}

pub fn display_tree_map(map: &HashMap<(u32,u32),u8>) {
  let mut max_row = 0;
  let mut max_col = 0;

  for k in map.keys() 
  {
    let (a,b) = k;
    if *a > max_row { max_row = *a; }
    if *b > max_col { max_col = *b; }
  }
  for row in 0..=max_row 
  {
    for col in 0..=max_col 
    {
      let b = map.get(&(row,col));
      if let Some(i) = b {
        print!("{}", *i as char);
      }
    }
    println!();
  }
}

pub fn read_tree_map(filename: String) -> HashMap<(u32,u32),u8> {
  let mut map = HashMap::new();

  let mut col = 0;
  let mut row = 0;

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");
  
  let bytes = contents.as_bytes();
  for &item in bytes {
      if item == 10 {
        row += 1;
        col = 0;
      } else {
        map.entry((row,col)).or_insert(item);
        col += 1;
      }
  }

  map
}  

pub fn solve(map: &HashMap<(u32,u32),u8>, row_step: u32, col_step: u32) -> u32 {
  let mut max_row = 0;
  let mut max_col = 0;

  for k in map.keys() 
  {
    let (a,b) = k;

    if *a > max_row { max_row = *a; }
    if *b > max_col { max_col = *b; }
  }

  println!("max {},{}", max_row, max_col);

  let mut col = 0;
  let mut tree_count = 0;
  for row in (row_step..=max_row).step_by(row_step as usize)
  {
    col = col + col_step;
    col = col % (max_col + 1);

    let b = map.get(&(row,col));
      if let Some(i) = b {
        if *i == 35 {
          // tree
          tree_count = tree_count + 1;
          // println!("Tree {},{} - {}", row, col, *i as char);
        } else if *i == 46 {
          // blank
          // println!("No Tree {},{} - {}", row, col, *i as char);
        } else {
          println!("Problem");
        }
      }
  }
  // println!("tree count {}", tree_count);
  tree_count
}
