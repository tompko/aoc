fn main() {
    let target = 36000000;
    let limit = 1000000;
    let mut presents = vec![0; limit];

    for i in 1..limit {
        let mut j = i;
        while j < limit {
            presents[j] += i*10;
            j += i;
        }
    }
    println!("{}", presents.into_iter().position(|p| p > target).unwrap());

    presents = vec![0; limit];

    for i in 1..limit {
        let mut j = i;
        let mut c = 0;
        while j < limit && c < 50 {
            presents[j] += i * 11;
            j += i; c += 1;
        }
    }
    println!("{}", presents.into_iter().position(|p| p > target).unwrap());
}
