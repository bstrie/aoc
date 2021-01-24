use std::fmt::Debug;

pub fn test<I, O>(f: impl Fn(I)->O, cases: &[(I, O)])
where I: Copy, O: Copy + Debug + PartialEq<O> {
    for (i, o) in cases {
        assert_eq!(f(*i), *o);
    }
}
