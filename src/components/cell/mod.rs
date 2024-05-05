#![allow(unused_variables)]

use crate::{CellValue, RenderOptions};

use leptos::*;

/// The default cell renderer. Uses the `<td>` element.
#[component]
pub fn DefaultTableCellRenderer<T, F>(
    /// The class attribute for the cell element. Generated by the classes provider.
    class: String,
    /// The value to display.
    #[prop(into)]
    value: MaybeSignal<T>,
    /// Event handler called when the cell is changed. In this default renderer this will never happen.
    on_change: F,
    /// The index of the column. Starts at 0.
    index: usize,
    options: RenderOptions,
) -> impl IntoView
where
    T: CellValue + Clone + 'static,
    F: Fn(T) + 'static,
{
    view! {
        <td class=class>{value.get().render_value(&options)}</td>
    }
}