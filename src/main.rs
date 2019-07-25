use rustz::prelude::*;
use std::rc::Rc;

trait New<T> {
    fn new(_: T) -> Self;
}

impl<T> New<T> for Box<T> {
    fn new(v: T) -> Box<T> {
        Box::new(v)
    }
}
impl<T> New<T> for Rc<T> {
    fn new(v: T) -> Rc<T> {
        Rc::new(v)
    }
}

struct Foo<T, P>
where
    P: HKT<T>,
{
    ptr: P::B,
}

impl<T, P> Foo<T, P>
where
    P: HKT<T, A = ()>,
    P::B: New<T>,
{
    fn new(v: T) -> Self {
        let p: P::B = P::B::new(v);
        Foo { ptr: p }
    }
}

fn main() {
    let v = Vec::unit(1);
    let v = v.flat_map(|x| vec![x.to_string(); 3]);
    println!("{:?}", v);

    let o = Option::unit(1);
    let o = o.flat_map(|&x| Some(x + 1));
    println!("{:?}", o);

    let o = Option::lift(1);
    let o = o.flat_map(|&x| Some(x + 1));
    println!("{:?}", o);

    let rc = Rc::unit(7);
    let rc = rc.flat_map(|&x| Rc::new(x * 3));
    println!("{:?}", rc);

    let b = Box::unit(7);
    let b = b.flat_map(|&x| Box::new(x * 4));
    println!("{:?}", b);

    let o = Some(Some(true));
    let o = o.join();
    println!("{:?}", o);

    let v = vec![vec![true], vec![false]];
    let v = v.join();
    println!("{:?}", v);

    let f1: &Fn(&i32) -> i32 = &|x| x * 3;
    let f = Some(f1);
    let o = Some(3);
    let o = o.seq(f);
    println!("{:?}", o);

    let f1: &Fn(&&str) -> String = &|x| x.to_uppercase();
    let f2: &Fn(&String) -> String = &|x| x.chars().filter(|c| c == &'H').collect::<String>();
    let o = vec!["hi", "how", "are", "you"];
    let o = o.fmap(f1);
    let o = o.fmap(f2);
    println!("{:?}", o);

    let f2: &Fn(&i32) -> i32 = &|x| x * 3;
    let f3: &Fn(&i32) -> i32 = &|x| x + 1;
    let f = vec![f2, f3];
    let o = vec![1, 2, 3, 4, 5];
    let o = o.seq(f);
    println!("{:?}", o);

    let f: Foo<_, Box<_>> = Foo::new(5);
    let p: Box<_> = f.ptr;
    println!("{:?}", p);

    let f: Foo<_, Rc<_>> = Foo::new("5".to_string());
    let p: Rc<_> = f.ptr;
    println!("{:?}", p);
}
