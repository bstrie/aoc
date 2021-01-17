use std::fmt::Debug;

pub fn test<O>(f: fn(&str)->O, input: &[(&str, O)])
    where O: Debug + PartialEq<O> {
    for (i, o) in input {
        assert_eq!(f(i), *o);
    }
}
