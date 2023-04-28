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
