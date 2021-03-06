use yew::prelude::*;

pub struct About;

impl Component for About {

    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender{
        true
    }
    fn view(& self) -> Html {
        html! {
            <div>{"You are on About"}</div>
        }
    }
}
