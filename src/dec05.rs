use std::fs;

pub fn read_input(filename: String) -> Vec<String> {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let seats: Vec<String> = contents.lines().map(|s| (&*s).to_string() ).collect();
  seats
}

const ROW_HI: u32 = 127;
const ROW_LO: u32 = 0;
const COL_HI: u32 = 7;
const COL_LO: u32 = 0;

fn calculate_seat_id(seat: String) -> u32 {

  // let b1 = seat.replace("B","1");
  // let b11 = b1.replace("F", "0");
  // let b2 = b11.replace("R","1");
  // let b21 = b2.replace("L","0");

  // let seat_id = isize::from_str_radix(&b21,2).unwrap();
  // return seat_id as u32;

  let instructions = seat.as_bytes();
  let mut row_hi = ROW_HI;
  let mut row_lo = ROW_LO;
  let mut col_hi = COL_HI;
  let mut col_lo = COL_LO;

  for i in instructions {
    if *i == b'F' {
      row_hi = row_hi - (row_hi-row_lo)/2;
    } else if *i == b'B' {
      let diff = row_hi - row_lo;
      row_lo = row_lo + diff/2 + diff % 2;
    } else if *i == b'L' {
      col_hi = col_hi - (col_hi-col_lo)/2;
    } else if *i == b'R' {
      let diff = col_hi - col_lo;
      col_lo = col_lo + diff/2 + diff % 2;
    }
  }
  // println!("seat {} row {} col {}", seat, row_lo, col_lo);
  return row_lo * 8 + col_lo;
}

pub fn calc_seat_ids(seats: &Vec<String>) -> Vec<u32> {
  let mut seat_id : Vec<u32>;

  seat_id = seats.iter().map(|s| calculate_seat_id(s.to_string())).collect();

  seat_id.sort();
  seat_id
}

pub fn solve_missing(seat_ids: &Vec<u32>) -> u32 {
  return solve_missing_at(seat_ids, 0);
}

fn solve_missing_at(seat_ids: &Vec<u32>, which: usize) -> u32 {
  
  if which == 0 {return solve_missing_at(seat_ids, 1); }
  if which >= seat_ids.len() {return 999; }

  let my_seat_id : u32 = seat_ids[which];
  let last_seat : u32 = seat_ids[which-1];

  if my_seat_id - last_seat == 2 {
    return last_seat + 1;
  } else{
    return solve_missing_at(seat_ids, which+1);
  }

}

pub fn solve_max(seat_ids: &Vec<u32>) -> u32 {
  return *seat_ids.iter().max().unwrap();
}

pub fn validate() {
  assert!(calculate_seat_id("BFFFBBFRRR".to_string())==567);
  assert!(calculate_seat_id("FFFBBBFRRR".to_string())==119);
}
