use std::fmt::Debug;

pub fn test<I, O, E, const N: usize>(f: impl Fn(I)->O, cases: [(I, E); N])
where O: Debug + PartialEq<E>, E: Debug {
    for (input, expected) in cases {
        assert_eq!(f(input), expected);
    }
}
