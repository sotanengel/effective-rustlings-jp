use std::any::Any;

// TODO: TにAnyトレイト境界を設けて、グローバルにユニークな識別子(数値)を出力するようにしてください。
fn print_type_id<T>(value: &T) {
    let type_id;
    let type_id = println!("The TypeId of the value is: {:?}", type_id);
}

fn main() {
    let value1 = 42;
    let value2 = "Hello, Rust!";
    let value3 = 3.14;

    print_type_id(&value1);
    print_type_id(&value2);
    print_type_id(&value3);
}
