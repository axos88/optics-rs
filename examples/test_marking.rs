pub trait X {}
struct X1;
struct X2;
impl X for X1 {}
impl X for X2 {}

pub trait Foo {
    type V: X;

    fn foo();
}

struct Foo1;
struct Foo2;

impl Foo for Foo1 {
    type V = X1;

    fn foo() {}
}

impl Foo for Foo2 {
    type V = X2;

    fn foo() {}
}

pub trait Bar {}

pub trait Baz {}

impl Bar for Foo1 {}

impl Baz for Foo2 {}

impl<T> Bar for T where T: Foo<V = X2> {}

fn main() {}
