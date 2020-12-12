use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_component_size::{ComponentSize, ComponentSizeObserver};

struct MyComponent {
    link: yew::ComponentLink<Self>,
    size: ComponentSize,
}

enum Msg {
    OnComponentSize(ComponentSize),
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            size: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::OnComponentSize(size) => {
                self.size = size;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let onsize = self.link.callback(Msg::OnComponentSize);
        html! {
            <div style="position:relative">
                <span>{format!("width: {}px, height: {}px", self.size.width, self.size.height)}</span>
                <ComponentSizeObserver onsize=onsize />
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn start_app() {
    App::<MyComponent>::new().mount_to_body();
}
