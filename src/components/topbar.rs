use dioxus::prelude::*;
use crate::state::{View, Modal, use_current_view, use_modal, use_search_query};

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
        header { 
            class: "h-14 min-h-14 flex items-center justify-between px-6 bg-dark-800 border-b border-zinc-800",
            
            // Page title
            h1 { class: "text-lg font-semibold text-zinc-100", "{title}" }

            // Search bar
            div { 
                class: "flex items-center gap-2 bg-dark-700 border border-zinc-700 rounded-lg px-4 py-2 w-80
                        focus-within:border-accent focus-within:ring-1 focus-within:ring-accent/20 transition-all",
                span { class: "text-zinc-500", "⌕" }
                input {
                    class: "flex-1 bg-transparent border-none outline-none text-zinc-100 text-sm placeholder-zinc-500",
                    r#type: "text",
                    placeholder: "Search contacts, deals, activities...",
                    value: "{search_query}",
                    onfocus: move |_| modal.set(Modal::Search),
                    oninput: move |e| search_query.set(e.value()),
                }
                span { class: "font-mono text-[10px] text-zinc-500 bg-dark-600 px-1.5 py-0.5 rounded", "⌘K" }
            }

            // Actions
            div { class: "flex gap-2",
                if let Some(label) = new_button_label {
                    button {
                        class: "inline-flex items-center gap-2 px-4 py-2 bg-accent text-dark-900 text-sm font-medium 
                                rounded-md hover:bg-accent-dim transition-colors",
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
