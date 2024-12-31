#[allow(dead_code)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
    Blinking,
}

// 信号の状態に応じてアクションを決定する関数
fn signal_action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("停止してください"), // 赤信号では停止
        TrafficLight::Yellow => println!("注意してください"), // 黄色信号では注意
        TrafficLight::Green => println!("進んでください"), // 緑信号では進行
        TrafficLight::Blinking => println!("点滅中 - 注意して進んでください"), // 点滅信号の処理を追加
    }
}

fn main() {
    let light = TrafficLight::Yellow;
    signal_action(light); // 黄色信号をテスト
}
