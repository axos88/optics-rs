use crate::{HasSetter, Setter};
use core::marker::PhantomData;

pub struct SetterImpl<S, A, L: Setter<S, A>>(pub L, PhantomData<(S, A)>);

impl<S, A, L: Setter<S, A>> SetterImpl<S, A, L> {
    fn new(l: L) -> Self {
        SetterImpl(l, PhantomData)
    }
}

impl<S, A, SETTER: Setter<S, A>> From<SETTER> for SetterImpl<S, A, SETTER> {
    fn from(value: SETTER) -> Self {
        Self::new(value)
    }
}

impl<S, A, L: Setter<S, A>> HasSetter<S, A> for SetterImpl<S, A, L> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}
