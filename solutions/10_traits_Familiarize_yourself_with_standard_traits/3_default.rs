#[derive(Debug, Default)]
// i32がDefaultを実装しているため、この記載だけでデフォルト値が設定できます。
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Default)]
#[allow(dead_code)]
enum Mode {
    #[default]
    Automatic, // デフォルト値にしたい要素の前に記載をすることで簡単に定義ができます。
    Manual,
}

fn main() {
    let p1 = Point::default();
    println!("{:?}", p1); // Point { x: 0, y: 0 , z:0 }

    let p2 = Point {
        x: 10,
        ..Default::default() // デフォルトを実装することで、フィールドの変更したい部分だけを明示的に書くことで初期化が可能になります。
    };
    println!("{:?}", p2); // Point { x: 10, y: 0 , z:0 }

    let mode = Mode::default();
    println!("{:?}", mode); // Automatic
}
