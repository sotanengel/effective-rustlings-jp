fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let op: fn(i32, i32) -> i32 = add; // 関数ポインタとして利用するためには明示的なfn型に変換が必要
    let fp1 = op;
    let fp2 = op;

    assert!(fp1 == fp2, "{:?} != {:?}", fp1, fp2);
}
