use css_in_rust::Style;
use yew::prelude::*;

#[derive(Debug)]
pub struct Menu {
  link: ComponentLink<Self>,
  style: Style,
  props: Props,
}

#[derive(Debug)]
pub enum Msg {}

#[derive(Debug, Clone, PartialEq)]
pub enum Orientation {
  Vertical,
  Horizontal,
}

impl Default for Orientation {
  fn default() -> Self {
    Orientation::Vertical
  }
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: String,
  #[prop_or_default]
  pub orientation: Orientation,
}

impl Component for Menu {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let mut style = Style::create("menu", include_str!("menu.scss"))
      .expect("An error occured while creating the style.");

    Menu {
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
    let mut orientation: String = "".into();
    if self.props.orientation == Orientation::Horizontal {
      orientation = "horizontal".into();
      // style = Classes::from(style.to_string()).extend("self.style".to_string());
    }
    html! {
      <div
        class=Classes::from(
          self.props.class.to_string()
        )
        .extend(self.style.to_string())
        .extend(orientation)
      >
        { self.props.children.clone() }
      </div>
    }
  }
}
