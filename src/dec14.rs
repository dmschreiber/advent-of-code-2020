use std::collections::HashMap;

// ********** COMMON ***************
#[derive(Debug)]
enum Arg {
  Mask(String),
  Mem(u64,u64),
}

fn new_num(num: String, mask: String) -> u64 {
  let retval;
  let mut ret_string : String = "".to_string();

  for (i,b) in mask.as_bytes().iter().enumerate() {
    if *b == b'X' {
      ret_string = ret_string + &String::from(&num[i..i+1]);
    } else if *b == b'0' {
      ret_string = ret_string + &String::from("0");
    } else {
      ret_string= ret_string + &String::from("1");
    }
  }

  retval = u64::from_str_radix(&ret_string,2).unwrap();

  retval
}

// Part 2 - vary address by the float positions marked with X
fn vary_address(address: String, addresses : &mut Vec<u64>) {
  if address.find("X") == None {
    addresses.push(u64::from_str_radix(&address,2).unwrap());
  } else {
    if let Some(i) = address.find("X") {
      vary_address(address[..i].to_string() + &"0".to_string() + &address[i+1..].to_string(), addresses);
      vary_address(address[..i].to_string() + &"1".to_string() + &address[i+1..].to_string(), addresses);
    }
  }
}

// Part 2 - calculate the addresses from the mask
fn new_address(address: String, mask: String) -> Vec<u64> {
  let mut new_addresses = <Vec<u64>>::new();
  let mut ret_string : String = "".to_string();

  for (i,b) in mask.as_bytes().iter().enumerate() {
    if *b == b'X' {
      ret_string += "X"; 
    } else if *b == b'0' {
      ret_string += &address[i..i+1].to_string();
    } else {
      ret_string += "1";
    }
  }
  vary_address(ret_string, &mut new_addresses);
  new_addresses
}

pub fn solve() {
  // Create vector of structs
  let lines: Vec<String> = include_str!("../inputs/dec14.txt").lines().map(|s| (&*s).to_string() ).collect();
  let mut things = <Vec<Arg>>::new();

  for line in lines {
    let args : Vec<String> = line.split(" = ").map(|s| (&*s).to_string() ).collect();
    let thing;
    if args[0] == "mask" {
      thing = Arg::Mask(args[1].to_string());
    } else {
      let one = args[0][4..args[0].len()-1].parse::<u64>().unwrap();
      thing = Arg::Mem(one,args[1].parse::<u64>().unwrap());
    }

    things.push(thing);
  }

  // solver logic
  let mut mask : String = "".to_string();
  let mut mem = <HashMap<u64,u64>>::new();

  for thing in &things {
    match thing {
      Arg::Mask(m) => {
        mask = m.to_string();
      }
      Arg::Mem(index,value) => {
        let num = format!("{:036b}", value);
        let new_num = new_num(num,mask.to_string());
        mem.insert(*index,new_num);
      }
    }
  }

  let retval = mem.values().sum::<u64>();
  println!("Day 14 part 1 sum is {:?}", retval);

  //// PART 2
  let mut mask : String = "".to_string();
  let mut mem = <HashMap<u64,u64>>::new();

  for thing in &things {
    match thing {
      Arg::Mask(m) => {
        mask = m.to_string();
      }
      Arg::Mem(index,value) => {
        let num = format!("{:036b}", index);
        let new_add = new_address(num,mask.to_string());
        for a in new_add {
          mem.insert(a,*value);
        }
      }
    }
  }

  let retval = mem.values().sum::<u64>();
  println!("Day 14 part 2 sum is {:?}", retval);
}
