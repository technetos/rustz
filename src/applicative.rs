use std::rc::Rc;
use crate::{HKT, functor::Functor};

pub trait Applicative<U>: Functor<U> {
    fn lift(value: U) -> Self::B
    where
        Self: HKT<U, A = U>;

    fn seq<F>(&self, _: <Self as HKT<F>>::B) -> <Self as HKT<U>>::B
    where
        F: Fn(&<Self as HKT<U>>::A) -> U,
        Self: HKT<F>;
}

impl<T, U> Applicative<U> for Option<T> {
    fn lift(value: U) -> Self::B {
        Some(value)
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::B) -> Option<U>
    where
        F: Fn(&T) -> U,
    {
        match *self {
            Some(ref value) => match fs {
                Some(f) => Some(f(value)),
                None => None,
            },
            None => None,
        }
    }
}

impl<T, U> Applicative<U> for Vec<T> {
    fn lift(value: U) -> Self::B {
        vec![value]
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::B) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let mut result = vec![];
        for f in fs.into_iter() {
            for x in self.iter() {
                let v = (f)(x);
                result.push(v);
            }
        }
        return result;
    }
}

impl<T, U> Applicative<U> for Rc<T> {
    fn lift(value: U) -> <Self as HKT<U>>::B {
        Rc::new(value)
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::B) -> Rc<U>
    where
        F: Fn(&T) -> U,
    {
        let v = fs(self);
        Rc::new(v)
    }
}

impl<T, U> Applicative<U> for Box<T> {
    fn lift(value: U) -> <Self as HKT<U>>::B {
        Box::new(value)
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::B) -> Box<U>
    where
        F: Fn(&T) -> U,
    {
        let v = fs(self);
        Box::new(v)
    }
}
