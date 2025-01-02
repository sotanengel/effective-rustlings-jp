fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero is not allowed")
    } else {
        Ok(a / b)
    }
}

fn main() -> Result<(), &'static str> {
    let value = divide(10, 3)?; // クエスションマークでエラーハンドリング
    println!("Result: {}", value);

    Ok(())
}
