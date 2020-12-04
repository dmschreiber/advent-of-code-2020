use std::collections::HashMap;
use std::fs;



#[allow(dead_code)]
pub fn display_tree_map(trees: &TreeMap) {
  let max_row = trees.max_row;
  let max_col = trees.max_col;

  for row in 0..=max_row 
  {
    for col in 0..=max_col 
    {
      let b = trees.map.get(&(row,col));
      if let Some(i) = b {
        print!("{}", *i as char);
      }
    }
    println!();
  }
}

pub struct TreeMap {
  map: HashMap<(u32,u32),u8>,
  max_row: u32,
  max_col: u32,
}

pub fn read_tree_map(filename: String) -> TreeMap {
  let mut tree = TreeMap {map: HashMap::new(), max_col: 0, max_row: 0};

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
        tree.map.entry((row,col)).or_insert(item);
        if col > tree.max_col { tree.max_col = col; }
        if row > tree.max_row { tree.max_row = row; }
        col += 1;
      }
  }

  tree
}  

pub fn solve(tree: &TreeMap, row_step: u32, col_step: u32) -> u32 {
  let max_row = tree.max_row;
  let max_col = tree.max_col;

  // println!("max {},{}", max_row, max_col);

  let mut col = 0;
  let mut tree_count = 0;
  for row in (row_step..=max_row).step_by(row_step as usize)
  {
    col = col + col_step;
    col = col % (max_col + 1);

    let b = tree.map.get(&(row,col));
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
