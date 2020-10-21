use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{prelude::*, route::Route, router::Router as YewRouter, switch::Permissive, Switch};

use crate::page::{
    ButtonPage, HomePage
};

#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    #[to = "/!"]
    RootPath,
    #[to = "/button!"]
    ButtonPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

pub struct Router {
    link: ComponentLink<Self>,
}

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { link: _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <YewRouter<AppRoutes>
                render = YewRouter::render(move |switch: AppRoutes | {
                    match switch {
                        AppRoutes::RootPath => html!{<HomePage/>},
                        AppRoutes::ButtonPath => html!{<ButtonPage/>},
                        AppRoutes::PageNotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoutes::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                    }
                } )
                redirect = YewRouter::redirect(|route: Route<()>| {
                    AppRoutes::PageNotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}
