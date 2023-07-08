#![allow(unused_variables)]

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "chrono")]
pub use self::chrono::*;

use core::fmt::Display;
use leptos::html::Td;
use leptos::*;
use std::str::FromStr;

/// The default cell renderer. Uses the `<td>` element.
#[component]
pub fn DefaultTableCellRenderer<T, C>(
    cx: Scope,
    /// The class attribute for the cell element. Generated by the classes provider.
    #[prop(into)]
    class: MaybeSignal<String>,
    /// The value to display.
    #[prop(into)]
    value: MaybeSignal<T>,
    /// The index of the column. Starts at 0.
    index: usize,
    /// Called when the content of the cell has changed.
    on_change: C,
    /// Set this to true to be able to edit the content of the cell.
    editable: bool,
) -> impl IntoView
where
    T: IntoView + FromStr + Clone + 'static,
    C: Fn(T) + 'static,
    <T as FromStr>::Err: std::fmt::Debug,
{
    if editable {
        let td_ref = create_node_ref::<Td>(cx);
        let on_input = move |_| {
            if let Some(td) = td_ref.get_untracked() {
                let value = td.inner_text();
                if let Ok(v) = T::from_str(&value) {
                    on_change(v);
                }
            }
        };

        return view! { cx,
            <td class=class _ref=td_ref on:input=on_input contenteditable>{value}</td>
        };
    }

    view! { cx,
        <td class=class>{value}</td>
    }
}

/// The default number cell renderer. Uses the `<td>` element.
#[component]
pub fn DefaultNumberTableCellRenderer<T, C>(
    cx: Scope,
    /// The class attribute for the cell element. Generated by the classes provider.
    #[prop(into)]
    class: MaybeSignal<String>,
    /// The value to display.
    #[prop(into)]
    value: MaybeSignal<T>,
    /// The index of the column. Starts at 0.
    index: usize,
    /// The number of digits to display after the decimal point. Provided by the `#[table(format(precision=X))]` attribute of the field.
    #[prop(optional)]
    precision: Option<usize>,
    /// Called when the content of the cell has changed.
    on_change: C,
    /// Set this to true to be able to edit the content of the cell.
    editable: bool,
) -> impl IntoView
where
    T: Display + FromStr + Clone + 'static,
    C: Fn(T) + 'static,
{
    let text = create_memo(cx, move |_| match precision {
        Some(precision) => format!("{:.precision$}", value()),
        None => format!("{}", value()),
    });

    if editable {
        let td_ref = create_node_ref::<Td>(cx);

        let on_input = move |_| {
            if let Some(td) = td_ref.get_untracked() {
                let value = td.inner_text();
                if let Ok(v) = T::from_str(&value) {
                    on_change(v);
                }
            }
        };

        return view! { cx,
            <td class=class _ref=td_ref on:input=on_input contenteditable>{text}</td>
        };
    }

    view! { cx,
        <td class=class>{text}</td>
    }
}
