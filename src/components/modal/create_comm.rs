use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn CreateComm(display: RwSignal<bool>) -> impl IntoView {
    let name = RwSignal::new("".to_string());
    let loading = RwSignal::new(false);
    let create_result: RwSignal<Option<LocalResource<Result<Uuid, ()>>>> =
        RwSignal::new(None);
    let create = move || {

    };

    view! {
        <form class="create_comm" autofocus>
        <p>
            <label for="name">"community name:"</label>
        </p>
        <p>
            <input type="text" id="name"
                bind:value=name
                required
            />
            <button type="submit"
                on:click=move |ev| {
                    ev.prevent_default();
                    create();
                }
                disabled=move || loading.get()
            >
            "Create"
            </button>
            </p>
        </form>
    }
}
