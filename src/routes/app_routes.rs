use crate::{
    components::login_protect::LoginProtect,
    routes::{login::Login, auth_routes::AuthRoutes, signup::Signup},
};
use leptos::prelude::*;
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    path,
};

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=|| view! {<LoginProtect view=|| view! {<Redirect path="/app/@home"/>} />}/>
                <Route path=path!("/login") view=Login/>
                <Route path=path!("/signup") view=Signup/>
                <AuthRoutes />
            </Routes>
        </Router>
    }
}
