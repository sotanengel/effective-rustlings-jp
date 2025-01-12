#[derive(Debug)]
#[allow(dead_code)]
struct MyStruct {
    value: String,
}

// TODO: MyStructにFrom<String>トレイトを実装して、main内部でintoが使えるようにしてください。

fn main() {
    let s = String::from("Hello");
    let my_struct: MyStruct = s.into();
    println!("{:?}", my_struct);
}
