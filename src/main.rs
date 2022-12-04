use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;
use crate::routes::AppRoute;

mod components
mod pages

use pages::{
    login::Login, home::Home, notification::Notification, pages::Pages, pods::Pods
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

feed,
home,
marketplace,
news-feed
notification
pods
profile
tour