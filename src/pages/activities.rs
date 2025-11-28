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
        div { class: "content-area",
            // Header
            div { class: "flex items-center justify-between mb-6",
                div { class: "flex gap-4",
                    h2 { class: "text-lg font-semibold", "Activities" }
                    if pending_count > 0 {
                        span {
                            class: "badge",
                            style: "background: rgba(245, 158, 11, 0.15); color: var(--warning);",
                            "{pending_count} pending"
                        }
                    }
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| modal.set(Modal::NewActivity),
                    "+ New Activity"
                }
            }

            // Filters
            div { class: "flex gap-2 mb-6",
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
                div { style: "width: 1px; background: var(--border); margin: 0 0.5rem;" }
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
            div { class: "card",
                if activities.is_empty() {
                    div { class: "empty-state",
                        div { class: "empty-state-icon", "◇" }
                        div { class: "empty-state-title", "No activities found" }
                        div { class: "empty-state-text",
                            "No activities match your current filter"
                        }
                    }
                } else {
                    div { class: "table-container",
                        table { class: "table",
                            thead {
                                tr {
                                    th { style: "width: 40px;" }
                                    th { "Activity" }
                                    th { "Type" }
                                    th { "Related To" }
                                    th { "Date" }
                                    th { style: "width: 80px;" }
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
        "btn btn-secondary btn-sm"
    } else {
        "btn btn-ghost btn-sm"
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

    let row_opacity = if activity.completed {
        "opacity: 0.5;"
    } else {
        ""
    };
    let title_style = if activity.completed {
        "text-decoration: line-through; color: var(--text-muted);"
    } else {
        ""
    };

    let checkbox_style = if activity.completed {
        "width: 20px; height: 20px; border: 2px solid var(--border); border-radius: 4px; display: flex; align-items: center; justify-content: center; transition: all 0.15s ease; background: var(--accent); border-color: var(--accent);"
    } else {
        "width: 20px; height: 20px; border: 2px solid var(--border); border-radius: 4px; display: flex; align-items: center; justify-content: center; transition: all 0.15s ease;"
    };

    rsx! {
        tr {
            class: "table-row-clickable",
            style: "{row_opacity}",

            // Checkbox
            td {
                div {
                    class: "cursor-pointer",
                    style: "{checkbox_style}",
                    onclick: {
                        let id = activity_id.clone();
                        move |_| toggle_activity_completed(&mut data, &id)
                    },
                    if activity.completed {
                        span { style: "color: var(--bg-primary); font-size: 12px;", "✓" }
                    }
                }
            }

            // Title & Description
            td {
                div {
                    class: "font-medium",
                    style: "{title_style}",
                    "{activity.title}"
                }
                if let Some(desc) = &activity.description {
                    div {
                        class: "text-sm text-muted truncate",
                        style: "max-width: 300px;",
                        "{desc}"
                    }
                }
            }

            // Type
            td {
                span { class: "flex items-center gap-2",
                    span { "{activity.activity_type.icon()}" }
                    span { class: "text-sm", "{activity.activity_type.display_name()}" }
                }
            }

            // Related
            td {
                span { class: "text-sm text-secondary", "{related}" }
            }

            // Date
            td {
                span { class: "text-sm text-muted font-mono", "{activity.format_date()}" }
            }

            // Actions
            td {
                button {
                    class: "btn btn-ghost btn-icon btn-sm",
                    style: "color: var(--danger);",
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
