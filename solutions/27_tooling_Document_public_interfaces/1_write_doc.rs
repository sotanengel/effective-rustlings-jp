/// # 概要
///
/// 文字列を整数にパースし、指定したメッセージとともに出力する関数です。
/// このサンプルコードでは、パースに失敗した場合に [`panic!`] マクロで
/// 強制終了する例を示しています。
///
/// # サンプルコード
///
/// ```
/// // この関数は例示のために作成したモジュールに含まれていますが、
/// // 実際には適宜修正してご利用ください。
/// //
/// // example モジュールを使うことを想定したサンプル
/// use example::parse_and_print;
///
/// // 整数文字列をパースし、出力する
/// let result = parse_and_print("42", "解析成功");
/// // 正しくパースできた場合は Ok(42) が返る
/// assert_eq!(result, Ok(42));
///
/// // パースに失敗した場合は panic! となります
/// // let result_fail = parse_and_print("abc", "解析失敗"); // panic!
/// ```
///
/// # パニックについて
///
/// - 入力文字列を [`i32::parse()`] でパースできない場合、
///   [`panic!`] で強制終了します。  
/// - [`panic!`](https://doc.rust-lang.org/std/macro.panic.html) マクロは
///   リカバリ不能なエラーを表すために使われるもので、アプリケーション全体を終了させます。
///
/// # `unsafe` の使用について
///
/// この関数では、内部で一部 [`unsafe`](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) ブロックを使用しています。
/// Rust では [`unsafe`] コードはメモリ安全性をコンパイラに保証させないため、
/// 十分に注意して取り扱う必要があります。
/// 以下のように一時的にポインタを変換する処理を含んでいますが、
/// 安全性については利用者自身が確認する責任を負います。
///
/// ```
/// // unsafeブロックの例
/// unsafe {
///     let num_ref: *const i32 = &10;
///     let num_mut_ref = num_ref as *mut i32;
///     // ポインタ操作等、通常の安全なRustでは禁止される操作が可能
/// }
/// ```
///
/// # パラメータ
///
/// * `input` - 整数へ変換したい文字列
/// * `message` - ログ出力や表示に使用するメッセージ
///
/// # 戻り値
///
/// * `Ok(i32)` - `input` のパースに成功した場合  
/// * `Err(std::num::ParseIntError)` - `input` のパースに失敗した場合
///
/// # 関連項目
///
/// - [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
/// - [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
/// - [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
/// - [`panic!`](https://doc.rust-lang.org/std/macro.panic.html)
/// - [`i32::parse()`](https://doc.rust-lang.org/std/primitive.i32.html#method.parse)
///
/// # 実装例
///
pub fn parse_and_print(input: &str, message: &str) -> Result<i32, std::num::ParseIntError> {
    // 文字列を整数にパースできるかチェック
    let value: i32 = match input.parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            // パースに失敗したら panic! する
            panic!("Failed to parse the input string: {:?}", e);
        }
    };

    // unsafeブロックの例示
    #[allow(unused_unsafe)]
    unsafe {
        let ptr: *const i32 = &value;
        let _mut_ptr = ptr as *mut i32;
        // ここで任意の不正な操作が可能なため、十分に注意が必要
    }

    // パースに成功したらメッセージと共に表示
    println!("{}: {}", message, value);

    // 結果を返す
    Ok(value)
}

use std::env;

fn main() {
    // コマンドライン引数をベクタに収集
    let args: Vec<String> = env::args().collect();

    // 第1引数があれば使用し、なければ "42" をデフォルト値とする
    let input_str = args.get(1).map(|s| s.as_str()).unwrap_or("42");
    // 第2引数があれば使用し、なければ "解析結果" をデフォルト値とする
    let message = args.get(2).map(|s| s.as_str()).unwrap_or("解析結果");

    // parse_and_print を呼び出し、その戻り値を表示
    match parse_and_print(input_str, message) {
        Ok(n) => println!("parse_and_print の戻り値: {}", n),
        Err(e) => println!("エラーが発生しました: {}", e),
    };
}
