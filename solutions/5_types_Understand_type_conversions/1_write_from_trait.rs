#[derive(Debug)]
#[allow(dead_code)]
struct MyStruct {
    value: String,
}

// Fromトレイトを実装することでIntoが自動実行されます。
impl From<String> for MyStruct {
    fn from(s: String) -> Self {
        MyStruct { value: s }
    }
}

fn main() {
    let s = String::from("Hello");
    let my_struct: MyStruct = s.into();
    println!("{:?}", my_struct);
}
