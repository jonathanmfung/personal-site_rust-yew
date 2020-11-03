use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/sidebar"]
    Sidebar,
    #[to = "/about"]
    About,
    // #[to = "/blog"]
    // BlogMain,
    // #[to = "/blog/{title}"]
    // BlogPost(String),
    // #[to = "/projects"]
    // ProjectsMain,
    // #[to = "/projects/{title}"]
    // PorjectPost(String),
    #[to = "/"]
    Landing,
}
