fn main() {
    let value: Option<i32> = Some(42);

    // TODO: expectを使って冗長な処理を簡潔にしてください。
    match value {
        Some(num) => println!("The value is: {}", num),
        None => panic!("Something wrong !"),
    };
}
