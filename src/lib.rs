#![feature(array_value_iter)] // projected to be stable in 1.52

use std::{
    array::IntoIter,
    fmt::Debug
};

pub fn test<I, O, E, const N: usize>(f: impl Fn(I)->O, cases: [(I, E); N])
where O: Debug + PartialEq<E>, E: Debug {
    for (input, expected) in IntoIter::new(cases) {
        assert_eq!(f(input), expected);
    }
}
