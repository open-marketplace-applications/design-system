use log::*;
use yew::prelude::*;

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
            <div>
               <h1>{ "OMA Design System" }</h1>
            </div>
        }
    }
}
