use std::error;
use std::fmt;

// 独自エラー型の定義 (タプル構造体)
#[derive(Debug)]
pub struct OriginalError(String);

// Displayトレイトを実装してエラーメッセージを整える
impl fmt::Display for OriginalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

// error::Errorトレイトを実装
impl error::Error for OriginalError {}

// From<String> を実装して String からの変換を簡単に
impl From<String> for OriginalError {
    fn from(message: String) -> Self {
        OriginalError(message)
    }
}

// &str からの変換もできるようにする
impl From<&str> for OriginalError {
    fn from(message: &str) -> Self {
        OriginalError(message.to_string())
    }
}

fn do_something(flag: bool) -> Result<(), OriginalError> {
    if flag {
        Ok(())
    } else {
        Err(OriginalError::from("Something went wrong"))
    }
}

fn main() {
    match do_something(false) {
        Ok(_) => println!("Success!"),
        Err(e) => {
            let error_message = format!("Failed: {}", e);
            println!("{}", error_message);
        }
    }
}
