#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HNil;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HCons<H, T>(pub H, pub T);

#[macro_export]
macro_rules! hlist {
    [] => { $crate::HNil };
    [$head: expr] => { $crate::HCons($head, $crate::HNil) };
    [$head: expr, $($tail: expr), *] => { $crate::HCons($head, hlist![$($tail), *]) };
    [$head: expr, $($tail: expr), *,] => { $crate::HCons($head, hlist![$($tail), *]) };
}

#[macro_export]
macro_rules! hlist_pat {
    [] => { $crate::HNil{} };
    [$head: pat] => { $crate::HCons($head, $crate::HNil{}) };
    [$head: pat, $($tail: pat), *] => { $crate::HCons($head, hlist_pat![$($tail), *]) };
    [$head: pat, $($tail: pat), *,] => { $crate::HCons($head, hlist_pat![$($tail), *]) };
}

#[macro_export]
macro_rules! HList {
    [] => { $crate::HNil };
    [$head: ty] => { $crate::HCons<$head, $crate::HNil> };
    [$head: ty, $($tail: ty), *] => { $crate::HCons<$head, HList![$($tail), *]> };
    [$head: ty, $($tail: ty), *,] => { $crate::HCons<$head, HList![$($tail), *]> };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macros() {
        let hlist_pat![]: HList![] = hlist![];

        let hlist_pat![a, b, c]: HList![i32, &'static str, bool] = hlist![42, "hello", false];
        let hlist_pat![d, e, f,]: HList![i32, &'static str, bool,] = hlist![64, "goodbye", true];

        assert_eq!(a, 42);
        assert_eq!(b, "hello");
        assert_eq!(c, false);
        assert_eq!(d, 64);
        assert_eq!(e, "goodbye");
        assert_eq!(f, true);
    }
}
