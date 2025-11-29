use crate::models::{Activity, ActivityType};
use crate::state::{Modal, delete_activity, toggle_activity_completed, use_app_data, use_modal};
use dioxus::prelude::*;

#[component]
pub fn ActivitiesPage() -> Element {
    let data = use_app_data();
    let mut modal = use_modal();
    let mut filter = use_signal(|| ActivityFilter::All);

    let activities: Vec<Activity> = {
        let data_ref = data.read();
        let mut acts: Vec<_> = data_ref.activities.clone();

        // Apply filter
        match *filter.read() {
            ActivityFilter::All => {}
            ActivityFilter::Tasks => acts.retain(|a| a.activity_type == ActivityType::Task),
            ActivityFilter::Calls => acts.retain(|a| a.activity_type == ActivityType::Call),
            ActivityFilter::Emails => acts.retain(|a| a.activity_type == ActivityType::Email),
            ActivityFilter::Meetings => acts.retain(|a| a.activity_type == ActivityType::Meeting),
            ActivityFilter::Notes => acts.retain(|a| a.activity_type == ActivityType::Note),
            ActivityFilter::Pending => acts.retain(|a| !a.completed),
            ActivityFilter::Completed => acts.retain(|a| a.completed),
        }

        // Sort by created date, newest first
        acts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        acts
    };

    let pending_count = data.read().pending_tasks_count();

    rsx! {
        div { class: "flex-1 overflow-hidden p-6 flex flex-col",
            // Header
            div { class: "flex items-center justify-between mb-6",
                div { class: "flex items-center gap-4",
                    h2 { class: "text-lg font-semibold text-zinc-100", "Activities" }
                    if pending_count > 0 {
                        span { class: "text-xs bg-amber-500/15 text-amber-400 px-2 py-0.5 rounded-full",
                            "{pending_count} pending"
                        }
                    }
                }
                button {
                    class: "px-4 py-2 bg-accent text-dark-900 text-sm font-medium rounded-md hover:bg-accent-dim transition-colors",
                    onclick: move |_| modal.set(Modal::NewActivity),
                    "+ New Activity"
                }
            }

            // Filters
            div { class: "flex gap-2 mb-6 flex-wrap",
                FilterTab {
                    label: "All",
                    active: *filter.read() == ActivityFilter::All,
                    onclick: move |_| filter.set(ActivityFilter::All),
                }
                FilterTab {
                    label: "Pending",
                    active: *filter.read() == ActivityFilter::Pending,
                    onclick: move |_| filter.set(ActivityFilter::Pending),
                }
                FilterTab {
                    label: "Completed",
                    active: *filter.read() == ActivityFilter::Completed,
                    onclick: move |_| filter.set(ActivityFilter::Completed),
                }

                div { class: "w-px bg-zinc-700 mx-2" }

                FilterTab {
                    label: "Tasks",
                    active: *filter.read() == ActivityFilter::Tasks,
                    onclick: move |_| filter.set(ActivityFilter::Tasks),
                }
                FilterTab {
                    label: "Calls",
                    active: *filter.read() == ActivityFilter::Calls,
                    onclick: move |_| filter.set(ActivityFilter::Calls),
                }
                FilterTab {
                    label: "Emails",
                    active: *filter.read() == ActivityFilter::Emails,
                    onclick: move |_| filter.set(ActivityFilter::Emails),
                }
                FilterTab {
                    label: "Meetings",
                    active: *filter.read() == ActivityFilter::Meetings,
                    onclick: move |_| filter.set(ActivityFilter::Meetings),
                }
                FilterTab {
                    label: "Notes",
                    active: *filter.read() == ActivityFilter::Notes,
                    onclick: move |_| filter.set(ActivityFilter::Notes),
                }
            }

            // Activity List
            div { class: "bg-dark-800 border border-zinc-800 rounded-xl overflow-hidden flex-1",
                if activities.is_empty() {
                    div { class: "flex flex-col items-center justify-center py-16 text-center",
                        div { class: "w-16 h-16 bg-dark-700 rounded-full flex items-center justify-center text-2xl text-zinc-500 mb-4",
                            "◇"
                        }
                        div { class: "text-zinc-100 font-medium mb-1", "No activities found" }
                        div { class: "text-sm text-zinc-500", "No activities match your current filter" }
                    }
                } else {
                    div { class: "overflow-x-auto",
                        table { class: "w-full",
                            thead {
                                tr { class: "border-b border-zinc-800 bg-dark-700",
                                    th { class: "w-10 px-4 py-3" }
                                    th { class: "text-left px-4 py-3 text-xs font-semibold text-zinc-500 uppercase tracking-wider", "Activity" }
                                    th { class: "text-left px-4 py-3 text-xs font-semibold text-zinc-500 uppercase tracking-wider", "Type" }
                                    th { class: "text-left px-4 py-3 text-xs font-semibold text-zinc-500 uppercase tracking-wider", "Related To" }
                                    th { class: "text-left px-4 py-3 text-xs font-semibold text-zinc-500 uppercase tracking-wider", "Date" }
                                    th { class: "w-20 px-4 py-3" }
                                }
                            }
                            tbody {
                                for activity in activities {
                                    ActivityRow { activity: activity }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ActivityFilter {
    All,
    Tasks,
    Calls,
    Emails,
    Meetings,
    Notes,
    Pending,
    Completed,
}

#[component]
fn FilterTab(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let class = if active {
        "px-3 py-1.5 text-sm rounded-md bg-dark-700 border border-zinc-700 text-zinc-100"
    } else {
        "px-3 py-1.5 text-sm rounded-md text-zinc-400 hover:bg-dark-700 transition-colors"
    };

    rsx! {
        button {
            class: "{class}",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

#[component]
fn ActivityRow(activity: Activity) -> Element {
    let mut data = use_app_data();
    let activity_id = activity.id.clone();

    let contact_name = activity
        .contact_id
        .as_ref()
        .and_then(|id| data.read().contact_by_id(id).map(|c| c.full_name()));

    let deal_name = activity
        .deal_id
        .as_ref()
        .and_then(|id| data.read().deal_by_id(id).map(|d| d.title.clone()));

    let related = match (&contact_name, &deal_name) {
        (Some(c), Some(d)) => format!("{} • {}", c, d),
        (Some(c), None) => c.clone(),
        (None, Some(d)) => d.clone(),
        (None, None) => "—".to_string(),
    };

    let row_opacity = if activity.completed { "opacity-50" } else { "" };
    let title_decoration = if activity.completed {
        "line-through text-zinc-500"
    } else {
        "text-zinc-100"
    };
    let checkbox_class = if activity.completed {
        "w-5 h-5 border-2 rounded flex items-center justify-center cursor-pointer transition-colors bg-accent border-accent"
    } else {
        "w-5 h-5 border-2 rounded flex items-center justify-center cursor-pointer transition-colors border-zinc-600 hover:border-accent"
    };

    rsx! {
        tr {
            class: "border-b border-zinc-800 hover:bg-dark-700/50 transition-colors {row_opacity}",

            // Checkbox
            td { class: "px-4 py-3",
                div {
                    class: "{checkbox_class}",
                    onclick: {
                        let id = activity_id.clone();
                        move |_| toggle_activity_completed(&mut data, &id)
                    },
                    if activity.completed {
                        span { class: "text-dark-900 text-xs", "✓" }
                    }
                }
            }

            // Title & Description
            td { class: "px-4 py-3",
                div { class: "font-medium text-sm {title_decoration}", "{activity.title}" }
                if let Some(desc) = &activity.description {
                    div { class: "text-sm text-zinc-500 truncate max-w-xs", "{desc}" }
                }
            }

            // Type
            td { class: "px-4 py-3",
                span { class: "flex items-center gap-2 text-sm text-zinc-400",
                    span { "{activity.activity_type.icon()}" }
                    span { "{activity.activity_type.display_name()}" }
                }
            }

            // Related
            td { class: "px-4 py-3",
                span { class: "text-sm text-zinc-400", "{related}" }
            }

            // Date
            td { class: "px-4 py-3",
                span { class: "text-sm text-zinc-500 font-mono", "{activity.format_date()}" }
            }

            // Actions
            td { class: "px-4 py-3",
                button {
                    class: "w-8 h-8 flex items-center justify-center rounded text-zinc-500 hover:bg-red-500/10 hover:text-red-400 transition-colors",
                    onclick: {
                        let id = activity_id.clone();
                        move |_| delete_activity(&mut data, &id)
                    },
                    "×"
                }
            }
        }
    }
}
