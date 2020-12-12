#![deny(missing_docs)]

//! ![Crates.io](https://img.shields.io/crates/l/yew-component-size) ![Crates.io](https://img.shields.io/crates/v/yew-component-size)
//!
//! A Yew component that emits events when the parent component changes width/height.
//! Only compatible with Yew using web_sys.
//!
//! # Example:
//! ```rust
//! let onsize = self.link.callback(|size: ComponentSize| {
//!     // Access to `size.width` and `size.height`
//! });
//!
//! html! {
//!     // Parent that you're tracking the size of must be `position: relative`
//!     <div style="position: relative;">
//!         // ...
//!         <ComponentSizeObserver onsize=onsize />
//!     </div>   
//! }
//! ```
//!
//! # How it works
//!
//! This uses a trick borrowed from Svelte where we use an iframe that is positioned absolutely
//! to fill it's parent element, and then we listen to the resize event of iframe's window.
//!
//! _**Note:** This incurs a small cost and so should not be used on a large number of elements at the same time._
//!
//! # License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! # Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlIFrameElement;
use yew::{html, Callback, Component, NodeRef, Properties};

const IFRAME_STYLE: &str = "display: block; position: absolute; top: 0; left: 0; width: 100%; height: 100%; overflow: hidden; border: 0; opacity: 0; pointer-events: none; z-index: -1;";

/// Yew component to observe changes to the size of the parent element.
///
/// See the crate documentation for an example and more information.
#[derive(Debug)]
pub struct ComponentSizeObserver {
    props: Props,
    iframe_ref: NodeRef,
    on_resize: Option<Closure<dyn Fn()>>,
}

/// ComponentSizeObserver properties
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    /// A callback that is fired when the component size changes for any reason.
    pub onsize: Callback<ComponentSize>,
}

/// A struct containing the width and height of the component
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComponentSize {
    /// Width of the component in pixels
    pub width: f64,

    /// Height of the component in pixels
    pub height: f64,
}

impl Component for ComponentSizeObserver {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        Self {
            props,
            iframe_ref: Default::default(),
            on_resize: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if self.props != props {
            self.props = props;
            self.add_resize_listener();
            false
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        html! {
            <iframe style=IFRAME_STYLE ref=self.iframe_ref.clone() />
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.add_resize_listener();
        }
    }
}

impl ComponentSizeObserver {
    fn add_resize_listener(&mut self) {
        let iframe = self.iframe_ref.cast::<HtmlIFrameElement>().unwrap();
        let window = iframe.content_window().unwrap();

        let iframe_ref = self.iframe_ref.clone();
        let size_callback = self.props.onsize.clone();
        let on_resize = Closure::wrap(Box::new(move || {
            let iframe = iframe_ref.cast::<HtmlIFrameElement>().unwrap();
            let bcr = iframe.get_bounding_client_rect();
            size_callback.emit(ComponentSize {
                width: bcr.width(),
                height: bcr.height(),
            });
        }) as Box<dyn Fn()>);
        window.set_onresize(Some(on_resize.as_ref().unchecked_ref()));
        self.on_resize = Some(on_resize);
    }
}
