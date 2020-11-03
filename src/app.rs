use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{AboutPage, LandingPage};
use crate::route::AppRoute;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute | {
                        match switch {
                            AppRoute::Landing => html!{ <LandingPage/> },
                            AppRoute::About => html!{ <AboutPage/> },
                            _ => html!{"NA"},
                        }
                    } )
                />
            </>
        }
    }
}
