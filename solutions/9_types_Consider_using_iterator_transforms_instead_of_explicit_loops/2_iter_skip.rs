#[allow(clippy::useless_vec)]
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let sum: i32 = numbers
        .iter()
        .skip(2) // 最初の2要素をスキップ
        .step_by(3) // 3つおきに要素を取得
        .sum();

    println!("Sum of elements at multiples of 3 indices: {}", sum);
}
