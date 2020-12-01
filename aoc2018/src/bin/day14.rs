const INPUT: usize = 306281;

fn main() {
    let mut elf1 = 0;
    let mut elf2 = 1;
    let mut recipes = vec![3, 7];
    let mut running = 0;
    let mut part2 = None;

    while recipes.len() < (INPUT + 10) || part2.is_none() {
        for d in digits(recipes[elf1] + recipes[elf2]) {
            recipes.push(d);
            running = ((running * 10) % 1000000) + d;
            if running == INPUT && part2.is_none() {
                part2 = Some(recipes.len() - 6);
            }
        }

        elf1 = (elf1 + recipes[elf1] + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] + 1) % recipes.len();
    }

    let part1 = recipes.into_iter().skip(INPUT).take(10).fold(0, |s, x| (s * 10) + x);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2.unwrap());
}

fn digits(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![0];
    }
    let mut ret = Vec::new();
    let mut n = n;

    while n > 0 {
        ret.push(n % 10);
        n = n / 10;
    }

    ret.reverse();
    ret
}
