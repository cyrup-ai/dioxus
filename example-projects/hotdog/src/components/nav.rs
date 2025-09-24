use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            span {}

            Link { to: Route::DogView,
                h1 { "🌭 HotDog! " }
            }

            Link { to: Route::Favorites, id: "heart", "♥️" }
        }

        Outlet::<Route> {}
    }
}
