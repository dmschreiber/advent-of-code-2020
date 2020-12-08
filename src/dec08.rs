use std::fs;
use std::collections::HashMap;

pub fn read_input(filename: String) -> Vec<String> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let rules: Vec<String> = contents.lines().map(|s| (&*s).to_string() ).collect();

  rules
}

pub fn create_structure(rule_lines: &Vec<String>) -> Vec<Thing> {
  let mut things = <Vec<Thing>>::new();

  for r in rule_lines {
    let items : Vec<String> = r.split_ascii_whitespace().map(|s| (&*s).to_string() ).collect();

    let my_op : &str = &items[0];

    let operator = match my_op {
      "jmp" => Operator::Jmp,
      "acc" => Operator::Acc,
      "nop" => Operator::Nop,
      _ => panic!("Invalid char"),
    };

    let argument = &items[1].parse::<i32>().unwrap();

    let new_thing = Thing {operator: operator, argument: *argument};
    things.push(new_thing);
  }
  // println!("{:?}", things);
  return things;
}

#[derive(Debug)]
pub struct Thing {
  operator: Operator,
  argument: i32,
}

#[derive(Debug)]
enum Operator {
  Nop,
  Acc,
  Jmp,
}

pub fn solve(things: &Vec<Thing>) -> i32 {
  return solve_part2_sub(things,None);
}

pub fn solve_part2_sub(things: &Vec<Thing>, change_index: Option<usize>) -> i32 {
  let mut accumulator = 0;
  let mut index = 0 as usize;
  let mut history = <HashMap<usize,bool>>::new();
  
  while history.get(&index) == None {
    history.insert(index, true);
    let v = things.get(index).unwrap();

    // Swap the Operation of at the specified index
    let op;
    match v.operator {
      Operator::Nop => {
        if (change_index != None) && (Some(index) == change_index) {
          op = Operator::Jmp;
        } else {
          op = Operator::Nop;
        }
      }
      Operator::Jmp => {
        if (change_index != None) && (Some(index) == change_index) {
          op = Operator::Nop;
        } else {
          op = Operator::Jmp;
        }
      }
      Operator::Acc => { op = Operator::Acc; }
    }

    // Do the Operation
    match op {
      Operator::Nop => {
        index += 1;
      } 
      Operator::Acc => {
        accumulator = v.argument + accumulator;
         index += 1;
      } 
      Operator::Jmp => {
        let tmp = (v.argument + index as i32) as usize;
        index = tmp;
      }  
    }
    if index >= things.len() {
      println!("Success at {:?} Final accumulator {}, index {}, and {:?}", change_index, accumulator, index, history.get(&index));
      // retval = 0;
      break;
    }
  }
  if change_index == None {
    // running program as-is
    println!("Infinte loop at {:?} Final accumulator {}, index {}, and {:?}", change_index, accumulator, index, history.get(&index));
    return accumulator
  } else if change_index != None && index >= things.len() {
    // changed one operator and found success
    return accumulator
  } else
  {
    // changed one operator and hit infinte loop
    return 0
  }
}

pub fn solve_part2(things: &Vec<Thing>) -> i32{
  for (i,_t) in things.iter().enumerate() {
    let retval = solve_part2_sub(things,Some(i));

    if retval != 0 {
      return retval;
    }
  }
  return 0;
}