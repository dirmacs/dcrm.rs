use dioxus::prelude::*;
use crate::state::{View, use_current_view, use_app_data};

#[component]
pub fn Sidebar() -> Element {
    let mut current_view = use_current_view();
    let data = use_app_data();

    let pending_tasks = data.read().pending_tasks_count();
    let active_deals = data.read().active_deals_count();

    rsx! {
        aside { 
            class: "w-60 min-w-60 bg-dark-800 border-r border-zinc-800 flex flex-col h-full",
            
            // Logo
            div { class: "px-4 py-5 border-b border-zinc-800",
                div { class: "font-mono text-xl font-bold tracking-tight",
                    span { class: "text-accent", "D" }
                    span { "CRM" }
                }
            }

            // Navigation
            nav { class: "flex-1 p-4 overflow-y-auto",
                // Main Section
                div { class: "mb-6",
                    div { class: "text-[10px] font-semibold text-zinc-500 uppercase tracking-wider mb-2 px-3",
                        "Main"
                    }
                    
                    NavItem {
                        label: "Dashboard",
                        icon: "◉",
                        active: *current_view.read() == View::Dashboard,
                        onclick: move |_| current_view.set(View::Dashboard),
                        badge: None,
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
                div { class: "mb-6",
                    div { class: "text-[10px] font-semibold text-zinc-500 uppercase tracking-wider mb-2 px-3",
                        "Pipeline"
                    }
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
            div { class: "p-4 border-t border-zinc-800 mt-auto",
                div { class: "text-[10px] text-zinc-600 text-center",
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
    badge: Option<String>,
) -> Element {
    let base_classes = "relative flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium cursor-pointer transition-colors mb-0.5";
    let state_classes = if active {
        "bg-accent/10 text-accent"
    } else {
        "text-zinc-400 hover:bg-zinc-800 hover:text-zinc-100"
    };
    
    rsx! {
        div { 
            class: "{base_classes} {state_classes}",
            onclick: move |e| onclick.call(e),
            
            if active {
                div { class: "absolute left-0 w-0.5 h-6 bg-accent rounded-r" }
            }
            
            span { class: "text-lg opacity-70", "{icon}" }
            span { "{label}" }
            
            if let Some(badge_text) = badge {
                span { 
                    class: "ml-auto text-[10px] bg-dark-700 px-1.5 py-0.5 rounded-full text-zinc-500",
                    "{badge_text}"
                }
            }
        }
    }
}

#[component]
fn QuickStat(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "px-3 py-2",
            div { class: "text-[10px] text-zinc-500 mb-0.5", "{label}" }
            div { class: "text-sm font-semibold text-zinc-100 font-mono", "{value}" }
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
