// 宣言的マクロを定義する
macro_rules! print_value {
    ($x:expr) => {
        println!("The value is: {}", $x);
    };
}

fn main() {
    let x = 42;

    // マクロを呼び出す
    print_value!(x);
}
