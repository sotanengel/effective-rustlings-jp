fn main() {
    let a = 10;
    let c = 5;

    // TODO: クロージャーとしてdivide_and_addを導入することでコンパイルエラーを解消してください。
    fn divide_and_add(b: i32) -> i32 {
        if b != 0 {
            a / b + c
        } else {
            c
        }
    }

    println!("Divide: {}", divide_and_add(1));
    println!("Divide: {}", divide_and_add(0));
}
