// TODO: 長さを表すメートルの型を定義しましょう。

// TODO: 重さを表すキログラムの型を定義しましょう。

// f64 から Meters への変換
impl From<f64> for Meters {
    fn from(value: f64) -> Self {
        Meters(value)
    }
}

// f64 から Kilograms への変換
impl From<f64> for Kilograms {
    fn from(value: f64) -> Self {
        Kilograms(value)
    }
}

// メートルをスケーリングする関数
fn scale_meters(m: Meters, factor: f64) -> Meters {
    Meters(m.0 * factor)
}

// キログラムをグラムに変換する関数
fn convert_to_grams(k: Kilograms) -> f64 {
    k.0 * 1000.0
}

fn main() {
    let height = Meters(1.75);
    let weight = Kilograms(68.0);

    let double_height = scale_meters(height, 2.0);
    println!("Double height: {} m", double_height.0);

    let weight_in_grams = convert_to_grams(weight);
    println!("Weight in grams: {} g", weight_in_grams);
}
