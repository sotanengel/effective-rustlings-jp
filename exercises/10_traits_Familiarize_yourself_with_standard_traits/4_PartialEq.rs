// TODO: この構造体にPartialEqとEqを実装しましょう。
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

// TODO: valueがNaN同士の比較であればtrueを返すように自分でPartialEqを実装しましょう。
#[derive(Debug)]
struct Measurement {
    value: f64,
    unit: String,
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
