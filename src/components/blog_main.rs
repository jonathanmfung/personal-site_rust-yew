use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::RouterAnchor;

pub struct BlogMain;

impl Component for BlogMain {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;

        html! {
            <div>
                {"Sidebar"}
                <Anchor route=AppRoute::Landing>{"Click for Landing"}</Anchor>
                <Anchor route=AppRoute::About>{"Click for About"}</Anchor>
            </div>
        }
    }
}
