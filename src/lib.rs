use std::fmt::Debug;

pub trait Test<I, O> {
    fn test(self, cases: &[(I, O)]);
}

impl<I, O, F> Test<I, O> for F
where I: Copy, O: Copy + Debug + PartialEq<O>, F: Fn(I)->O {
    fn test(self, cases: &[(I, O)]) {
        for (i, o) in cases {
            assert_eq!(self(*i), *o);
        }
    }
}
