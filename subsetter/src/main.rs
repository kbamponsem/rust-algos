// Explain in details what is going on in this entire Rust program.

// This is a macro that implements the SubSetter trait for all types that implement the
// SubSetter trait. This is a macro because we want to implement the trait for all types
// that implement the trait, and we can't do that with a normal impl block.

// The macro is called impl_subsets, and it takes a list of types as arguments. The macro
// expands to a bunch of impl blocks, one for each type. The macro is defined below the
// main function.

// The SubSetter trait is defined below the main function. It has one associated type, T,
// and one method, subsets. The subsets method returns a vector of hash sets of T. The
// subsets method is implemented for all types that implement the SubSetter trait.

// The main function prints the subsets of the set {1, 2, 3, 4} and the subsets of the set
// {"a", "b", "c"}. The subsets method is called on the sets, and the result is printed.

// The assert_same function is defined below the main function. It takes two vectors of
// hash sets of T, and asserts that they are the same. The assert_same function is called
// in the tests module.

// The set macro is defined below the assert_same function. It takes a list of elements as
// arguments, and returns a hash set of those elements. The set macro is called in the
// tests module.

// The union macro is defined below the set macro. It takes two hash sets as arguments, and
// returns the union of the two hash sets. The union macro is called in the tests module.

// The tests module is defined below the union macro. It contains two tests. The first test
// asserts that the set macro expands to the same thing as a manually constructed hash set.
// The second test asserts that the subsets method returns the correct subsets for a few
// different sets.

// The macro is defined below the tests module. It takes a list of types as arguments. The
// macro expands to a bunch of impl blocks, one for each type. The macro is called in the
// main function.

use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

pub trait SubSetter {
    type T: Sized;
    fn subsets(&self) -> Vec<HashSet<Self::T>>;
}

macro_rules! impl_subsets {
    ($($typ:ty $(,)?)*) => {
        $(impl SubSetter for HashSet<$typ> {
            type T = $typ;
            fn subsets(&self) -> Vec<HashSet<Self::T>> {
                let mut out = vec![];

                out.push(set!({}));

                for el in self {
                    let mut list = vec![];
                    for v in out.iter() {
                        list.push(union!(v.clone(), set!({ el })));
                    }
                    out.append(&mut list);
                }
                out
            }
        })*
    };
}

// Implement subsets for all types
impl_subsets! {i32, u32, String}

fn main() {
    println!("{:?}", set!({1, 2, 3, 4}).subsets());
    println!(
        "{:?}",
        set!({String::from("a"), String::from("b"), String::from("c")}).subsets()
    );
}

fn assert_same<T: Sized + Eq + PartialOrd + Clone + Ord>(a: Vec<HashSet<T>>, b: Vec<HashSet<T>>) {
    let transform = |a: Vec<HashSet<T>>| -> Vec<T> {
        let mut a = a
            .into_iter()
            .map(|v| v.into_iter().collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        a.sort();
        a
    };

    if transform(a) == transform(b) {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[macro_export]
macro_rules! set {
    ($({$($a:ident$(,)?)*})?) => {{
        let mut inner = HashSet::new();
        $(
            $(
                inner.insert($a.clone());
            )?
        )*
        inner
    }};

    ($({$($a:expr$(,)?)*})?) => {{
        let mut inner = HashSet::new();
        $(
            $(

                inner.insert($a);
            )?
        )*
        inner
    }};
}

#[macro_export]
macro_rules! union {
    ($a:expr, $b:expr) => {{
        let mut out = HashSet::new();
        for v in $a {
            out.insert(v);
        }
        for v in $b {
            out.insert(v);
        }
        out
    }};
}

#[cfg(test)]
mod tests {
    use crate::{assert_same, SubSetter};
    use std::collections::HashSet;

    #[test]
    fn test_set_macro() {
        assert_eq!(set!({1, 2, 3}), {
            let mut set = HashSet::new();
            set.insert(1);
            set.insert(2);
            set.insert(3);

            set
        });
    }

    #[test]
    fn test_subsetter() {
        assert_same(set!({ 1 }).subsets(), vec![set!(), set!({ 1 })]);
        assert_same(
            set!({1, 2}).subsets(),
            vec![set!(), set!({ 1 }), set!({ 2 }), set!({1, 2})],
        );
        assert_same(
            set!({1, 2, 3}).subsets(),
            vec![
                set!({1, 2, 3}),
                set!({1,2}),
                set!({2, 3}),
                set!({1,3}),
                set!({ 1 }),
                set!({ 2 }),
                set!({ 3 }),
                set!(),
            ],
        );
    }
}
