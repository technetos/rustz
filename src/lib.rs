use std::rc::Rc;

pub trait HKT<U> {
    type A; // Current type
    type B; // Type with A swapped with U
}

macro_rules! derive_hkt {
    ($t:ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type A = T;
            type B = $t<U>;
        }
    };
}

derive_hkt!(Vec);
derive_hkt!(Option);
derive_hkt!(Box);
derive_hkt!(Rc);

mod functor;
mod applicative;
mod monad;

pub mod prelude {
    pub use crate::{HKT, functor::Functor, applicative::Applicative, monad::Monad};
}
