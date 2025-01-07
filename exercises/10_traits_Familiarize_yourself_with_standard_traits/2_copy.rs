// TODO: この構造体にCopyを実装しましょう。
#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // p1の値がp2にコピーされる（ムーブではなくコピー）

    // TODO: Point構造体がCopyを実装した時に、以下のコードはどのように書き直すべきでしょうか？
    let p3 = p1.clone();

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p2: {:?}", p3);
}
