use yew::prelude::*;
use design_system::{Layout as PageLayout, Page};

use crate::{
    components::{TopNav, SideNav},
};

pub struct Layout {
    props: Props
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Layout {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        
        Layout {
            props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <PageLayout>
                <TopNav />
                <SideNav />
                <Page>
                    { self.props.children.clone() }
                </Page>
            </PageLayout>
        }
    }
}