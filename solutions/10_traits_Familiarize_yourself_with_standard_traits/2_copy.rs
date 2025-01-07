#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // p1の値がp2にコピーされる（ムーブではなくコピー）

    // CopyトレイトはCloneトレイトに制約を持つのでcloneを使うことができるが、
    // ビット単位のコピーを作成する方が方が速いため、そのまま記載した方が良い。
    let p3 = p1;

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p2: {:?}", p3);
}
