#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/79236386")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/79236386")]

//! Parse the root tokens in the rsx! { } macro
//! =========================================
//!
//! This parsing path emerges directly from the macro call, with `RsxRender` being the primary entrance into parsing.
//! This feature must support:
//! - [x] Optionally rendering if the `in XYZ` pattern is present
//! - [x] Fragments as top-level element (through ambiguous)
//! - [x] Components as top-level element (through ambiguous)
//! - [x] Tags as top-level elements (through ambiguous)
//! - [x] Good errors if parsing fails
//!
//! Any errors in using rsx! will likely occur when people start using it, so the first errors must be really helpful.
//!
//! # Completions
//! Rust analyzer completes macros by looking at the expansion of the macro and trying to match the start of identifiers in the macro to identifiers in the current scope
//!
//! Eg, if a macro expands to this:
//! ```rust, ignore
//! struct MyStruct;
//!
//! // macro expansion
//! My
//! ```
//! Then the analyzer will try to match the start of the identifier "My" to an identifier in the current scope (MyStruct in this case).
//!
//! In dioxus, our macros expand to the completions module if we know the identifier is incomplete:
//! ```rust, ignore
//! // In the root of the macro, identifiers must be elements
//! // rsx! { di }
//! dioxus_elements::elements::di
//!
//! // Before the first child element, every following identifier is either an attribute or an element
//! // rsx! { div { ta } }
//! // Isolate completions scope
//! mod completions__ {
//!     // import both the attributes and elements this could complete to
//!     use dioxus_elements::elements::div::*;
//!     use dioxus_elements::elements::*;
//!     fn complete() {
//!         ta;
//!     }
//! }
//!
//! // After the first child element, every following identifier is another element
//! // rsx! { div { attribute: value, child {} di } }
//! dioxus_elements::elements::di
//! ```

mod assign_dyn_ids;
mod attribute;
mod component;
mod element;
mod forloop;
mod ifchain;
mod node;
mod raw_expr;
mod rsx_block;
mod rsx_call;
mod template_body;
mod text_node;

mod diagnostics;
mod expr_node;
mod ifmt;
mod literal;
mod location;
mod partial_closure;
mod util;

// Re-export the namespaces into each other
pub use diagnostics::Diagnostics;
pub use ifmt::*;
pub use innerlude::*;
pub use node::*;
pub use partial_closure::PartialClosure;
use quote::{ToTokens, TokenStreamExt, quote};
pub use rsx_call::*;
use syn::{
    Result, Token,
    parse::{Parse, ParseStream},
};
pub use template_body::TemplateBody;
pub(crate) mod innerlude {
    pub use crate::{
        attribute::*, component::*, diagnostics::*, element::*, expr_node::*, forloop::*,
        ifchain::*, ifmt::*, literal::*, location::*, node::*, raw_expr::*, rsx_block::*,
        template_body::*, text_node::*, util::*,
    };
}
