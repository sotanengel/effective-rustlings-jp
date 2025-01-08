// 面積を計算する基本トレイト
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

// AreaトレイトをRectangleに実装
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// AreaトレイトをCircleに実装
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// TODO: トレイトオブジェクトとしてAreaを追加してください。
fn print_area(shape: Area) {
    println!("面積は: {:.2}", shape.area());
}

fn main() {
    let rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let circle = Circle { radius: 7.5 };

    // 異なる型の参照をVecに格納
    let shapes: Vec<&dyn Area> = vec![&rectangle, &circle];

    // ループで面積を計算
    for shape in &shapes {
        print_area(*shape);
    }
}
