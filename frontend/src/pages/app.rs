use yew::prelude::*;
use yew_router::prelude::*;

use crate::routing::AppRoute;

use crate::pages::layout::switch;

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <BrowserRouter>
            <Switch<AppRoute> render={switch}/>
        </BrowserRouter>
    )
}
