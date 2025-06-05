use crate::some_prism::some_prism;
use optics::{HasSetter, mapped_lens};

mod some_prism {
    use optics::{HasGetter, HasSetter, Prism, PrismImpl};

    struct WasNone;
    struct SomePrism<A>(std::marker::PhantomData<A>);

    impl<A: Clone> HasGetter<Option<A>, A> for SomePrism<A> {
        type GetterError = WasNone;

        fn try_get(&self, source: &Option<A>) -> Result<A, Self::GetterError> {
            if let Some(value) = source {
                Ok(value.clone())
            } else {
                Err(WasNone)
            }
        }
    }

    impl<A> HasSetter<Option<A>, A> for SomePrism<A> {
        fn set(&self, source: &mut Option<A>, value: A) {
            *source = Some(value);
        }
    }

    pub fn some_prism<A: Clone>() -> PrismImpl<Option<A>, A, impl Prism<Option<A>, A>> {
        SomePrism(std::marker::PhantomData).into()
    }
}

fn main() {
    #[allow(dead_code)]
    struct Point {
        x: u32,
        y: u32,
        z: Option<u32>,
    }

    let mut point = Point {
        x: 10,
        y: 20,
        z: Some(30),
    };

    let z_prism =
        mapped_lens(|p: &Point| p.z, |p: &mut Point, z| p.z = z).compose_with_prism(some_prism());

    z_prism.set(&mut point, 42);

    assert_eq!(point.z, Some(42));
}
