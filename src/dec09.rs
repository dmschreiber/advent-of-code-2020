use std::collections::HashMap;

pub fn create_structure(lines: &Vec<String>) -> Vec<Thing> {
  let mut things = <Vec<Thing>>::new();

  for line in lines {
    things.push( Thing{ argument: 1, operator: Operator::Nop, } );
  }
  things
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

pub fn solve(things: &Vec<Thing>) -> u32 {
  let retval = 0;

  retval
}