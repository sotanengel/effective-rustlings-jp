#[derive(Debug, Clone)]
#[allow(dead_code)]
struct User {
    name: String,
    age: u32,
}

// 秘密鍵を保持する構造体
// 意図しない場所でコピーを作成して、カギの流出を防ぐためにCloneトレイトは実装しない方が良い
#[derive(Debug)]
#[allow(dead_code)]
struct SecretKey {
    key: Vec<u8>,
}

fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
    };

    let user2 = user1.clone();

    println!("{:?}", user1);
    println!("{:?}", user2);

    let secret_key = SecretKey {
        key: vec![1, 2, 3, 4, 5],
    };
    println!("{:?}", secret_key);
}
