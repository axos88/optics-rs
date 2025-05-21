use crate::{HasSetter, Setter};
use core::marker::PhantomData;


pub struct SetterImpl<S, A, SETTER: Setter<S, A>>(pub SETTER, PhantomData<(S, A)>);

impl<S, A, SETTER: Setter<S, A>> SetterImpl<S, A, SETTER> {
    fn new(l: SETTER) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        SetterImpl(l, PhantomData)
    }
}

impl<S, A, SETTER: Setter<S, A>> From<SETTER> for SetterImpl<S, A, SETTER> {
    fn from(value: SETTER) -> Self {
        Self::new(value)
    }
}

impl<S, A, SETTER: Setter<S, A>> HasSetter<S, A> for SetterImpl<S, A, SETTER> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}
