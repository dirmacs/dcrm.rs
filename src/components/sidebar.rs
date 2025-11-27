use crate::state::{View, use_app_data, use_current_view};
use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    let mut current_view = use_current_view();
    let data = use_app_data();

    let pending_tasks = data.read().pending_tasks_count();
    let active_deals = data.read().active_deals_count();

    rsx! {
        aside { class: "sidebar",
            // Logo
            div { class: "sidebar-header",
                div { class: "logo",
                    span { class: "logo-accent", "D" }
                    span { "CRM" }
                }
            }

            // Navigation
            nav { class: "sidebar-nav",
                // Main Section
                div { class: "nav-section",
                    div { class: "nav-section-title", "Main" }

                    NavItem {
                        label: "Dashboard",
                        icon: "◉",
                        active: *current_view.read() == View::Dashboard,
                        onclick: move |_| current_view.set(View::Dashboard),
                    }

                    NavItem {
                        label: "Contacts",
                        icon: "◎",
                        active: *current_view.read() == View::Contacts,
                        onclick: move |_| current_view.set(View::Contacts),
                        badge: None,
                    }

                    NavItem {
                        label: "Deals",
                        icon: "◈",
                        active: *current_view.read() == View::Deals,
                        onclick: move |_| current_view.set(View::Deals),
                        badge: Some(active_deals.to_string()),
                    }

                    NavItem {
                        label: "Activities",
                        icon: "◇",
                        active: *current_view.read() == View::Activities,
                        onclick: move |_| current_view.set(View::Activities),
                        badge: if pending_tasks > 0 { Some(pending_tasks.to_string()) } else { None },
                    }
                }

                // Quick Stats
                div { class: "nav-section",
                    div { class: "nav-section-title", "Pipeline" }
                    QuickStat {
                        label: "Active Deals",
                        value: active_deals.to_string(),
                    }
                    QuickStat {
                        label: "Pipeline Value",
                        value: format_currency(data.read().total_pipeline_value()),
                    }
                    QuickStat {
                        label: "Won This Period",
                        value: format_currency(data.read().won_deals_value()),
                    }
                }
            }

            // Footer
            div {
                style: "padding: 1rem; border-top: 1px solid var(--border); margin-top: auto;",
                div {
                    style: "font-size: 0.7rem; color: var(--text-muted); text-align: center;",
                    "DCRM v0.1.0"
                }
            }
        }
    }
}

#[component]
fn NavItem(
    label: &'static str,
    icon: &'static str,
    active: bool,
    onclick: EventHandler<MouseEvent>,
    #[props(default)] badge: Option<String>,
) -> Element {
    let class = if active {
        "nav-item active"
    } else {
        "nav-item"
    };

    rsx! {
        div {
            class: "{class}",
            onclick: move |e| onclick.call(e),
            style: "position: relative;",

            span { class: "nav-icon", "{icon}" }
            span { "{label}" }

            if let Some(badge_text) = badge {
                span {
                    style: "margin-left: auto; font-size: 0.7rem; background: var(--bg-tertiary);
                            padding: 2px 6px; border-radius: 10px; color: var(--text-muted);",
                    "{badge_text}"
                }
            }
        }
    }
}

#[component]
fn QuickStat(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            style: "padding: 0.5rem 0.75rem; margin-bottom: 0.25rem;",
            div {
                style: "font-size: 0.7rem; color: var(--text-muted); margin-bottom: 2px;",
                "{label}"
            }
            div {
                style: "font-size: 0.875rem; font-weight: 600; color: var(--text-primary);
                        font-family: var(--font-mono);",
                "{value}"
            }
        }
    }
}

fn format_currency(value: f64) -> String {
    if value >= 1_000_000.0 {
        format!("${:.1}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("${:.0}K", value / 1_000.0)
    } else {
        format!("${:.0}", value)
    }
}
