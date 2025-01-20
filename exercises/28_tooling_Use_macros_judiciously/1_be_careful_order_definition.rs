fn main() {
    // マクロを呼び出す
    print_hello!();
}

// TODO: 宣言的マクロの定義の位置は配慮する必要があります。
//       適切な場所に配置し直してください。
macro_rules! print_hello {
    () => {
        println!("Hello, world!");
    };
}
