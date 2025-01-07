// TODO: この構造体にDefaultを実装しましょう。
#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// TODO: このenumにDefaultを実装しましょう。
#[derive(Debug)]
#[allow(dead_code)]
enum Mode {
    Automatic, // TODO: デフォルト値はこの要素にしてください。
    Manual,
}

fn main() {
    let p1 = Point::default();
    println!("{:?}", p1);

    // TODO: xには10を代入して、それ以外はデフォルト値を入れましょう。
    let p2 = Point {};
    println!("{:?}", p2);

    let mode = Mode::default();
    println!("{:?}", mode);
}
