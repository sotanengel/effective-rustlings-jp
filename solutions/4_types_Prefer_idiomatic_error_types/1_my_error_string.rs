use std::{error::Error, fmt};

// 独自エラー型
#[derive(Debug)]
struct MyError {
    message: String,
    code: u32,
}

// Displayトレイトを実装してエラーメッセージを整える
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Error Code: {}] {}", self.code, self.message)
    }
}

// error::Errorトレイトを実装
impl Error for MyError {}

// From<String> を実装して String からの変換を簡単に
impl From<String> for MyError {
    fn from(message: String) -> Self {
        MyError {
            message,
            code: 1000, // デフォルトのエラーコード
        }
    }
}

// &str からの変換もできるようにする
impl From<&str> for MyError {
    fn from(message: &str) -> Self {
        MyError {
            message: message.to_string(),
            code: 1000, // デフォルトのエラーコード
        }
    }
}

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
