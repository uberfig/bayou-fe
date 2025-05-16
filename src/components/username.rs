use leptos::prelude::*;

#[component]
pub fn UsernameEntry(
    username: RwSignal<String>,
    may_only_contain: RwSignal<bool>,
    disabled: RwSignal<bool>,
) -> impl IntoView {
    view! {
        
            <input type="username" id="username" required
                on:input:target=move |ev| {
                    let val: String = ev.target().value();
                    if !val.chars().all(|x| char::is_alphanumeric(x) || x.eq(&'_')) {
                        may_only_contain.set(true);
                        ev.target().set_value(&username.get());
                        return;
                    }
                    may_only_contain.set(false);
                    username.set(ev.target().value().to_lowercase());
                    ev.target().set_value(&username.get());
                }
                prop:value=username
                disabled=move || disabled.get()
            />
    }
}
