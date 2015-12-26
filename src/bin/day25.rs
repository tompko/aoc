use std::collections::HashMap;

fn main() {
    let mut paper: HashMap<(i32, i32), i64> = HashMap::new();
    paper.insert((1, 1), 20151125);
    let mut prev = 20151125;

    let mut idx = 3;
    loop {
        let mut x = 1;
        while x < idx {
            let coord = (x, idx - x);

            let next = (prev * 252533) % 33554393;

            paper.insert(coord, next);
            prev = next;
            x += 1;
        }
        idx += 1;

        if paper.contains_key(&(2978, 3083)) {
            break;
        }
    }

    println!("{}", paper[&(3083, 2978)]);
    println!("{}", paper[&(6, 5)]);
    // for y in 1..7 {
    //     let mut line = String::new();
    //     for x in 1..7 {
    //         line = line + &paper[&(x, y)].to_string() + "\t";
    //     }
    //     println!("{}", line);
    // }
}
