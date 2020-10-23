use css_in_rust::Style;
use yew::prelude::*;

// 📚 Design System
use crate::Icon;

#[derive(Debug)]
pub struct SideNav {
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

impl Component for SideNav {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let style = Style::create("side_nav", include_str!("side_nav.scss")).expect("An error occured while creating the style.");
    
    SideNav {
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
        { self.props.children.clone() }
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