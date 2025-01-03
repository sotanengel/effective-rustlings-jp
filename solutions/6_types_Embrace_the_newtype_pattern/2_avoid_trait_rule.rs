struct MyVec(Vec<i32>);

trait Sum {
    fn sum(&self) -> i32;
}

// 自分で作成したnewTypeのため、Sumを実装することができる。
impl Sum for MyVec {
    fn sum(&self) -> i32 {
        self.0.iter().sum()
    }
}

fn main() {
    let numbers = MyVec(vec![1, 2, 3, 4]);
    println!("Sum: {}", numbers.sum());
}
