use crate::router::AppRoutes;
use yew::prelude::*;
use yew_router::prelude::*;

// 📚 Design System
use design_system::{
  SideNav as Nav, 
  menu::{ Menu, Orientation },
  MenuItem,
  MenuItemGroup,
};

pub struct SideNav {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for SideNav {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _handle: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Nav>
              <Menu orientation=Orientation::Horizontal>

                <MenuItemGroup text="👋 Welcome" class="active">
                  <RouterAnchor<AppRoutes> route=AppRoutes::Introduction>
                    <MenuItem text="Introduction" />
                  </RouterAnchor<AppRoutes>>
                  <RouterAnchor<AppRoutes> route=AppRoutes::Introduction>
                    <MenuItem text="Peter Weinmann" />
                  </RouterAnchor<AppRoutes>>
                  <RouterAnchor<AppRoutes> route=AppRoutes::Introduction>
                    <MenuItem text="Riehcht am Stän" />
                  </RouterAnchor<AppRoutes>>
                </MenuItemGroup>

                <MenuItemGroup icon="atom" text="Atoms" class="active">
                  <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                    <MenuItem text="Color" />
                  </RouterAnchor<AppRoutes>>
                  <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                    <MenuItem text="Typography" />
                  </RouterAnchor<AppRoutes>>
                  <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                    <MenuItem text="Icon" />
                  </RouterAnchor<AppRoutes>>
                  <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                    <MenuItem text="Button" />
                  </RouterAnchor<AppRoutes>>
                </MenuItemGroup>

                <MenuItemGroup icon="molecule" text="Molecules" class="active">
                  <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                    <MenuItem text="Button" />
                  </RouterAnchor<AppRoutes>>
                </MenuItemGroup>

                <MenuItemGroup icon="organism" text="Organisms" class="active">
                  <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                    <MenuItem text="Button" />
                  </RouterAnchor<AppRoutes>>
                </MenuItemGroup>

                <MenuItemGroup icon="template" text="Templates" class="active">
                  <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                    <MenuItem text="Button" />
                  </RouterAnchor<AppRoutes>>
                </MenuItemGroup>
                
                <MenuItemGroup icon="page" text="Pages" class="active">
                  <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                    <MenuItem text="Button" />
                  </RouterAnchor<AppRoutes>>
                </MenuItemGroup>

              </Menu>
            </Nav>
        }
    }
}