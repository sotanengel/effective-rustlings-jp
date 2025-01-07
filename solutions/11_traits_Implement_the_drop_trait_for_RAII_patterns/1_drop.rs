#[allow(dead_code)]
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
    // 引数として「self」ではなく、「&mut self」を利用するのは、通常メソッドのように呼び出さないためである。
    // 仮に呼び出せてしまうと、以下のようなコードが実行できてしまう。
    // # example
    // x.drop()
    // x.0 *= 100; // 呼び出し後にMyStructの資源が使えてしまう。
    fn drop(&mut self) {
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
