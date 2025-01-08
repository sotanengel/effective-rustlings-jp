// MyTrait トレイトの定義
trait MyTrait {
    // 【必須メソッド】
    // トレイトを実装する型は、このメソッドを必ず定義しなければなりません。
    fn required_method(&self);

    // 【デフォルトメソッド】
    // 必須ではありません。必要に応じてオーバーライドできます。
    fn default_method(&self) {
        println!("default_method: これはトレイトで定義されたデフォルト実装です。");
    }

    // 【デフォルトメソッド（戻り値あり）】
    // 必須ではありません。必要に応じてオーバーライドできます。
    fn default_method_with_return(&self) -> i32 {
        println!("default_method_with_return: これはトレイトで定義されたデフォルト実装です。");
        42
    }
}

// トレイトを実装する構造体
struct MyStruct {
    pub name: String,
}

// MyStruct における MyTrait の実装
impl MyTrait for MyStruct {
    // required_method はトレイトで必須とされているので、必ず定義が必要
    fn required_method(&self) {
        println!("required_method: Hello, my name is {}!", self.name);
    }

    // default_method はデフォルトメソッドだが、ここでは独自実装でオーバーライド
    fn default_method(&self) {
        println!(
            "default_method (override): 構造体 {} 独自の実装です。オーバーライドされました。",
            self.name
        );
    }

    // default_method_with_return はデフォルト実装のまま使いたい場合は定義不要。
}

fn main() {
    let my_struct = MyStruct {
        name: String::from("Alice"),
    };

    // 【必須メソッド】呼び出し
    my_struct.required_method();

    // 【デフォルトメソッド（オーバーライド済）】呼び出し
    my_struct.default_method();

    // 【デフォルトメソッド（戻り値あり）】デフォルトの実装を呼び出し
    let result = my_struct.default_method_with_return();
    println!("default_method_with_return の戻り値: {}", result);
}
