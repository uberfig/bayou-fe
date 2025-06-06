use crate::{
    components::login_protect::LoginProtect,
    routes::{login::Login, room_routes::RoomRoutes, signup::Signup},
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
                <Route path=path!("/") view=|| view! {<LoginProtect view=|| view! {<Redirect path="/rooms/@home"/>} />}/>
                <Route path=path!("/login") view=Login/>
                <Route path=path!("/signup") view=Signup/>
                <RoomRoutes />
            </Routes>
        </Router>
    }
}