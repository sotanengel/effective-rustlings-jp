fn main() {
    let some_value: Option<i32> = Some(42);

    // 値がある場合だけ処理を行う
    if let Some(x) = some_value {
        println!("Found: {}", x);
    }
}
