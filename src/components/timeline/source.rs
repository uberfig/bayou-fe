use leptos::{component, prelude::*, view, IntoView};

#[component]
pub fn RenderSrc(src: String, render: bool) -> impl IntoView {
    match render {
    true => view! {
            <details class="item-src">
                    <summary>{ "source" }</summary>
                    <pre>
                    {
                        let source = src.split("\n").map(|val| view! {<span>{val.to_string()}</span>}).collect::<Vec<_>>();
                        source
                    }
                    </pre>
            </details>
        }.into_any(),
    false => view! {}.into_any(),
    }
}
