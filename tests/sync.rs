use shared_vec::ArcVec;
use std::thread::scope;

#[test]
fn arc_vec() {
    let v = ArcVec::from_boxed_slice(Box::new([1, 2, 3, 4, 5]));
    assert_eq!(v.len(), 5);

    scope(|s| {
        s.spawn(|| {
            let v2 = v.clone();
            assert_eq!(v2.len(), 5);
            assert_eq!(&*v2, &[1, 2, 3, 4, 5]);
        });
        s.spawn(|| {
            let v3 = v.get(1..4).unwrap();
            assert_eq!(v3.len(), 3);
            assert_eq!(&*v3, &[2, 3, 4]);
        });
        s.spawn(|| {
            let v4 = v.idx(0..2);
            assert_eq!(v4.len(), 2);
            assert_eq!(&*v4, &[1, 2]);
        });
    });
}
