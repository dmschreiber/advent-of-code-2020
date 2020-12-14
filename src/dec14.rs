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
  println!("mask   {}", mask);
  println!("num    {}", num);
  println!("result {}", ret_string);

  retval = u64::from_str_radix(&ret_string,2).unwrap();

  retval
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
      println!("{},{}",&args[0][4..(args[0].len()-1)],&args[1]);
      let one = args[0][4..args[0].len()-1].parse::<u64>().unwrap();
      thing = Arg::Mem(one,args[1].parse::<u64>().unwrap());
    }

    things.push(thing);
  }

  // println!("{:?}", things);
  let mut mask : String = "".to_string();
  let mut mem = <HashMap<u64,u64>>::new();

  for thing in things {
    match thing {
      Arg::Mask(m) => {
        mask = m.to_string();
      }
      Arg::Mem(index,value) => {
        let num = format!("{:036b}", value);
        // println!("{}", num);
        // println!("{}", mask);
        let new_num = new_num(num,mask.to_string());

        mem.insert(index,new_num);

      }
    }
  }
  let mut retval = 0;
  for m in mem.values() {
    retval += m;
  }


  println!("{:?}", retval);
}
