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
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// メイン関数
fn main() {
    let x = 5;
    let y = 7;
    let result = add(x, y);
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
            // ベンチマークしたい処理
            // コンパイラーが最適化でループを削除してしまうため、「std::hint::black_box」という恒等関数を入力とする。
            add(std::hint::black_box(1000), std::hint::black_box(999))
        });
    }
}
