enum Apple {
    Price(u32),
}

enum Meat {
    Price(u32),
}

// AppleとMeatを引数にとる関数
fn print_price(apple: Apple, meat: Meat) {
    match apple {
        Apple::Price(price) => println!("Apple price: {} yen", price),
    }
    match meat {
        Meat::Price(price) => println!("Meat price: {} yen", price),
    }
}

fn main() {
    let apple = Apple::Price(300);
    let meat = Meat::Price(1200);

    print_price(meat, apple); // TODO: Enumを使うことであるエラーをコンパイル時に特定できます。
}
