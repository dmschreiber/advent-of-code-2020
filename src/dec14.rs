use std::collections::HashMap;

// ********** COMMON ***************
#[derive(Debug)]
enum Arg {
  Mask(String),
  Mem(u64,u64),
}

fn new_num(num: String, mask: String) -> u64 {
  let mut retval = 0;
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
  // println!("mask   {}", mask);
  // println!("num    {}", num);
  // println!("result {}", ret_string);

  retval = u64::from_str_radix(&ret_string,2).unwrap();

  retval
}

fn vary_address(address: String, addresses : &mut Vec<u64>) {
  if address.find("X") == None {
    addresses.push(u64::from_str_radix(&address,2).unwrap());
  } else {
      if let Some(i) = address.find("X") {
        // let mut new_add = address.clone();
        // new_add.replace_range(i..i+1,"0");
        // println!("{}-{}",address[..i].to_string(), &address[i+1..].to_string());
        vary_address(address[..i].to_string() + &"0".to_string() + &address[i+1..].to_string(), addresses);
        vary_address(address[..i].to_string() + &"1".to_string() + &address[i+1..].to_string(), addresses);
        // new_add.replace_range(i..i+1,"1");
        // vary_address(new_add, addresses);
      }
    }
}

fn new_address(address: String, mask: String) -> Vec<u64> {
  let mut new_addresses = <Vec<u64>>::new();
  let mut ret_string : String = "".to_string();

  for (i,b) in mask.as_bytes().iter().enumerate() {
    if *b == b'X' {
      ret_string = ret_string + "X"; // &String::from(&address[i..i+1]);
    } else if *b == b'0' {
      ret_string = ret_string + &String::from(&address[i..i+1]);
    } else {
      ret_string= ret_string + &String::from("1");
    }
  }
  // println!("num    {}", address);
  // println!("mask   {}", mask);
  // println!("result {}", ret_string);

  vary_address(ret_string, &mut new_addresses);
  new_addresses
}

pub fn solve() {
  let lines: Vec<String> = include_str!("../inputs/dec14.txt").lines().map(|s| (&*s).to_string() ).collect();
  let mut things = <Vec<Arg>>::new();

  for line in lines {
    let args : Vec<String> = line.split(" = ").map(|s| (&*s).to_string() ).collect();
    let thing;
    if args[0] == "mask" {
      thing = Arg::Mask(args[1].to_string());
    } else {
      // println!("{},{}",&args[0][4..(args[0].len()-1)],&args[1]);
      let one = args[0][4..args[0].len()-1].parse::<u64>().unwrap();
      thing = Arg::Mem(one,args[1].parse::<u64>().unwrap());
    }

    things.push(thing);
  }

  // println!("{:?}", things);
  let mut mask : String = "".to_string();
  let mut mem = <HashMap<u64,u64>>::new();

  for thing in &things {
    match thing {
      Arg::Mask(m) => {
        mask = m.to_string();
      }
      Arg::Mem(index,value) => {
        let num = format!("{:036b}", value);
        // println!("{}", num);
        // println!("{}", mask);
        let new_num = new_num(num,mask.to_string());
        mem.entry(*index).or_insert(new_num);
        // mem.insert(*index,new_num);

      }
    }
  }
  let mut retval = 0;
  for m in mem.values() {
    retval += m;
  }
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
        // println!("{}", num);
        // println!("{}", mask);
        let new_add = new_address(num,mask.to_string());
        for a in new_add {
          mem.insert(a,*value);
        }
      }
    }
  }
  let mut retval = 0;
  for m in mem.values() {
    retval += m;
  }

  println!("Day 14 part 2 sum is {:?}", retval);
}
