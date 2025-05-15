use crate::opaque::{A, B};
use optics::Lens;

mod opaque {
    use optics::{Lens, mapped_lens};

    #[derive(Debug, Clone)]
    pub struct A {
        email: String,
    }

    #[derive(Debug, Clone)]
    pub struct Person {
        email: String,
    }

    #[derive(Debug, Clone)]
    pub struct B {
        person: Person,
    }

    impl A {
        pub fn email_lens() -> impl Lens<A, String> {
            mapped_lens(|a: &A| a.email.clone(), |a, email| a.email = email)
        }
    }

    impl B {
        pub fn email_lens() -> impl Lens<B, String> {
            mapped_lens(
                |b: &B| b.person.email.clone(),
                |b, email| b.person.email = email,
            )
        }
    }

    impl Default for A {
        fn default() -> Self {
            A {
                email: "FOO@EXAMPLE.COM".to_string(),
            }
        }
    }

    impl Default for B {
        fn default() -> Self {
            B {
                person: Person {
                    email: "BAR@EXAMPLE.COM".to_string(),
                },
            }
        }
    }
}

// Can perform the operation without direct access to fields of A
fn lens_lowercase_email(p: &mut A, lens: impl Lens<A, String>) {
    lens.set(p, lens.get(p).to_lowercase())
}

// Can perform the operation even without any knowledge about the structure of T
fn lens_generic_lowercase_email<T>(t: &mut T, lens: impl Lens<T, String>) {
    lens.set(t, lens.get(t).to_lowercase())
}

// We can't access the private fields of A or B here, we can only get hold of a lens.
fn main() {
    let mut b = B::default();
    //b.person.email = b.person.email.to_lowercase(); // Field `person` in struct `opaque::B` is private

    let mut a = A::default();
    println!("A::default={:?}", a);
    assert_eq!(A::email_lens().get(&a), "FOO@EXAMPLE.COM");

    lens_lowercase_email(&mut a, A::email_lens());
    assert_eq!(A::email_lens().get(&a), "foo@example.com");
    println!("A with lowercased email {:?}", a);

    println!("B::default{:?}", b);
    assert_eq!(B::email_lens().get(&b), "BAR@EXAMPLE.COM");

    lens_generic_lowercase_email(&mut b, B::email_lens());
    println!("B with lowercase email {:?}", b);
    assert_eq!(B::email_lens().get(&b), "bar@example.com");
}
