use std::{error::Error, fmt};

// エラーメッセージとエラーコードを保持する独自エラー型です。
#[derive(Debug)]
struct MyError {
    message: String,
    code: u32,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Error Code: {}] {}", self.code, self.message)
    }
}

// TODO: error::Errorトレイトを実装してください。

// TODO: From<String> を実装して String からの変換を簡単にしましょう。

// TODO: &str からの変換もできるようにしましょう。

fn do_something(flag: bool) -> Result<(), MyError> {
    if flag {
        Ok(())
    } else {
        Err(MyError::from("Something went wrong"))
    }
}

fn main() {
    match do_something(false) {
        Ok(_) => println!("Success!"),
        Err(e) => {
            eprintln!("Error: {}", e); // 独自エラー型の出力
            if let Some(source) = e.source() {
                eprintln!("Caused by: {}", source);
            }
        }
    }

    let error_from_string: MyError = "An error occurred".to_string().into();
    println!("Generated error: {}", error_from_string);

    let error_from_str: MyError = "Another error occurred".into();
    println!("Generated error: {}", error_from_str);
}
