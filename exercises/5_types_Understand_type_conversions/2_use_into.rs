fn print_length<T: Into<String>>(value: T) {
    // TODO: intoを使って、valueを変換した値を変数sに格納してください。
    println!("Length: {}", s.len());
}

fn main() {
    let s = "Hello".to_string();
    let str_slice = "Goodbye";

    print_length(s);
    print_length(str_slice);
}
