// The homepage is statically rendered, so we don't need to a persistent websocket connection.

use dioxus::prelude::*;

use crate::{
    api::{Sort, fetch_products},
    components::{nav, product_item::product_item},
};

pub(crate) fn Home() -> Element {
    let products = use_server_future(|| fetch_products(10, Sort::Ascending))?;
    let products = products().unwrap()?;

    rsx! {
        nav::nav {}
        section { class: "p-10",
            for product in products {
                product_item {
                    product
                }
            }
        }
    }
}
