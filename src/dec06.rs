use std::fs;


pub fn read_input(filename: String) -> Vec<String> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let customs: Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string() ).collect();

  customs
}

fn is_q_in_form(f: &String, q: char) -> u32 {
  if f.find(q) != None {
    return 1;
  } else {
    return 0;
  }
}
pub fn solve(forms: &Vec<String>) -> u32 {
  let questions = "abcdefghijklmnopqrstuvwxyz".as_bytes();
  let mut total_answer_count = 0;

  for f in forms {
    for q in questions {
      total_answer_count = total_answer_count + is_q_in_form(&f, *q as char);
    }
  }
  total_answer_count
}

pub fn solve_part_2(forms: &Vec<String>) -> u32 {
  let questions = "abcdefghijklmnopqrstuvwxyz".as_bytes();
  let mut total_answer_count = 0;

  for f in forms {
    for q in questions {
      let mut question_count = 0;
      let mut people_count = 0;
      let people = f.lines();
      for person in people {
        people_count += 1;
        if person.find(*q as char) != None {
          question_count += 1;
        }
      }
      if question_count == people_count {
        total_answer_count += 1;
      }
      
    }
  }
  total_answer_count
}