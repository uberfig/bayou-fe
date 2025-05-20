use crate::components::{
    communities::CommunitiesBar, login_protect::LoginProtect, unimplimented::NotFinished,
};
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component]
pub fn PersonalContainer() -> impl IntoView {
    view! {
        <nav>
            <CommunitiesBar />
            <p>"above me are the joined communities and below todo (user dms)"</p>
        </nav>
        <main>
            <Outlet/>
        </main>
    }
}

#[component(transparent)]
pub fn PersonalRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/@me") view=PersonalContainer >
        <Route path=path!("") view=|| view! {<LoginProtect view=|| view! { <p>"meow should display the dms on this page but none open, maybe have a pic of a logo/mascot here on pc"</p> } />} />
        // should display user dms with the current one highlighted in nav
        <Route path=path!("/:room_id") view=|| view! {<LoginProtect view=NotFinished />}/>
      </ParentRoute>
    }
    .into_inner()
}
