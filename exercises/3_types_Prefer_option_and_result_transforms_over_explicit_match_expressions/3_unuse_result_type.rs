fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero is not allowed")
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10, 0);

    // TODO: 返却されたResult型は使用する必要があります。
    // matchを使って、計算結果を表示してください。
    println!("result is Error");
}
