#[allow(clippy::unnecessary_literal_unwrap)]
fn main() {
    let value: Option<i32> = Some(42);

    let result = value.expect("Something wrong !");
    println!("The value is: {}", result);
}
