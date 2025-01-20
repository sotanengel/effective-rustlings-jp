// 宣言的マクロを定義する
macro_rules! print_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    // マクロを呼び出す
    print_hello!();
}
