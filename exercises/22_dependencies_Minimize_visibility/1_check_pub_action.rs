mod my_mod {
    // TODO: この構造体のpublic_valueにだけアクセスできるようにしてください。
    struct MyStruct {
        public_value: i32,
        private_value: i32,
    }

    // TODO: newとpublic_method、get_private_valueをアクセスできるようにしてください。
    impl MyStruct {
        fn new(pub_val: i32, priv_val: i32) -> Self {
            MyStruct {
                public_value: pub_val,
                private_value: priv_val,
            }
        }

        fn private_method(&self) {
            println!(
                "private_method called: public_value={}, private_value={}",
                self.public_value, self.private_value
            );
        }

        fn public_method(&self) {
            println!("public_method called: public_value={}", self.public_value);
            self.private_method();
        }

        fn get_private_value(&self) -> i32 {
            self.private_value
        }
    }

    // TODO: MyEnumにアクセスできるようにしてください。
    #[allow(dead_code)]
    enum MyEnum {
        A,
        B,
    }

    // TODO: MyTraitを実装したメソッド全てにアクセスできるようにしてください。
    trait MyTrait {
        fn trait_method(&self);
    }

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
