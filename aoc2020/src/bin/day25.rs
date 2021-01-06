const CARD_PUBLIC_KEY: usize = 7573546;
const DOOR_PUBLIC_KEY: usize = 17786549;

fn main() {
    let mut card_loop_size: Option<usize> = None;
    let mut door_loop_size: Option<usize> = None;
    let mut loop_index = 0;
    let mut key = 1;

    while card_loop_size.is_none() || door_loop_size.is_none() {
        key = key * 7;
        key = key % 20201227;
        loop_index += 1;
        if card_loop_size.is_none() && key == CARD_PUBLIC_KEY {
            card_loop_size = Some(loop_index);
        }
        if door_loop_size.is_none() && key == DOOR_PUBLIC_KEY {
            door_loop_size = Some(loop_index);
        }
    }

    let mut key = 1;
    for _ in 0..door_loop_size.unwrap() {
        key = (key * CARD_PUBLIC_KEY) % 20201227;
    }

    println!("Part 1: {}", key);
}