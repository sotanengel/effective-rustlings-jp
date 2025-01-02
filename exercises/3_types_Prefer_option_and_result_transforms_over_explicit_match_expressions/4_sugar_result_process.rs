fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero is not allowed")
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10, 3); // TODO: クエスションマークを使ってさらに簡潔に表現しましょう。

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => panic!("Result: {}", e),
    }
}
