use optics::{HasOver, mapped_lens};

trait HasIncrement<S, A> {
    fn increment(&self, s: &mut S);
}

impl<S, SETTER: HasOver<S, u32>> HasIncrement<S, u32> for SETTER {
    fn increment(&self, source: &mut S) {
        self.over(source, |a| a + 1);
    }
}

fn main() {
    #[allow(dead_code)]
    struct Point {
        x: u32,
        y: u32,
    }

    let mut point = Point { x: 10, y: 20 };
    let x_lens = mapped_lens(|p: &Point| p.x, |p: &mut Point, x| p.x = x);

    x_lens.increment(&mut point);
    x_lens.increment(&mut point);
    assert_eq!(point.x, 12);
}
