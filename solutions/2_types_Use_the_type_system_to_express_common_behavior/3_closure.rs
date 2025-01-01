fn main() {
    let a = 10;
    let c = 5;
    let divide_and_add = |b: i32| if b != 0 { a / b + c } else { c }; // クロージャーで定義することで、環境にある変数(a, c)を使った関数にすることができる。

    println!("Divide: {}", divide_and_add(1));
    println!("Divide: {}", divide_and_add(0));
}
