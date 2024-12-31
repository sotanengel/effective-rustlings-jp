enum TrafficLight {
    Red,
    Yellow,
    Green,
    Blinking,
}

fn signal_action(light: TrafficLight) {
    match light {
        // TODO: コンパイルエラーを解消してください。
        TrafficLight::Red => println!("停止してください"),
        TrafficLight::Green => println!("進んでください"),
    }
}

fn main() {
    let light = TrafficLight::Yellow;
    signal_action(light);
}
