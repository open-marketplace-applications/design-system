use web_sys::Element;
use yew::prelude::*;
use yew::NodeRef;

use crate::components::Canvas;
use design_system::{Button, Section};
use pulldown_cmark::{html::push_html, Options, Parser};

pub struct ButtonPage {
    content: String,
    node_ref: NodeRef,
}

impl Component for ButtonPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        // let val = "# Hello world!";
        let val = include_str!("../../crate/design-system/src/atoms/button/README.md");
        let parser = Parser::new_ext(&val, options);
        let mut parsed_text = String::new();
        push_html(&mut parsed_text, parser);

        ButtonPage {
            content: parsed_text,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&self.content);
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Section class="transparent">
                    <h1>{"Button"}</h1>
                </Section>
                <Section class="transparent">
                    <h2>{"🖥 Canvas"}</h2>
                    <Canvas>
                        <Button>{"Example Button"}</Button>
                    </Canvas>
                </Section>
                <Section class="transparent">
                    <h2>{"✨ Props"}</h2>
                    <table>
                        <thead>
                            <tr>
                                <th>{"Name"}</th>
                                <th>{"Description"}</th>
                                <th>{"Default"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{"type"}</td>
                                <td>{"button | submit | reset"}</td>
                                <td>{"button"}</td>
                            </tr>
                            <tr>
                                <td>{"size"}</td>
                                <td>{"sm | md | lg"}</td>
                                <td>{"md"}</td>
                            </tr>
                            <tr>
                                <td>{"color"}</td>
                                <td>{"primary | black | dark | light | white"}</td>
                                <td>{"primary"}</td>
                            </tr>
                            <tr>
                                <td>{"block"}</td>
                                <td>{"true | false"}</td>
                                <td>{"false"}</td>
                            </tr>
                        </tbody>
                    </table>
                </Section>
                <Section class="transparent">
                    <h2>{"📚 Docs"}</h2>
                    <div ref=self.node_ref.clone() />
                </Section>
            </>
        }
    }
}
