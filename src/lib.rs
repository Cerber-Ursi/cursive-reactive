use cursive::{Cursive, view::View};

type Renderer<Props, Out> = Box<dyn Fn(Props) -> Out>;

pub enum Element<Props> {
    Native(Renderer<Props, Box<dyn View>>),
    Reactive(Renderer<Props, Box<dyn Component<Props = ()>>>),
    Fragment(Renderer<Props, Vec<Element<()>>>),
    None,
}

pub trait Component {
    type Props;
    fn render(&self) -> Element<Self::Props>;
}

pub fn render<Props, C: Component<Props = Props> + ?Sized>(comp: &C, props: Props, context: &mut Cursive) {
    mount(comp.render(), props, context);
}

fn mount<Props>(elem: Element<Props>, props: Props, context: &mut Cursive) {
    use Element::*;
    match elem {
        Native(renderer) => context.add_layer(renderer(props)),
        Reactive(renderer) => render(&*renderer(props), (), context),
        Fragment(renderer) => renderer(props).into_iter().for_each(|el| mount(el, (), context)),
        None => {},
    };
}

#[test]
fn test() {
    use cursive::views::TextView;
    struct Test;
    impl Component for Test {
        type Props = ();
        fn render(&self) -> Element<()> {
            Element::Native(Box::new(|_| Box::new(TextView::new("Test"))))
        }
    }
    let backend = cursive::backend::puppet::Backend::init(Some(cursive::vec::Vec2{x: 10, y: 10}));
    let output = backend.stream();
    let mut ctx = Cursive::new(move || backend);
    render(&Test, (), &mut ctx);
    ctx.refresh();
    ctx.step();
    output.recv().unwrap();
    output.recv().unwrap().print_stdout();
}
