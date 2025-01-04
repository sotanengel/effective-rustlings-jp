fn print_length<T: Into<String>>(value: T) {
    let s: String = value.into(); // トレイト制約を実行する際にはIntoを使う。
    println!("Length: {}", s.len());
}

fn main() {
    let s = "Hello".to_string();
    let str_slice = "Goodbye";

    print_length(s);
    print_length(str_slice);
}
