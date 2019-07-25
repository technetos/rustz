use std::rc::Rc;
use crate::HKT;

pub trait Functor<U>: HKT<U> {
    fn fmap<F>(&self, f: F) -> Self::B
    where
        F: Fn(&Self::A) -> U;
}

impl<T, U> Functor<U> for Vec<T> {
    fn fmap<F>(&self, f: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let mut result = Vec::with_capacity(self.len());
        for value in self {
            result.push(f(value));
        }
        result
    }
}

impl<T, U> Functor<U> for Option<T> {
    fn fmap<F>(&self, f: F) -> Option<U>
    where
        F: Fn(&T) -> U,
    {
        match *self {
            Some(ref value) => Some(f(value)),
            None => None,
        }
    }
}

impl<T, U> Functor<U> for Rc<T> {
    fn fmap<F>(&self, f: F) -> Rc<U>
    where
        F: Fn(&T) -> U,
    {
        let v = f(self);
        Rc::new(v)
    }
}

impl<T, U> Functor<U> for Box<T> {
    fn fmap<F>(&self, f: F) -> Box<U>
    where
        F: Fn(&T) -> U,
    {
        let v = f(self);
        Box::new(v)
    }
}
