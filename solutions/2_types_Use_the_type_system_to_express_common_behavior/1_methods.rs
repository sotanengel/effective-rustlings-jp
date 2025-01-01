enum Item {
    Apple(u32), // 価格
    Meat(u32),  // 価格
}

impl Item {
    // 参照を取るメソッド(データの構造を取得するが、変更はしないことがシグネチャから分かる)
    fn price(&self) -> u32 {
        match self {
            Item::Apple(price) => *price,
            Item::Meat(price) => *price,
        }
    }

    // 可変参照を取るメソッド(データを変更する可能性がある)
    fn increase_price(&mut self, amount: u32) {
        match self {
            Item::Apple(price) => *price += amount,
            Item::Meat(price) => *price += amount,
        }
    }

    // 所有権を消費するメソッド(データを消費する)
    fn consume(self) {
        match self {
            Item::Apple(price) => println!("Consumed apple worth {} yen", price),
            Item::Meat(price) => println!("Consumed meat worth {} yen", price),
        }
    }
}

fn main() {
    let mut apple = Item::Apple(300);
    let meat = Item::Meat(1200);

    // 参照メソッドの呼び出し
    println!("Apple price: {} yen", apple.price());

    // 可変参照メソッドの呼び出し
    apple.increase_price(100);
    println!("Increased apple price: {} yen", apple.price());

    // 所有権を消費するメソッドの呼び出し
    meat.consume();
}
