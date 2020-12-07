
#[macro_use] extern crate lazy_static;
use std::time::{Instant};
mod dec01;
mod dec02;
mod dec03;
mod dec04;
mod dec05;
mod dec06;
mod dec07;

fn dec01() {
    let mut my_numbers = dec01::read_input("./inputs/dec01.txt".to_string());

    // sort
    my_numbers.sort();
    // println!("{:?}", my_numbers);

    dec01::solve(my_numbers.clone());
    dec01::solve_part2(my_numbers.clone());

}

fn dec02() {
    let my_passwords = dec02::read_input("./inputs/dec02.txt".to_string());

    dec02::solve_part1(&my_passwords);
    dec02::solve_part2(&my_passwords);
}

fn dec03() {
    let my_tree_map = dec03::read_tree_map("./inputs/dec03.txt".to_string());

    let p1 = dec03::solve(&my_tree_map,1,3) as u128;
    println!("{} trees found in part 1", p1);

    let p = 
    dec03::solve(&my_tree_map,1,1) as u128 *
    p1*
    dec03::solve(&my_tree_map,1,5) as u128*
    dec03::solve(&my_tree_map,1,7) as u128*
    dec03::solve(&my_tree_map,2,1) as u128;
    println!("{} is the product of all", p);

}

fn dec04() {

    // assert!(dec04::validate_birthyear("2002"));
    // assert!(!dec04::validate_birthyear("2003"));
    // assert!(dec04::validate_height("60in"));
    // assert!(dec04::validate_height("190cm"));
    // assert!(!dec04::validate_height("190in"));
    // assert!(!dec04::validate_height("190"));
    // assert!(dec04::validate_haircolor("#123abc"));
    // assert!(!dec04::validate_haircolor("#123abz"));
    // assert!(!dec04::validate_haircolor("123abc"));
    // assert!(dec04::validate_eyecolor("brn"));
    // assert!(!dec04::validate_eyecolor("wat"));
    // assert!(dec04::validate_pid("000000001"));
    // assert!(!dec04::validate_pid("0123456789"));


    let passports = dec04::read_input("./inputs/dec04.txt".to_string());

    dec04::solve(&passports, false);

    dec04::solve(&passports, true);

}

fn dec05() {

    dec05::validate();

    let start = Instant::now();
    let seats = dec05::read_input("./inputs/dec05.txt".to_string());
    let seat_ids = dec05::calc_seat_ids(&seats);
    println!("max seat_id is {}", dec05::solve_max(&seat_ids, 0, 0));
    println!("my seat is {}", dec05::solve_missing(&seat_ids));
    println!("Day 5 complete in {:?}", start.elapsed());

}

fn dec06() {
    let start = Instant::now();
    let customs = dec06::read_input("./inputs/dec06.txt".to_string());
    let answer =dec06::solve(&customs);
    println!("Part 1 customs questions are {}", answer);
    assert!(answer == 6930);

    let answer = dec06::solve_part_2(&customs);
    assert!(answer == 3585);
    println!("Part 2 customs questions are {}", answer );

    println!("Day 6 complete in {:?}", start.elapsed());
}

fn dec07() {
    let rules = dec07::read_input("./inputs/dec07.txt".to_string());

    let start = Instant::now();
    let structured_rules = dec07::create_structure(&rules);
    assert!(dec07::solve(&structured_rules)==242);
    println!("Day 7 part 1 complete in {:?}", start.elapsed());
    let start = Instant::now();

    assert!(dec07::solve_part2(&structured_rules)==176035);
    println!("Day 7 part 2 complete in {:?}", start.elapsed());

}

fn main() {
    let start = Instant::now();
    dec01();
    println!("Day 1 complete in {:?}", start.elapsed());

    let start = Instant::now();
    dec02();
    println!("Day 2 complete in {:?}", start.elapsed());

    let start = Instant::now();
    dec03();
    println!("Day 3 complete in {:?}", start.elapsed());

    let start = Instant::now();
    dec04();
    println!("Day 4 complete in {:?}", start.elapsed());

    dec05();

    dec06();
    dec07();
}
