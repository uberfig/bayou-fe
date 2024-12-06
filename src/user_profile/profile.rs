use leptos::{component, html::{div, h1, h3}, prelude::*, view, IntoView, Params};
use leptos_router::{hooks::use_params, params::Params};

use crate::{masto_api::accounts::{webfinger_account, Webfinger}, masto_types::account::Account, state::State};

#[derive(Params, PartialEq, Clone)]
struct ProfileParams {
    webfinger: Option<String>,
}

#[component]
pub fn Profile() -> impl IntoView {
    let params = use_params::<ProfileParams>();

    let webfinger = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.webfinger.clone())
            .unwrap_or("missing param".to_string())
    };

    let state: ReadSignal<State> = use_context().expect("missing state");
    let link = Webfinger::webfinger_request(&state.get(), &webfinger());
    let account = LocalResource::new(move || webfinger_account(link.clone()));

    view! { <AcountWrap account=account/> }
}

#[component]
pub fn AcountWrap(account: LocalResource<Option<Account>>) -> impl IntoView {
    view! {
        {move || match account.get() {
            None => view! { <p>"Loading..."</p> }.into_any(),
            Some(account) => match account.into_taken() {
                None => view! { <p>"Loading..."</p> }.into_any(),
                Some(account) => view! { <Account account=account/> }.into_any(),
            }
        }}
    }
}

#[component]
pub fn Account(account: Account) -> impl IntoView {
    let account = account.parse_emoji();
    let display_name = match &account.display_name.is_empty() {
        true => account.username,
        false => account.display_name,
    };
    let display_name = h3().inner_html(display_name);
    let description = div().inner_html(account.note);

    view! {
        <img src={ account.avatar.clone() } class="pfp profile-pfp" />
        <div class="no-decoration">
            <div class="inline">
                { display_name }
            </div>
            <p class="no-margin">{ format!("@{}", account.acct) }</p>
            {description}
        </div>
    }
}
