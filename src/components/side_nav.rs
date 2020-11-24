use crate::router::AppRoutes;
use yew::prelude::*;
use yew_router::prelude::*;

// 📚 Design System
use design_system::{
  menu::{Menu, Orientation},
  Logo, MenuItem, MenuItemGroup, SideNav as Nav,
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
          <Logo>{"Design System"}</Logo>

          <Menu orientation=Orientation::Horizontal>

            <MenuItemGroup text="👋 Get Started">
              <RouterAnchor<AppRoutes> route=AppRoutes::Introduction>
                <MenuItem text="Introduction" />
              </RouterAnchor<AppRoutes>>
              <RouterAnchor<AppRoutes> route=AppRoutes::Introduction>
                <MenuItem text="Theming" />
              </RouterAnchor<AppRoutes>>
              <RouterAnchor<AppRoutes> route=AppRoutes::Introduction>
                <MenuItem text="Dark Mode" />
              </RouterAnchor<AppRoutes>>
              <RouterAnchor<AppRoutes> route=AppRoutes::Introduction>
                <MenuItem text="Contribute" />
              </RouterAnchor<AppRoutes>>
            </MenuItemGroup>

            <MenuItemGroup text="💧 Atoms" class="active">
              <MenuItem text="Logo" />
              <MenuItem text="Color" />
              <MenuItem text="Typescale" />
              <MenuItem text="Layout" />
              <MenuItem text="Icon" />
              <RouterAnchor<AppRoutes> route=AppRoutes::ButtonPath>
                <MenuItem text="Button" class="active" />
              </RouterAnchor<AppRoutes>>
              <MenuItem text="Input" />
              <MenuItem text="Select" />
              <MenuItem text="Checkbox" />
              <MenuItem text="Switch" />
              <MenuItem text="MenuItem" />
            </MenuItemGroup>

            <MenuItemGroup text="🌱 Molecules">
              <MenuItem text="Button Group" />
              <MenuItem text="Menu" />
            </MenuItemGroup>

            <MenuItemGroup text="🌳 Organisms">
              <MenuItem text="Form" />
              <MenuItem text="Table" />
              <MenuItem text="SideNav" />
              <MenuItem text="TopNav" />
            </MenuItemGroup>

            <MenuItemGroup text="📑 Templates">
              <MenuItem text="Auth" />
              <MenuItem text="Userland" />
              <MenuItem text="Adminland" />
            </MenuItemGroup>

            <MenuItemGroup text="📚 Pages">
              <MenuItem text="Login" />
              <MenuItem text="Home" />
              <MenuItem text="Product" />
              <MenuItem text="Category" />
              <MenuItem text="Index" />
              <MenuItem text="Create" />
              <MenuItem text="Read" />
              <MenuItem text="Edit" />
              <MenuItem text="Delete" />
            </MenuItemGroup>


          </Menu>
        </Nav>
    }
  }
}
