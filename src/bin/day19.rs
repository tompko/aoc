use std::cmp::max;

const INPUT: usize = 3018458;

fn main() {
    let l = INPUT - (INPUT.next_power_of_two() / 2);
    let part1 = 2 * l + 1;

    let finput = INPUT as f64;
    let t = finput.log(3.0) as u32;
    let t = 3usize.pow(t);
    let part2 = if INPUT == t { INPUT } else { max(INPUT - t, 2*INPUT - 3*t) };

    println!("1: {}", part1);
    println!("2: {}", part2);
}
