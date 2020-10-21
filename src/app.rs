use log::*;
use yew::prelude::*;


use crate::{
    components::Nav,
    router::{AppRoutes, Router},
};
use design_system::{Theme, Page, Container, H1};

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg { }
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <Theme>
            <Nav />
                <Page>
                    <Container>
                    <Router />
                    </Container>
                </Page>
            </Theme>
        }
    }
}
