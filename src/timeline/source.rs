use leptos::{component, prelude::*, view, IntoView};

#[component]
pub fn RenderSrc(src: String) -> impl IntoView {
    view! {
        <details class="item-src">
                <summary>{ "source" }</summary>
                <pre>
                {
                    let source = src.split("\n").map(|val| view! {<span>{val.to_string()}</span>}).collect::<Vec<_>>();
                    source
                }
                </pre>
        </details>
    }
}
