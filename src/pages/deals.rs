use crate::models::{Deal, DealStage};
use crate::state::{Modal, delete_deal, update_deal_stage, use_app_data, use_modal};
use dioxus::prelude::*;

#[component]
pub fn DealsPage() -> Element {
    let data = use_app_data();
    let mut modal = use_modal();

    rsx! {
        div { class: "content-area",
            // Header Stats
            div { class: "flex items-center justify-between mb-6",
                div { class: "flex gap-6",
                    PipelineStat {
                        label: "Total Pipeline",
                        value: format_currency(data.read().total_pipeline_value()),
                    }
                    PipelineStat {
                        label: "Weighted Value",
                        value: format_currency(data.read().weighted_pipeline_value()),
                    }
                    PipelineStat {
                        label: "Active Deals",
                        value: data.read().active_deals_count().to_string(),
                    }
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| modal.set(Modal::NewDeal),
                    "+ New Deal"
                }
            }

            // Pipeline Board
            div { class: "pipeline-board",
                for stage in DealStage::active() {
                    PipelineColumn {
                        stage: stage,
                        deals: data.read().deals_by_stage(stage).into_iter().cloned().collect(),
                    }
                }

                // Won & Lost columns (collapsed)
                ClosedDealsColumn {
                    won_deals: data.read().deals_by_stage(DealStage::Won).into_iter().cloned().collect(),
                    lost_deals: data.read().deals_by_stage(DealStage::Lost).into_iter().cloned().collect(),
                }
            }
        }
    }
}

#[component]
fn PipelineStat(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            div { class: "text-xs text-muted mb-1", "{label}" }
            div { class: "font-mono font-semibold text-lg", "{value}" }
        }
    }
}

#[component]
fn PipelineColumn(stage: DealStage, deals: Vec<Deal>) -> Element {
    let total_value: f64 = deals.iter().map(|d| d.value).sum();

    rsx! {
        div { class: "pipeline-column",
            // Column Header
            div { class: "pipeline-column-header",
                div { class: "pipeline-column-title",
                    span {
                        class: "pipeline-dot",
                        style: "background: {stage.color()};",
                    }
                    span { class: "font-medium text-sm", "{stage.display_name()}" }
                }
                div { class: "flex items-center gap-2",
                    span { class: "pipeline-column-count", "{deals.len()}" }
                    span { class: "text-xs text-muted font-mono", "{format_currency(total_value)}" }
                }
            }

            // Column Body
            div { class: "pipeline-column-body",
                for deal in deals {
                    DealCard { deal: deal }
                }
            }
        }
    }
}

#[component]
fn DealCard(deal: Deal) -> Element {
    let data = use_app_data();
    let mut modal = use_modal();
    let deal_id = deal.id.clone();
    let mut show_actions = use_signal(|| false);

    let contact_name = deal
        .contact_id
        .as_ref()
        .and_then(|id| data.read().contact_by_id(id).map(|c| c.full_name()));

    rsx! {
        div {
            class: "deal-card",
            onmouseenter: move |_| show_actions.set(true),
            onmouseleave: move |_| show_actions.set(false),
            onclick: {
                let id = deal_id.clone();
                move |_| modal.set(Modal::DealDetail(id.clone()))
            },

            div { class: "flex items-start justify-between",
                div { class: "deal-card-title", "{deal.title}" }
                if *show_actions.read() {
                    DealQuickActions {
                        deal_id: deal_id.clone(),
                        current_stage: deal.stage,
                    }
                }
            }

            div { class: "deal-card-company",
                "{deal.company}"
                if let Some(name) = contact_name {
                    span { class: "text-muted", " • {name}" }
                }
            }

            div { class: "deal-card-footer",
                span { class: "deal-card-value", "{deal.format_value()}" }
                span { class: "deal-card-date", "{deal.probability}% prob." }
            }
        }
    }
}

#[component]
fn DealQuickActions(deal_id: String, current_stage: DealStage) -> Element {
    let mut data = use_app_data();
    let mut show_menu = use_signal(|| false);

    let next_stages: Vec<DealStage> = DealStage::all()
        .into_iter()
        .filter(|s| *s != current_stage)
        .collect();

    rsx! {
        div {
            class: "dropdown relative",
            onclick: move |e| {
                e.stop_propagation();
                show_menu.set(!show_menu());
            },

            button {
                class: "btn btn-ghost btn-icon btn-sm",
                "⋮"
            }

            if *show_menu.read() {
                div { class: "dropdown-menu",
                    div { class: "text-xs text-muted px-3 py-1", "Move to:" }
                    for stage in next_stages {
                        div {
                            class: "dropdown-item",
                            onclick: {
                                let id = deal_id.clone();
                                move |e| {
                                    e.stop_propagation();
                                    update_deal_stage(&mut data, &id, stage);
                                    show_menu.set(false);
                                }
                            },
                            span {
                                class: "pipeline-dot",
                                style: "background: {stage.color()}; width: 6px; height: 6px;",
                            }
                            "{stage.display_name()}"
                        }
                    }
                    div { class: "border-t border-solid", style: "border-color: var(--border); margin: 0.25rem 0;" }
                    div {
                        class: "dropdown-item danger",
                        onclick: {
                            let id = deal_id.clone();
                            move |e| {
                                e.stop_propagation();
                                delete_deal(&mut data, &id);
                            }
                        },
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
fn ClosedDealsColumn(won_deals: Vec<Deal>, lost_deals: Vec<Deal>) -> Element {
    let won_value: f64 = won_deals.iter().map(|d| d.value).sum();
    let lost_value: f64 = lost_deals.iter().map(|d| d.value).sum();

    rsx! {
        div { class: "pipeline-column",
            style: "background: var(--bg-tertiary);",

            // Won Section
            div { class: "pipeline-column-header",
                style: "border-bottom: none;",
                div { class: "pipeline-column-title",
                    span {
                        class: "pipeline-dot",
                        style: "background: var(--success);",
                    }
                    span { class: "font-medium text-sm text-success", "Won" }
                }
                div { class: "flex items-center gap-2",
                    span { class: "pipeline-column-count", "{won_deals.len()}" }
                    span { class: "text-xs text-success font-mono", "{format_currency(won_value)}" }
                }
            }

            // Lost Section
            div { class: "pipeline-column-header",
                div { class: "pipeline-column-title",
                    span {
                        class: "pipeline-dot",
                        style: "background: var(--danger);",
                    }
                    span { class: "font-medium text-sm text-danger", "Lost" }
                }
                div { class: "flex items-center gap-2",
                    span { class: "pipeline-column-count", "{lost_deals.len()}" }
                    span { class: "text-xs text-danger font-mono", "{format_currency(lost_value)}" }
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
