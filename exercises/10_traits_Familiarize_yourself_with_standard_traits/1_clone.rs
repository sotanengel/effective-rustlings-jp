// TODO: Cloneを実装して、main関数が動くようにしましょう。
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

// TODO: この構造体がCloneを実装すべきかどうか考えてみましょう。
#[derive(Debug)]
struct SecretKey {
    key: Vec<u8>,
}

fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
    };

    let user2 = user1;

    println!("{:?}", user1);
    println!("{:?}", user2);

    let secret_key = SecretKey {
        key: vec![1, 2, 3, 4, 5],
    };
    println!("{:?}", secret_key);
}
