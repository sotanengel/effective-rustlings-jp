#[allow(clippy::useless_vec)]
fn main() {
    let numbers = vec![17, 3, 25, 8, 42, 7, 19, 6, 1, 12, 9, 5];

    let mut odd_numbers = Vec::new();

    // TODO: イテレータを使って簡潔に実装し直してください。
    for i in 0..numbers.len() {
        let x = numbers[i];
        if x % 2 != 0 && x < 10 {
            odd_numbers.push(x);
        }
    }

    println!("{:?}", odd_numbers);
}
