pub fn last<T>(v: &Vec<T>) -> &T {
    let lastidx: usize = v.len() - 1;
    &v[lastidx]
}

#[test]
fn last_returns_last_item() {
    let v: Vec<i32> = vec![2,4,5,8];
    assert_eq!(*last(&v), 8);
}
