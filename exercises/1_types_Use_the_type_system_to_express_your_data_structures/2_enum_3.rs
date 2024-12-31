// 信号の状態を表すEnum
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 車両を表す構造体
// TODO:先ほどまでの問題のTrafficLightを参考にcurrent_lightに適切な型を挿入しなさい。
struct Vehicle {
    model: String,
    current_light: String, // 信号状態を保持するフィールド("Red"か"Yellow"、"Green"が入る)
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
        current_light: "Red", // 赤信号
    };

    drive(&car); // 実行すると「Toyota Prius は停止しています。」と表示
}
