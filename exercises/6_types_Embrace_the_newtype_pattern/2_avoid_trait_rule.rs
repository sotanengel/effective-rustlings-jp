struct MyVec(Vec<i32>);

trait Sum {
    fn sum(&self) -> i32;
}

// TODO: 自分で作成したnewTypeにSumトレイトを実装しましょう。

fn main() {
    let numbers = MyVec(vec![1, 2, 3, 4]);
    println!("Sum: {}", numbers.sum());
}
