#[cfg(test)]
mod tests {

  #[test]
  pub fn dec22_prod() {
    super::solve_part1("./inputs/dec22.txt".to_string());
    super::solve_part2("./inputs/dec22.txt".to_string());

  }
  #[test]
  pub fn dec22_test() {
      assert!(306==super::solve_part1("./inputs/dec22-test.txt".to_string()));
      assert!(291==super::solve_part2("./inputs/dec22-test.txt".to_string()));
      super::format_deck_v3(&vec![45,35,48,34,42,18,32,11,47,38,37,5,46,28,25,24,33,26,44,29,31,17,43,27,49,30,36,13]);
      super::format_deck_v3(&vec![45,35,48,34,42,18,32,47,11,38,37,5,46,28,25,24,33,26,44,29,31,17,43,27,49,30,36,13]);
  }  
}

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::time::{Instant};
use sha2::{Sha256, Digest};


pub fn solve_part1(filename : String) -> u64 {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let players : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
  let player1_lines : Vec<String> = players[0].lines().map(|s| (&*s).to_string()).collect();
  let player2_lines : Vec<String> = players[1].lines().map(|s| (&*s).to_string()).collect();

  let mut player1 : Vec<u32> = player1_lines[1..].iter().map(|l| l.parse::<u32>().unwrap() ).collect();
  let mut player2 : Vec<u32> = player2_lines[1..].iter().map(|l| l.parse::<u32>().unwrap() ).collect();

  let mut round = 1;
  println!("Player 1 deck {:?}", player1);

  while player1.len() > 0 && player2.len() > 0 {
    let card1 = player1[0];
    let card2 = player2[0];

    player1.remove(0);
    player2.remove(0);
    if card1 > card2 {
      println!("{} v {}, player 1 wins", card1, card2);
      player1.insert(player1.len(),card1);
      player1.insert(player1.len(), card2);
    } else {
      println!("{} v {}, player 2 wins", card1, card2);
      player2.insert(player2.len(), card2);
      player2.insert(player2.len(),card1);
    }
    round += 1;

  }
  let mut acc = 0;
  if player1.len() == 0 {
    println!("player 2 wins game");
    for (index,n) in player2.iter().enumerate() {
      acc = acc + *n as u64 * (player2.len() as u64 - index as u64);
    }
  } else {
    println!("player 1 wins game");
    for (index,n) in player1.iter().enumerate() {
      acc = acc + *n as u64 * (player1.len() as u64 - index as u64);
    }
  }
  println!("GAME SCORE {} ",acc);
  return acc;
  // println!("{:?}", player1);
  // println!("{:?}", player2);
  // return player1.iter().fold(1, |acc,n| acc * *n as u64 );
}

// 1~2~3
fn format_deck(d : &Vec<u32>) -> String {
  let deck_string = d.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
  // println!("{}", deck_string);
  return deck_string;
}

fn format_deck_v2(d : &Vec<u32>) -> u128 {
  let deck : u128 = d.iter().fold(0, |acc, n| (acc << 6) as u128 + *n as u128);
  // println!("{:?} -> {}", d, deck);
  return deck;
}

fn format_deck_v3(d : &Vec<u32>) -> u64 {

  // process input message
  let deck_string = d.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");

  // acquire hash digest in the form of GenericArray,
  // which in this case is equivalent to [u8; 20]

  // println!("{} {}", result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
  let mut s = sha2::Sha256::new();
  s.input(deck_string.as_bytes());
  let retval = s.result()[..].iter().fold(0, |acc,n| (acc << 8) as u64 + *n as u64);
  println!("{:?}",retval);
  retval
}

fn get_loser(w : u32) -> u32 {
  let ed;
  if w == 1 {
    ed = 2;
  } else {
    ed = 1;
  }
  return ed;
}

fn play_combat(p1 : &Vec<u32>, p2 : &Vec<u32>, sub_game_history : &mut HashMap::<(u32,u128,u128),u32>, game : u32) -> (u32,Vec<u32>) {
  let start = Instant::now();

  // println!("Let's play game {}", game);
  let mut player1 = p1.clone();
  let mut player2 = p2.clone();

  let mut deck_history = vec![];
  let mut round = 1;
  let mut winner = 0;
  while winner == 0 
  {
    if game <= 3 { 
      println!("Game {} round {} (history {}) player 1: {} player 2: {}", game, round, sub_game_history.keys().len(), player1.len(), player2.len()); 
    }
    // if cards are in teh same order as in past, player 1 wins
    let current_decks = (format_deck_v2(&player1),format_deck_v2(&player2));
    // let current_decks = format!("{}:{}",format_deck(&player1), format_deck(&player2));
    if deck_history.contains(&current_decks) 
    {
      // println!("INFITE LOOP"); // player 1 wins
      winner = 1;
      break;
    } else {
      deck_history.push(current_decks);

      // println!("{:?}", deck_history);
      let card1 = player1[0];
      let card2 = player2[0];

      player1 = player1[1..].to_vec(); // player1.remove(0);
      player2 = player2[1..].to_vec(); // .remove(0);

      let round_winner;
      if player1.len() >= card1 as usize && player2.len() >= card2 as usize {
        if game <= 3 { 
          println!("playing a subgame to determine winner");
        }
        // let local_start = Instant::now();
        let current_decks = (game, format_deck_v2(&player1),format_deck_v2(&player2));
        
        let history_w = sub_game_history.get(&current_decks);

        if let Some(w) = history_w {
          round_winner = *w;
        } else {
          let (sub_game_winner, _deck) = play_combat(&player1, &player2, sub_game_history, game+1);
          sub_game_history.insert(current_decks, sub_game_winner);
          sub_game_history.insert((current_decks.0,current_decks.2,current_decks.1),get_loser(sub_game_winner));
          round_winner = sub_game_winner;
        }
      } else {
        if card1 > card2 {
          // println!("{} v {}, player 1 wins", card1, card2);
          round_winner = 1;
        } else {
          round_winner = 2;
          // println!("{} v {}, player 2 wins", card1, card2);
        }
      }
      if round_winner == 1 {
        player1.insert(player1.len(),card1);
        player1.insert(player1.len(), card2);
      } else {
        player2.insert(player2.len(), card2);
        player2.insert(player2.len(),card1);

      }

      if player1.len() == 0 { winner = 2; }
      if player2.len() == 0 { winner = 1; }
    }
    round +=1;
  }
  // println!("Game winner player {}", winner);
  if game == 4 {
   println!("Level {} took {:?}", game, start.elapsed());
  }

  if winner == 1 {
    (winner,player1)
  } else {
    (winner,player2)
  }
}
pub fn solve_part2(filename : String) -> u64 {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let players : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
  let player1_lines : Vec<String> = players[0].lines().map(|s| (&*s).to_string()).collect();
  let player2_lines : Vec<String> = players[1].lines().map(|s| (&*s).to_string()).collect();

  let player1 : Vec<u32> = player1_lines[1..].iter().map(|l| l.parse::<u32>().unwrap() ).collect();
  let player2 : Vec<u32> = player2_lines[1..].iter().map(|l| l.parse::<u32>().unwrap() ).collect();

  let mut sub_game_history = HashMap::new();
  let (winner, winner_deck) = play_combat(&player1, &player2, &mut sub_game_history, 1);

  let mut acc = 0;
  if winner == 2 {
    println!("player 2 wins game");
  } else {
    println!("player 1 wins game");
  }

  for (index,n) in winner_deck.iter().enumerate() {
    acc = acc + *n as u64 * (winner_deck.len() as u64 - index as u64);
  }

  println!("GAME SCORE {} ",acc);
  return acc;
}