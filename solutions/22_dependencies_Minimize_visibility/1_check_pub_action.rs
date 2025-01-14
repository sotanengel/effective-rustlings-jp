mod my_mod {
    // パブリックな構造体
    pub struct MyStruct {
        pub public_value: i32, // パブリックなフィールド
        private_value: i32,    // 非公開のフィールド
    }

    impl MyStruct {
        // パブリックなコンストラクタ
        pub fn new(pub_val: i32, priv_val: i32) -> Self {
            MyStruct {
                public_value: pub_val,
                private_value: priv_val,
            }
        }

        // 非公開のメソッド
        fn private_method(&self) {
            println!(
                "private_method called: public_value={}, private_value={}",
                self.public_value, self.private_value
            );
        }

        // パブリックなメソッド
        pub fn public_method(&self) {
            println!("public_method called: public_value={}", self.public_value);
            self.private_method();
        }

        // 非公開フィールドにアクセスするためのパブリックメソッド
        pub fn get_private_value(&self) -> i32 {
            self.private_value
        }
    }

    // パブリックなenum
    #[allow(dead_code)]
    pub enum MyEnum {
        A,
        B,
    }

    // パブリックなトレイト
    pub trait MyTrait {
        fn trait_method(&self);
    }

    // トレイトを実装する構造体もpub
    pub struct Implementer;

    impl MyTrait for Implementer {
        fn trait_method(&self) {
            println!("Implementer trait_method");
        }
    }
}

fn main() {
    // トレイトをスコープに導入する
    use my_mod::MyTrait;

    // MyStructの利用
    let s = my_mod::MyStruct::new(42, 7);
    println!("Public Value: {}", s.public_value);
    println!(
        "Accessed Private Value through getter: {}",
        s.get_private_value()
    );
    s.public_method();

    // MyEnumの利用
    let e = my_mod::MyEnum::A;
    match e {
        my_mod::MyEnum::A => println!("Matched variant A"),
        my_mod::MyEnum::B => println!("Matched variant B"),
    }

    // MyTraitの利用
    let impl_obj = my_mod::Implementer;
    // traitをスコープに入れているので、trait_methodが使用可能
    impl_obj.trait_method();
}
