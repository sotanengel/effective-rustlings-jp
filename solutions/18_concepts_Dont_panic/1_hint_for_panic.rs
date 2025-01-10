use std::env;
use std::num::ParseIntError;

// これは「必須の環境変数」が無いとプログラムを継続できないケースを想定した関数。
// もし `PATH` がセットされていなければ、回復不能なエラーとみなして panic! する。
fn get_critical_env() -> String {
    match env::var("PATH") {
        Ok(value) => value,
        Err(_) => panic!("PATH environment variable is not set! Aborting..."),
    }
}

// -------------------------------------------------------
// Result を使ったエラー処理の例（回復可能なエラー）
// -------------------------------------------------------

// 文字列を整数に変換する関数
// 変換が失敗した場合は ParseIntError を返し、呼び出し元で対処できるようにする
fn parse_str_to_int(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn main() {
    println!("=== めったにないが必須条件が満たされていない場合の panic! ===");

    // 本来であれば CRITICAL_ENV が必ず設定されている前提だが、
    // もし設定が漏れていたらプログラムを継続できないので panic! する。
    let critical_value = get_critical_env();

    println!("PATH is set to: {}", critical_value);

    // 以下は通常の処理(例: サーバー起動など)を続ける
    println!("The program continues with the critical environment value...");

    println!("\n=== Result を使うケース ===");
    let inputs = vec!["42", "100", "not_number", "256"];
    for s in inputs {
        match parse_str_to_int(s) {
            Ok(num) => println!("'{}' -> {}", s, num),
            Err(e) => eprintln!("'{}' は整数に変換できませんでした: {}", s, e),
        }
    }
}

// -------------------------------------------------------
// テストコード
// -------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // PATH が正しくセットされている場合のテスト
    //
    // 注意: 環境変数はプロセス全体で共有されるため、テスト並列実行の設定によっては
    // 衝突が起こる可能性があります。実際のプロダクションコードでは
    // テスト時に専用のプロセスを立ち上げるか、シリアル実行するなどの工夫が必要です。
    #[test]
    fn test_get_critical_env_ok() {
        // テスト用に環境変数をセット
        env::set_var("PATH", "test_value");
        let value = get_critical_env();
        assert_eq!(value, "test_value");
    }

    // PATH がセットされていない場合は panic! することを確認
    #[test]
    #[should_panic(expected = "PATH environment variable is not set! Aborting...")]
    fn test_get_critical_env_panic() {
        // 一旦アンセット
        env::remove_var("PATH");
        // ここで必ず panic! するはず
        let _ = get_critical_env();
    }

    // -------------------------------------------------------
    // parse_str_to_int のテスト (Result の確認)
    // -------------------------------------------------------

    #[test]
    fn parse_str_to_int_success() {
        let result = parse_str_to_int("123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 123);
    }

    #[test]
    fn parse_str_to_int_error() {
        let result = parse_str_to_int("not_a_number");
        assert!(result.is_err());
        // エラーの型が ParseIntError であることを確認
        match result {
            Err(e) => {
                // ここではあえてエラーメッセージまでは比較しないが、
                // メッセージを比較する場合は e.to_string() 等を用いる
                assert!(e.to_string().contains("invalid digit"));
            }
            _ => unreachable!(),
        }
    }
}
