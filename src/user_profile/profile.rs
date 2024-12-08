use chrono::{Datelike, Month};
use leptos::{component, html::{dd, div, h3}, prelude::*, view, IntoView, Params};
use leptos_router::{hooks::use_params, params::Params};

use crate::{masto_api::{accounts::{webfinger_account, Webfinger}, timelines::{account_timeline, TimelineParams}}, masto_types::account::{Account, Field}, state::State, timeline::{feed::RenderFeed, source::RenderSrc}};

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
    let link = move || Webfinger::webfinger_request(&state.get(), &webfinger());
    let (account, _set_account) = signal(LocalResource::new(move || webfinger_account(link())));

    view! { <AcountWrap account=account.get()/> }
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
    let source = account.clone();
    let account = account.enrich_content();
    let display_name = match &account.display_name.is_empty() {
        true => account.username,
        false => account.display_name,
    };
    let display_name = h3().inner_html(display_name);
    let description = div().inner_html(account.note);
    let mut fields = account.fields;
    let time = chrono::DateTime::from_timestamp_millis(account.created_at).expect("invalid timestamp");
    let time_pretty = format!("{} {}, {}", time.day(), Month::try_from(u8::try_from(time.month()).unwrap()).unwrap().name(), time.year());
    fields.insert(0, Field { name: "joined".to_string(), value: time_pretty, verified_at: None });
    let fields = match fields.is_empty() {
        false => {
            let fields = fields.into_iter().map(|field| {
                let value = dd().inner_html(field.value);
                view! {
                    <div class="field-description" class:verified=field.verified_at.is_some()>
                        <dt>{field.name}</dt>
                        {value}
                    </div>
                }
            }).collect::<Vec<_>>();
            Some(view! {
                <dl>
                    {fields}
                </dl>
            })
        },
        true => None,
    };

    let state: ReadSignal<State> = use_context().expect("missing state");

    view! {
        <img src={ account.header.clone() } class="profile-header" />
        <div class="profile">
            
            <img src={ account.avatar.clone() } class="pfp profile-pfp" />
            <div class="no-decoration">
                <div class="inline">
                    { display_name }
                </div>
                <p class="no-margin">{ format!("@{}", account.acct) }</p>
                {description}
                {fields}
            </div>
            <RenderSrc src=serde_json::to_string_pretty(&source).unwrap() />
        </div>
        <RenderFeed 
            feed=account_timeline(&account.id)
            params=TimelineParams::new(&state.get_untracked()).exclude_replies()
        />
    }
}
