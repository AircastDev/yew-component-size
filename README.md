# yew-component-size

![Crates.io](https://img.shields.io/crates/l/yew-component-size) ![Crates.io](https://img.shields.io/crates/v/yew-component-size)

A Yew component that emits events when the parent component changes width/height.
Only compatible with Yew using web_sys.

## Example:

```rust
let onsize = self.link.callback(|size: ComponentSize| {
    // Access to `size.width` and `size.height`
});

html! {
    // Parent that you're tracking the size of must be `position: relative`
    <div style="position: relative;">
        // ...
        <ComponentSizeObserver onsize=onsize />
    </div>
}
```

## How it works

This uses a trick borrowed from Svelte where we use an iframe that is positioned absolutely
to fill it's parent element, and then we listen to the resize event of iframe's window.

_**Note:** This incurs a small cost and so should not be used on a large number of elements at the same time._

## License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
