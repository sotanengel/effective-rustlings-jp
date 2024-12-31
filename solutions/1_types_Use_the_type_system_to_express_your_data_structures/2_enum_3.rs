// 信号の状態を表すEnum
#[allow(dead_code)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 車両を表す構造体
struct Vehicle {
    model: String,
    current_light: TrafficLight, // 信号状態をTrafficLightというEnumで定義することで「有効な値の組み合わせ」だけを格納できるようにしよう。
}

// 信号の状態に応じて車両のアクションを決める関数
fn drive(vehicle: &Vehicle) {
    match vehicle.current_light {
        TrafficLight::Red => println!("{} は停止しています。", vehicle.model),
        TrafficLight::Yellow => println!("{} は注意しています。", vehicle.model),
        TrafficLight::Green => println!("{} は進んでいます。", vehicle.model),
    }
}

fn main() {
    // 車両インスタンスを作成し、信号を設定
    let car = Vehicle {
        model: String::from("Toyota Prius"),
        current_light: TrafficLight::Red, // 赤信号
    };

    drive(&car); // 実行すると「Toyota Prius は停止しています。」と表示
}
