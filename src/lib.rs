use cursive::{Cursive, view::View};

type Renderer<Out> = Box<dyn Fn() -> Out>;

pub enum Element {
    Native(Renderer<Box<dyn View>>),
    Reactive(Renderer<Box<dyn Component>>),
    Fragment(Renderer<Vec<Element>>),
    None,
}

pub trait Component {
    fn render(&self) -> Element;
}

pub fn render<C: Component + ?Sized>(comp: &C, context: &mut Cursive) {
    mount(comp.render(), context);
}

fn mount(elem: Element, context: &mut Cursive) {
    use Element::*;
    match elem {
        Native(renderer) => context.add_layer(renderer()),
        Reactive(renderer) => render(&*renderer(), context),
        Fragment(renderer) => renderer().into_iter().for_each(|el| mount(el, context)),
        None => {},
    };
}

#[test]
fn test() {
    use cursive::views::TextView;
    struct Test;
    impl Component for Test {
        fn render(&self) -> Element {
            Element::Native(Box::new(|| Box::new(TextView::new("Test"))))
        }
    }
    let backend = cursive::backend::puppet::Backend::init(Some(cursive::vec::Vec2{x: 10, y: 10}));
    let output = backend.stream();
    let mut ctx = Cursive::new(move || backend);
    render(&Test, &mut ctx);
    ctx.refresh();
    ctx.step();
    output.recv().unwrap();
    output.recv().unwrap().print_stdout();
}
