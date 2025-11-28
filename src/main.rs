//! DCRM - Dirmacs CRM
//! A minimal, efficient customer relationship manager
//! Built with Rust + Dioxus for maximum performance

#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod models;
mod pages;
mod state;

use components::{ModalContainer, Sidebar, TopBar};
use models::load_data;
use pages::{ActivitiesPage, ContactsPage, DashboardPage, DealsPage};
use state::{Modal, View};

fn main() {
    // Launch the desktop application
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Load persisted data or sample data
    let app_data = use_signal(|| load_data());

    // App state
    let current_view = use_signal(|| View::Dashboard);
    let modal = use_signal(|| Modal::None);
    let search_query = use_signal(|| String::new());

    // Provide context
    use_context_provider(|| app_data);
    use_context_provider(|| current_view);
    use_context_provider(|| modal);
    use_context_provider(|| search_query);

    rsx! {
        // Include the stylesheet
        style { {include_str!("../assets/main.css")} }

        div { class: "app-container",
            // Sidebar Navigation
            Sidebar {}

            // Main Content Area
            div { class: "main-content",
                // Top Bar with Search
                TopBar {}

                // Page Content
                match *current_view.read() {
                    View::Dashboard => rsx! { DashboardPage {} },
                    View::Contacts => rsx! { ContactsPage {} },
                    View::Deals => rsx! { DealsPage {} },
                    View::Activities => rsx! { ActivitiesPage {} },
                }
            }

            // Modal Container
            ModalContainer {}
        }
    }
}
