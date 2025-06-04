use leptos::{html::Dialog, prelude::*};

#[component]
pub fn BaseModal<View: IntoView + Clone + 'static>(view: View, display: RwSignal<bool>) -> impl IntoView {
    let view = move || view.clone().into_any();
    let node_ref = NodeRef::<Dialog>::new();
    Effect::new(move || {
        let disp = display.get();
        if let Some(dialog) = node_ref.get() {
            match disp {
                true => {
                    let _ = dialog.show_modal();
                },
                false => {
                    let _ = dialog.close();
                },
            };
        }
    });
    view! {
        <dialog node_ref=node_ref
            on:close=move |_| {
                // ensure that display is synced with the actual
                // behavior if it is closed via some other means
                display.set(false);
            }
        >
            {view}
        </dialog>
    }
}
