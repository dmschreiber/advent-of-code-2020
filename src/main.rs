mod dec18;

fn main() {
    println!("Hello, world!");

    let my_map = dec18::read_map("/Users/dschreiber/Projects/advent-of-code-2020/src/dec18.txt".to_string());

    dec18::display_map(my_map);
}
