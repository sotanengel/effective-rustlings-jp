#[allow(clippy::useless_vec)]
fn main() {
    let numbers = vec![17, 3, 25, 8, 42, 7, 19, 6, 1, 12, 9, 5];

    let odd_numbers: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 != 0 && x < 10)
        .copied()
        .collect();

    println!("{:?}", odd_numbers);
}
