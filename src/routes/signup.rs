use leptos::prelude::*;

use crate::{
    api::{
        methods::auth::signup::signup,
        types::{
            signup_result::SignupResult, signup_user::SignupUser
        },
    },
    components::username::UsernameEntry,
    state::State,
};

async fn signup_tasks(
    request: SignupUser,
    state: State,
    loading: RwSignal<bool>,
) -> Result<(), SignupResult> {
    let result = signup(&state, &request).await;
    loading.set(false);
    result
}

#[component]
pub fn Signup() -> impl IntoView {
    let username = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());
    let uname_pass_incorrect = RwSignal::new(false);
    let may_only_contain = RwSignal::new(false);

    let loading = RwSignal::new(false);
    let signup_result: RwSignal<Option<LocalResource<Result<(), SignupResult>>>> =
        RwSignal::new(None);

    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    view! {
        <main>
            <form>
                <p>
                    <label for="username">"username:"</label>
                    <UsernameEntry
                        username=username
                        may_only_contain=may_only_contain
                        disabled=loading
                    />
                </p>
                <p>
                <label for="password">"password:"</label>
                    <input type="password" id="password"
                        bind:value=password
                        disabled=move || loading.get()
                        required
                    />
                
                </p>
                <p>
                    <button type="submit"
                        on:click=move |ev| {
                            ev.prevent_default();
                            loading.set(true);
                            signup_result.set(Some(LocalResource::new(move || {
                                let request = SignupUser {
                                    username: username.get_untracked(),
                                    password: password.get_untracked(),
                                    email: None,
                                    token: None,
                                    application_message: None,
                                };
                                signup_tasks(request, state.get_untracked(), loading)
                            })));

                        }
                        disabled=move || loading.get()
                    >
                    "Login"
                    </button>
                </p>
            </form>
            <Show when=move || uname_pass_incorrect.get()>
                <p>"username or password is incorrect"</p>
            </Show>
            <Show when=move || may_only_contain.get()>
                <p>"username may only contain numbers, letters, and underscores"</p>
            </Show>
            <Show when=move || loading.get()>
                <p>"loading..."</p>
            </Show>

            <Show when=move || signup_result.get().is_some()>
                <p>
                    {move || {
                        if let Some(val) = signup_result.get() {
                            if let Some(val) = val.get() {
                                let msg = val.err().unwrap_or(SignupResult::Success);
                                return msg.to_string();
                            }
                        }
                        "an unexpected error has occured".to_string()
                    }}
                </p>
            </Show>
        </main>
    }
}
