fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // TODO: fp1とfp2に関数ポインタを適用してください。
    let fp1 = add;
    let fp2 = add;

    assert!(fp1 == fp2, "{:?} != {:?}", fp1, fp2);
}
