#![feature(test)]
// ↑ Nightly Rust のみで使用できる機能 (#[bench] などのため)
//   Stable Rust ではこの行と benchmark 関連の箇所を削除するか、
//   代わりに Criterion などのベンチマーククレートを利用してください。

extern crate test; // Nightly Rust のベンチマーク用クレート

/// 2つの整数を足し合わせる関数
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// // TODO: add関数を使った結果が5になることをassert_eq!でテストしてください。
///
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// メイン関数
fn main() {
    let x = 5;
    let y = 7;
    // TODO: add関数を使って、xとyを足し合わせた結果をresultに格納してください。
    let result;
    println!("{x} + {y} = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    /// ユニットテスト
    #[test]
    fn test_add() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-5, 5), 0);
    }

    /// ベンチマーク
    /// 実行コマンド (Nightly Rust):
    /// `cargo +nightly bench`
    #[bench]
    fn bench_add(b: &mut Bencher) {
        b.iter(|| {
            // TODO: add関数を以下に記載し、ベンチマークテストを実行してください。
            //       aとbの値はどんな値でも構いません。
        });
    }
}
