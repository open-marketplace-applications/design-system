use log::*;
use yew::prelude::*;


use crate::{
    layout::Layout,
    router::{Router},
};
use design_system::{Theme};

pub struct App { }

pub enum Msg {
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg { }
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <Theme>
                <Layout>
                    <Router />
                </Layout>
            </Theme>
        }
    }
}
