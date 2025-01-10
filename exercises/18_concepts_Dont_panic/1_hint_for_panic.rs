use std::env;
use std::num::ParseIntError;

// `CRITICAL_ENV` がセットされているか、確認する。
// TODO: このコードはpanic!とResultどちらがいいでしょうか？
fn get_critical_env() -> {
    match env::var("PATH") {
        Ok(value) => value,
        Err(_) => ,
    }
}

// 文字列を整数に変換する関数
// TODO: このコードはpanic!とResultどちらがいいでしょうか？
fn parse_str_to_int(input: &str) ->  {
    input.parse::<i32>()
}

fn main() {
    let critical_value = get_critical_env();
    println!("PATH is set to: {}", critical_value);


    let inputs = vec!["42", "100", "not_number", "256"];
    // TODO: 以下でparse_str_to_intでintに変更して出力してください。
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

    // CRITICAL_ENV がセットされていない場合は panic! することを確認
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
