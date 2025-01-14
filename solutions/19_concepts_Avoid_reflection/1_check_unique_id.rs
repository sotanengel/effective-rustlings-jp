use std::any::Any;

fn print_type_id<T: Any>(value: &T) {
    let type_id = value.type_id();
    println!("The TypeId of the value is: {:?}", type_id);
}

fn main() {
    let value1 = 42;
    let value2 = "Hello, Rust!";
    let value3 = 2.468;

    print_type_id(&value1);
    print_type_id(&value2);
    print_type_id(&value3);
}
