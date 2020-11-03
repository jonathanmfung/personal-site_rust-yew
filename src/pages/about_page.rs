use yew::prelude::*;
use crate::components::{Sidebar, About};

pub struct AboutPage;

impl Component for AboutPage {

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
            <>
            <Sidebar/>
            <About/>
            </>
        }
    }
}
