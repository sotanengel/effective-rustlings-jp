// 面積を計算する基本トレイト
trait Area {
    fn area(&self) -> f64;
}

// 面積を拡張するトレイト（Areaを制約として持つ）
trait Shape: Area {
    fn double_area(&self) -> f64 {
        self.area() * 2.0
    }
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

// AreaトレイトをRectangleに実装
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// ShapeトレイトをRectangleに実装
impl Shape for Rectangle {}

// AreaトレイトをCircleに実装
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// ShapeトレイトをCircleに実装
impl Shape for Circle {}

// Shapeを受け取る関数
fn print_double_area<T: Shape>(shape: &T) {
    println!("2倍の面積は: {:.2}", shape.double_area());
}

fn main() {
    let rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let circle = Circle { radius: 7.5 };

    print_double_area(&rectangle);
    print_double_area(&circle);
}
