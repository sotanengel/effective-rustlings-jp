fn main() {
    let value: Option<i32> = Some(42);

    // TODO: Noneの場合の処理は必要ないのでifを使った処理に変更してください。
    match value {
        Some(x) => println!("Found: {}", x),
        None => println!("No value found."),
    }
}
