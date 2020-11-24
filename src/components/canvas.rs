use css_in_rust::Style;
use yew::prelude::*;

pub struct Canvas {
  link: ComponentLink<Self>,
  style: Style,
  props: Props,
}

#[derive(Debug)]
pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: String,
}

impl Component for Canvas {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let style = Style::create("canvas", include_str!("canvas.scss"))
      .expect("An error occured while creating the style.");
    Canvas {
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
