use std::error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum AppError {
    NotFound(io::Error),
    InvalidInput(FromUtf8Error),
    InternalError(Box<dyn error::Error>),
}

// Displayトレイトを実装してエラーメッセージを整える
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(e) => write!(f, "Not Found: {}", e),
            AppError::InvalidInput(e) => write!(f, "Invalid Input: {}", e),
            AppError::InternalError(e) => write!(f, "Internal Error: {}", e),
        }
    }
}

// Errorトレイトを実装
impl error::Error for AppError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            AppError::NotFound(e) => Some(e),
            AppError::InvalidInput(e) => Some(e),
            AppError::InternalError(e) => Some(e.as_ref()),
        }
    }
}

// From<String> を実装して String からの変換を簡単に
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::NotFound(err)
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(err: FromUtf8Error) -> Self {
        AppError::InvalidInput(err)
    }
}

impl From<String> for AppError {
    fn from(message: String) -> Self {
        AppError::InternalError(Box::new(io::Error::new(io::ErrorKind::Other, message)))
    }
}

// サンプル関数
fn do_something(input: &str) -> Result<(), AppError> {
    if input.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Input cannot be empty",
        ))?
    } else if input == "404" {
        Err(AppError::from("Requested resource not found".to_string()))?
    } else if input == "500" {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Unexpected error occurred",
        ))?
    }
    Ok(())
}

// プロセス関数をResultでラップ
fn process() -> Result<(), AppError> {
    let inputs = vec!["", "404", "success"];

    for input in inputs {
        do_something(input)?; // エラーがあれば即座にreturn
        println!("Processed: {}", input);
    }

    Ok(())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_do_something_not_found() {
        let result = do_something("");
        match result {
            Err(AppError::NotFound(e)) => {
                assert_eq!(e.kind(), io::ErrorKind::NotFound);
                assert_eq!(e.to_string(), "Input cannot be empty");
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_do_something_internal_error() {
        let result = do_something("404");
        match result {
            Err(AppError::InternalError(e)) => {
                assert!(e.to_string().contains("Requested resource not found"));
            }
            _ => panic!("Expected InternalError"),
        }
    }

    #[test]
    fn test_do_something_success() {
        let result = do_something("success");
        assert!(result.is_ok());
    }

    #[test]
    fn test_process() {
        let result = process();
        match result {
            Err(AppError::NotFound(e)) => {
                assert_eq!(e.to_string(), "Input cannot be empty");
            }
            Err(AppError::InternalError(e)) => {
                assert!(e.to_string().contains("Requested resource not found"));
            }
            Ok(_) => {} // 成功ケースは特にアサートしない
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::Other, "io error");
        let app_error: AppError = io_err.into();
        match app_error {
            AppError::NotFound(e) => {
                assert_eq!(e.to_string(), "io error");
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_from_utf8_error() {
        let utf8_err = String::from_utf8(vec![0, 159]).unwrap_err();
        let app_error: AppError = AppError::from(utf8_err);
        match app_error {
            AppError::InvalidInput(e) => {
                assert!(e.to_string().contains("invalid utf-8"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_from_string() {
        let app_error: AppError = String::from("custom error").into();
        match app_error {
            AppError::InternalError(e) => {
                assert_eq!(e.to_string(), "custom error");
            }
            _ => panic!("Expected InternalError"),
        }
    }
}
