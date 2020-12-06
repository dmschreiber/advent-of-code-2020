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

  let total_answer_count = forms.iter().map(|f|
    questions.iter().map(|&q| is_q_in_form(&f, q as char)).sum::<u32>()
  ).sum::<u32>();

  total_answer_count
}

fn are_all_q_in_form(people: &Vec<&str>, q: char, question_count: usize) -> u32 {

  if people.len() == question_count {
    return 1;
  } else {
    if people[question_count].find(q as char) != None {
      return are_all_q_in_form(people, q, question_count + 1);
    } else {
      return 0;
    }
  }
}

fn tally_form(f: &String) -> u32 {
  let questions = "abcdefghijklmnopqrstuvwxyz".as_bytes();
  let people = f.lines().collect::<Vec<&str>>();

   return questions.iter().map(|&q| are_all_q_in_form(&people, q as char, 0)).sum::<u32>();

}

pub fn solve_part_2(forms: &Vec<String>) -> u32 {
  let total_answer_count;

  total_answer_count = forms.iter().map(|f| tally_form(&f)).sum::<u32>();

  total_answer_count
}