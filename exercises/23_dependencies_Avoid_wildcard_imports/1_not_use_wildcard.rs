mod module_a {
    pub trait MyTrait {
        fn foo(&self);
    }
}

mod module_b {
    #[allow(dead_code)]
    pub trait MyTrait {
        fn bar(&self);
    }

    pub trait AnotherTrait {
        fn baz(&self);
    }
}

// TODO: ワイルドカードを使ってしまったため、トレイト名が衝突してしまっています。
//       明示的にインポートをすることで問題を解消してください。
use module_a::*;
use module_b::*;

struct MyStruct;

impl MyTrait for MyStruct {
    fn foo(&self) {
        println!("Foo from module_a");
    }
}

// module_b の AnotherTrait を MyStruct に対して実装
impl AnotherTrait for MyStruct {
    fn baz(&self) {
        println!("Baz from module_b");
    }
}

fn main() {
    let s = MyStruct;
    s.foo();
    s.baz();
}
