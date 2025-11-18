use alloc::boxed::Box;

#[test]
fn rc_vec() {
    use crate::RcVec;

    let v = RcVec::from_boxed_slice(Box::new([1, 2, 3, 4, 5]));
    assert_eq!(v.len(), 5);

    let v2 = v.clone();
    assert_eq!(v2.len(), 5);
    assert_eq!(&*v2, &[1, 2, 3, 4, 5]);

    let v3 = v.get(1..4).unwrap();
    assert_eq!(v3.len(), 3);
    assert_eq!(&*v3, &[2, 3, 4]);

    let v4 = v.idx(0..2);
    assert_eq!(v4.len(), 2);
    assert_eq!(&*v4, &[1, 2]);
}

#[test]
fn arc_vec() {
    use crate::ArcVec;

    let v = ArcVec::from_boxed_slice(Box::new([1, 2, 3, 4, 5]));
    assert_eq!(v.len(), 5);

    let v2 = v.clone();
    assert_eq!(v2.len(), 5);
    assert_eq!(&*v2, &[1, 2, 3, 4, 5]);

    let v3 = v.get(1..4).unwrap();
    assert_eq!(v3.len(), 3);
    assert_eq!(&*v3, &[2, 3, 4]);

    let v4 = v.idx(0..2);
    assert_eq!(v4.len(), 2);
    assert_eq!(&*v4, &[1, 2]);
}
