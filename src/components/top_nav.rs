use crate::router::AppRoutes;
use yew::prelude::*;
use yew_router::prelude::*;

// 📚 Design System
use design_system::{TopNav as Nav, Logo, Menu, MenuItem, Tooltip};

pub struct TopNav {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for TopNav {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Nav>
              <Menu>
                <RouterAnchor<AppRoutes> route=AppRoutes::RootPath>
                  <Logo />
                </RouterAnchor<AppRoutes>>

                <RouterAnchor<AppRoutes> route=AppRoutes::RootPath>
                  <MenuItem text="HomePage" icon="label" />
                </RouterAnchor<AppRoutes>>

                <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                  <MenuItem text="Button" />
                </RouterAnchor<AppRoutes>>
              </Menu>

            </Nav>
        }
    }
}