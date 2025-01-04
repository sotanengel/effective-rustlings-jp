#[allow(clippy::useless_vec)]
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // TODO: イテレータを使って簡潔に実装し直してください。
    let mut sum = 0;
    for i in 0..numbers.len() {
        sum += numbers[i];
    }

    println!("Sum: {}", sum);
}
