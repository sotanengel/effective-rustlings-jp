#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Measurement {
    value: f64,
    unit: String,
}

// PartialEqの実装
impl PartialEq for Measurement {
    fn eq(&self, other: &Self) -> bool {
        (self.value.is_nan() && other.value.is_nan())  // 両方がNaNならtrue
            || (self.value == other.value)             // それ以外は通常の比較
            && (self.unit == other.unit) // 単位も比較
    }
}

fn main() {
    let u1 = User {
        id: 1,
        name: String::from("Alice"),
    };

    let u2 = User {
        id: 1,
        name: String::from("Alice"),
    };

    let u3 = User {
        id: 2,
        name: String::from("Bob"),
    };

    println!("u1 == u2: {}", u1 == u2); // true
    println!("u1 == u3: {}", u1 == u3); // false

    let m1 = Measurement {
        value: f64::NAN,
        unit: String::from("m"),
    };

    let m2 = Measurement {
        value: f64::NAN,
        unit: String::from("m"),
    };

    println!("m1 == m2: {}", m1 == m2); // true
}
