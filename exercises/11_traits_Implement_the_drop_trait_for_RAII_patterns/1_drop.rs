struct MyStruct(i32);

trait MyTrait {
    fn do_something(&self);
}

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something");
    }
}

impl Drop for MyStruct {
    // TODO: dropの引数が間違っています。
    // 引数部分を修正して、drop実行後にはMyStructの資源にアクセスできないようにしてください。
    fn drop(&self) {
        println!("MyStruct is being dropped!");
        // 以降にドロップ処理を書く
    }
}

fn main() {
    let x = MyStruct(57);
    x.do_something();

    // ここで明示的にdropを実行する
    drop(x);
}
