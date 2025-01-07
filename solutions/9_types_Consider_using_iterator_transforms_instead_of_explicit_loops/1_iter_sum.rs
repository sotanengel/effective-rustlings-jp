#[allow(clippy::useless_vec)]
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // イテレータを使って合計を計算します。
    let sum: i32 = numbers.iter().sum();

    println!("Sum: {}", sum);
}
