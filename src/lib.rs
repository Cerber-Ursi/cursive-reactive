use cursive::view::View;
use cursive::Cursive;

pub trait Element {
    type Native: View;
    fn as_native(self) -> Self::Native;
}

pub struct NativeElement<V: View> {
    view: V,
}

impl<V: View> Element for NativeElement<V> {
    type Native = V;
    fn as_native(self) -> Self::Native {
        self.view
    }
}

pub struct ComponentElement<P: PartialEq, C: Component<P>> {
    inner: C,
    props: P,
}

impl<P: PartialEq, C: Component<P>> Element for ComponentElement<P, C> {
    type Native = <<C as Component<P>>::Output as Element>::Native;
    fn as_native(self) -> Self::Native {
        self.inner.render(self.props).as_native()
    }
}

pub trait Component<P: PartialEq> {
    type Output: Element;
    fn render(&self, props: P) -> Self::Output;
}

impl<E: Element, P: PartialEq, F: Fn(P) -> E> Component<P> for F {
    type Output = E;
    fn render(&self, props: P) -> Self::Output {
        self(props)
    }
}

pub fn run(ctx: &mut Cursive, root: impl Element) {
    ctx.add_layer(root.as_native());
    ctx.run();
}
