// TODO: マクロの中で使える変数xを定義してください。
macro_rules! print_value {
    {
        println!("The value is: {}", $x);
    };
}

fn main() {
    let x = 42;

    // マクロを呼び出す
    print_value!(x);
}
