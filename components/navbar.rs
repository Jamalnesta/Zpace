use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;
use crate::routes::AppRoute;

#[function_component(App)]
fn nav() -> Html {
    html! {
        <div class="navbarContainer">
            <div class="navbarLeft"> Left </div>
            <div class="navbarCenter"> Center </div>
            <div class="navbarRight"> Right </div>
        </div>
    }
}

fn nav() {
    yew::Renderer::<App>::new().render();
}