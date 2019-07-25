use std::rc::Rc;
use crate::{HKT, applicative::Applicative};

pub trait Monad<U>: Applicative<U> {
    fn flat_map<F>(&self, _: F) -> Self::B
    where
        F: FnMut(&Self::A) -> Self::B;

    fn unit(x: U) -> Self::B
    where
        Self: HKT<U, A = U>,
    {
        Self::lift(x)
    }

    fn join<T>(&self) -> T
    where
        Self: HKT<U, A = T, B = T>,
        T: Clone,
    {
        self.flat_map(|x| x.clone())
    }
}

impl<T, U> Monad<U> for Vec<T> {
    fn flat_map<F>(&self, mut f: F) -> Vec<U>
    where
        F: FnMut(&T) -> Vec<U>,
    {
        let mut result = vec![];
        for x in self {
            let v = f(x);
            result.extend(v);
        }
        result
    }
}

impl<T, U> Monad<U> for Option<T> {
    fn flat_map<F>(&self, mut f: F) -> Option<U>
    where
        F: FnMut(&T) -> Option<U>,
    {
        match *self {
            Some(ref value) => f(value),
            None => None,
        }
    }
}

impl<T, U> Monad<U> for Rc<T> {
    fn flat_map<F>(&self, mut f: F) -> Rc<U>
    where
        F: FnMut(&T) -> Rc<U>,
    {
        f(self)
    }
}

impl<T, U> Monad<U> for Box<T> {
    fn flat_map<F>(&self, mut f: F) -> Box<U>
    where
        F: FnMut(&T) -> Box<U>,
    {
        f(self)
    }
}
