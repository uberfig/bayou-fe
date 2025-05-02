use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| set_count.set(3)
        >
            "Click me: "
            {count} // count is a read signal so we pass it in and it is reactive. if we passed in count.get to be rendered it doesn't work
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}

#[component]
fn ControlledInput() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());

    view! {
        <input type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_name.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn ControlledInput2() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());

    view! {
        <input type="text"

            on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_name.set(ev.target().value());
            }

            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:change:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_name.set("change".to_string());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn ControlledInput3() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());
let email = RwSignal::new("".to_string());
let spam_me = RwSignal::new(true);

view! {
    <input type="text"
        bind:value=(name, set_name)
    />
    <input type="email"
        bind:value=email
    />
    <label>
        "Please send me lots of spam email."
        <input type="checkbox"
            bind:checked=spam_me
        />
    </label>
    <p>"Name is: " {name}</p>
    <p>"Email is: " {email}</p>
    <Show when=move || spam_me.get()>
        <p>"You’ll receive cool bonus content!"</p>
    </Show>
}
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(ControlledInput3);
}
