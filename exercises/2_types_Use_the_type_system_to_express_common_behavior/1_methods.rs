enum Item {
    Apple(u32), // 価格
    Meat(u32),  // 価格
}

impl Item {
    // TODO: Itemの価格を返却するメソッドを書いてください。
    fn price(&self) -> u32 {}

    // TODO: Itemの価格を変更するメソッドを書いてください。
    fn increase_price(&mut self, amount: u32) {}

    // TODO: 「Consumed {apple or meat} worth {} yen」という出力をするメソッドを書いてください。
    fn consume(self) {}
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
