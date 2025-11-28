use crate::models::{ActivityType, DealStage};
use crate::state::{Modal, use_app_data, use_modal};
use dioxus::prelude::*;

#[component]
pub fn DashboardPage() -> Element {
    let data = use_app_data();
    let mut modal = use_modal();

    rsx! {
        div { class: "content-area",
            // Stats Grid
            div { class: "stats-grid",
                StatCard {
                    label: "Total Contacts",
                    value: data.read().contacts.len().to_string(),
                    icon: "◎",
                }
                StatCard {
                    label: "Active Deals",
                    value: data.read().active_deals_count().to_string(),
                    icon: "◈",
                }
                StatCard {
                    label: "Pipeline Value",
                    value: format_currency(data.read().total_pipeline_value()),
                    icon: "◆",
                    accent: true,
                }
                StatCard {
                    label: "Won Revenue",
                    value: format_currency(data.read().won_deals_value()),
                    icon: "✓",
                    positive: true,
                }
            }

            // Two-column layout
            div { class: "grid grid-cols-2 gap-6",
                // Pipeline Overview
                div { class: "card",
                    div { class: "card-header",
                        h3 { class: "card-title", "Pipeline Overview" }
                    }
                    div { class: "card-body",
                        for stage in DealStage::active() {
                            PipelineStageRow {
                                stage: stage,
                                count: data.read().deals_by_stage(stage).len(),
                                value: data.read().deals_by_stage(stage).iter().map(|d| d.value).sum::<f64>(),
                            }
                        }
                    }
                }

                // Recent Activity
                div { class: "card",
                    div { class: "card-header",
                        h3 { class: "card-title", "Recent Activity" }
                        button {
                            class: "btn btn-ghost btn-sm",
                            "View All"
                        }
                    }
                    div { class: "card-body",
                        {
                            let recent_activities: Vec<_> = data.read().recent_activities(5)
                                .into_iter()
                                .cloned()
                                .collect();

                            if recent_activities.is_empty() {
                                rsx! {
                                    div { class: "empty-state",
                                        div { class: "empty-state-title", "No activities yet" }
                                        div { class: "empty-state-text", "Start logging your interactions" }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "activity-list",
                                        for activity in recent_activities {
                                            {
                                                let icon = activity.activity_type.icon().to_string();
                                                let title = activity.title.clone();
                                                let meta = activity.format_date();
                                                let completed = activity.completed;

                                                rsx! {
                                                    ActivityRow {
                                                        icon: icon,
                                                        title: title,
                                                        meta: meta,
                                                        completed: completed,
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Pending Tasks
            div { class: "card mt-6",
                div { class: "card-header",
                    h3 { class: "card-title",
                        "Pending Tasks"
                        if data.read().pending_tasks_count() > 0 {
                            span {
                                style: "margin-left: 0.5rem; font-size: 0.75rem; color: var(--warning);
                                        background: rgba(245, 158, 11, 0.15); padding: 2px 8px; border-radius: 10px;",
                                "{data.read().pending_tasks_count()}"
                            }
                        }
                    }
                    button {
                        class: "btn btn-secondary btn-sm",
                        onclick: move |_| modal.set(Modal::NewActivity),
                        "+ Add Task"
                    }
                }
                div { class: "card-body",
                    {
                        let tasks: Vec<_> = data.read().activities.iter()
                            .filter(|a| a.activity_type == ActivityType::Task && !a.completed)
                            .cloned()
                            .collect();

                        if tasks.is_empty() {
                            rsx! {
                                div { class: "empty-state",
                                    div { class: "empty-state-icon", "✓" }
                                    div { class: "empty-state-title", "All caught up!" }
                                    div { class: "empty-state-text", "No pending tasks" }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "activity-list",
                                    for task in tasks.iter().take(5) {
                                        TaskRow {
                                            id: task.id.clone(),
                                            title: task.title.clone(),
                                            due: task.due_date.map(|d| d.format("%b %d").to_string()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(
    label: &'static str,
    value: String,
    icon: &'static str,
    #[props(default = false)] accent: bool,
    #[props(default = false)] positive: bool,
) -> Element {
    let value_style = if accent {
        "color: var(--accent);"
    } else if positive {
        "color: var(--success);"
    } else {
        ""
    };

    rsx! {
        div { class: "stat-card",
            div { class: "flex items-center justify-between mb-2",
                span { class: "stat-label", "{label}" }
                span {
                    style: "font-size: 1.25rem; opacity: 0.5;",
                    "{icon}"
                }
            }
            div {
                class: "stat-value",
                style: "{value_style}",
                "{value}"
            }
        }
    }
}

#[component]
fn PipelineStageRow(stage: DealStage, count: usize, value: f64) -> Element {
    let total_pipeline = 600000.0; // Rough estimate for percentage calculation
    let percentage = ((value / total_pipeline) * 100.0).min(100.0);

    rsx! {
        div {
            style: "padding: 0.75rem 0; border-bottom: 1px solid var(--border);",
            div { class: "flex items-center justify-between mb-2",
                div { class: "flex items-center gap-2",
                    span {
                        class: "pipeline-dot",
                        style: "background: {stage.color()};",
                    }
                    span { class: "text-sm font-medium", "{stage.display_name()}" }
                    span { class: "text-xs text-muted", "({count})" }
                }
                span { class: "font-mono text-sm text-accent", "{format_currency(value)}" }
            }
            // Progress bar
            div {
                style: "height: 4px; background: var(--bg-tertiary); border-radius: 2px; overflow: hidden;",
                div {
                    style: "height: 100%; background: {stage.color()}; width: {percentage}%;
                            border-radius: 2px; transition: width 0.3s ease;",
                }
            }
        }
    }
}

#[component]
fn ActivityRow(icon: String, title: String, meta: String, completed: bool) -> Element {
    let opacity = if completed { "0.5" } else { "1" };

    rsx! {
        div {
            class: "activity-item",
            style: "opacity: {opacity};",
            div { class: "activity-icon", "{icon}" }
            div { class: "activity-content",
                div { class: "activity-title", "{title}" }
                div { class: "activity-meta", "{meta}" }
            }
        }
    }
}

#[component]
fn TaskRow(id: String, title: String, due: Option<String>) -> Element {
    let mut data = use_app_data();

    rsx! {
        div {
            class: "activity-item cursor-pointer",
            onclick: move |_| {
                crate::state::toggle_activity_completed(&mut data, &id);
            },
            div {
                class: "activity-icon",
                style: "border: 2px solid var(--border); background: transparent;",
            }
            div { class: "activity-content",
                div { class: "activity-title", "{title}" }
                if let Some(d) = due {
                    div { class: "activity-meta", "Due: {d}" }
                }
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
