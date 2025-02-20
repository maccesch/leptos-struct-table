use crate::BodyRef;
use leptos::prelude::*;

/// Default tbody renderer. Please note that this is **NOT** a `#[component]`.
///
/// # Arguments
///
/// * `content` - The content of the renderer. It's like the children of this view.
/// * `class` - The class attribute that is passed to the root element
/// * `node_ref` - The `NodeRef` referencing the root tbody element.
///
/// This render function has to render exactly one root element.
#[allow(non_snake_case)]
pub fn DefaultTableBodyRenderer(
    content: impl IntoView,
    class: Signal<String>,
    body_ref: BodyRef,
) -> impl IntoView {
    view! {
        <tbody class=class use:body_ref>
            {content}
        </tbody>
    }
}
