use css_in_rust::Style;
use yew::prelude::*;

// 📚 Design System
use crate::Icon;

#[derive(Debug)]
pub struct MenuItemGroup {
    link: ComponentLink<Self>,
    style: Style,
    props: Props,
}

#[derive(Debug)]
pub enum Msg {}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: String,
  #[prop_or_default]
  pub text: String,
  #[prop_or_default]
  pub icon: &'static str,
}

impl Component for MenuItemGroup {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let style = Style::create("menu_item_group", include_str!("menu_item_group.scss")).expect("An error occured while creating the style.");
    
    Self {
      link,
      style,
      props: props.to_owned(),
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {

    html! {
      <div
          class=Classes::from(self.props.class.to_string()).extend(self.style.to_string())
        >
        <div class="header">
          {render_icon(self.props.icon.clone())}
          {render_text(self.props.text.clone())}
        </div>
        <div class="options">
          { self.props.children.clone() }
        </div>
      </div>
    }
  }
}

pub fn render_text(text: String) -> Html {
  if text.is_empty() {
    html! {
      <></>
    }
  } else {
    html! {
      <span>{ text }</span>
    }
  }
}

pub fn render_icon(name: &'static str) -> Html {
  if name.is_empty() {
    html! {
      <></>
    }
  } else {
    html! {
      <Icon name=name />
    }
  }
}