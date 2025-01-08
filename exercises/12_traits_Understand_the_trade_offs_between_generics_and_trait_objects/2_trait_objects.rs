// 面積を計算するトレイト
trait Area {
    fn area(&self) -> f64;
}

// 長方形の構造体
struct Rectangle {
    width: f64,
    height: f64,
}

// 円の構造体
struct Circle {
    radius: f64,
}

// トレイトをRectangleに実装
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// トレイトをCircleに実装
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// TODO: Areaのトレイト制約を「トレイトオブジェクト」で追加してください。
fn print_area(shape: ) {
    println!("面積は: {:.2}", shape.area());
}

fn main() {
    let rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let circle = Circle { radius: 7.5 };

    print_area(&rectangle);
    print_area(&circle);
}
