#[allow(clippy::useless_vec)]
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let mut sum = 0;
    let mut index = 2; // 最初の2要素をスキップ

    // TODO: イテレータを使って簡潔に実装し直してください。
    while index < numbers.len() {
        sum += numbers[index];
        index += 3; // 3つおきに要素を取得
    }

    println!("Sum of elements at multiples of 3 indices: {}", sum);
}
