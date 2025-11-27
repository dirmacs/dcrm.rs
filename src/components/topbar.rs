use crate::state::{Modal, View, use_current_view, use_modal, use_search_query};
use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    let current_view = use_current_view();
    let mut modal = use_modal();
    let mut search_query = use_search_query();

    let title = match *current_view.read() {
        View::Dashboard => "Dashboard",
        View::Contacts => "Contacts",
        View::Deals => "Deals",
        View::Activities => "Activities",
    };

    let new_button_label = match *current_view.read() {
        View::Dashboard => None,
        View::Contacts => Some("New Contact"),
        View::Deals => Some("New Deal"),
        View::Activities => Some("New Activity"),
    };

    rsx! {
        header { class: "top-bar",
            // Page title
            h1 { class: "page-title", "{title}" }

            // Search bar
            div { class: "search-bar",
                span { style: "color: var(--text-muted);", "⌕" }
                input {
                    class: "search-input",
                    r#type: "text",
                    placeholder: "Search contacts, deals, activities...",
                    value: "{search_query}",
                    onfocus: move |_| modal.set(Modal::Search),
                    oninput: move |e| search_query.set(e.value()),
                }
                span { class: "search-shortcut", "⌘K" }
            }

            // Actions
            div { class: "quick-actions",
                if let Some(label) = new_button_label {
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            match *current_view.read() {
                                View::Contacts => modal.set(Modal::NewContact),
                                View::Deals => modal.set(Modal::NewDeal),
                                View::Activities => modal.set(Modal::NewActivity),
                                _ => {}
                            }
                        },
                        span { "+" }
                        span { "{label}" }
                    }
                }
            }
        }
    }
}
