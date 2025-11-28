use crate::models::Contact;
use crate::state::{Modal, delete_contact, use_app_data, use_modal};
use dioxus::prelude::*;

#[component]
pub fn ContactsPage() -> Element {
    let data = use_app_data();
    let mut modal = use_modal();
    let mut selected_id = use_signal(|| None::<String>);

    let contacts = data.read().contacts.clone();
    let selected_contact = selected_id
        .read()
        .as_ref()
        .and_then(|id| data.read().contact_by_id(id).cloned());

    rsx! {
        div { class: "split-view",
            // Contact List
            div { class: "split-list",
                div { class: "split-list-header",
                    div { class: "flex items-center justify-between",
                        span { class: "text-sm text-muted", "{contacts.len()} contacts" }
                        button {
                            class: "btn btn-primary btn-sm",
                            onclick: move |_| modal.set(Modal::NewContact),
                            "+ New"
                        }
                    }
                }
                div { class: "split-list-body",
                    if contacts.is_empty() {
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "◎" }
                            div { class: "empty-state-title", "No contacts yet" }
                            div { class: "empty-state-text", "Add your first contact to get started" }
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| modal.set(Modal::NewContact),
                                "+ Add Contact"
                            }
                        }
                    } else {
                        for contact in contacts {
                            ContactListItem {
                                contact: contact.clone(),
                                selected: selected_id.read().as_ref() == Some(&contact.id),
                                onclick: {
                                    let id = contact.id.clone();
                                    move |_| selected_id.set(Some(id.clone()))
                                },
                            }
                        }
                    }
                }
            }

            // Contact Detail
            div { class: "split-detail",
                if let Some(contact) = selected_contact {
                    ContactDetail {
                        contact: contact,
                        on_close: move |_| selected_id.set(None),
                    }
                } else {
                    div { class: "empty-state h-full",
                        div { class: "empty-state-icon", "◎" }
                        div { class: "empty-state-title", "Select a contact" }
                        div { class: "empty-state-text", "Choose a contact from the list to view details" }
                    }
                }
            }
        }
    }
}

#[component]
fn ContactListItem(contact: Contact, selected: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let bg = if selected {
        "background: var(--bg-hover);"
    } else {
        ""
    };
    let border = if selected {
        "border-left: 3px solid var(--accent);"
    } else {
        "border-left: 3px solid transparent;"
    };

    rsx! {
        div {
            class: "contact-item",
            style: "{bg} {border}",
            onclick: move |e| onclick.call(e),

            div { class: "avatar", "{contact.initials()}" }
            div { class: "contact-info",
                div { class: "contact-name", "{contact.full_name()}" }
                div { class: "contact-details",
                    if let Some(company) = &contact.company {
                        span { "{company}" }
                    } else {
                        span { "{contact.email}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ContactDetail(contact: Contact, on_close: EventHandler<MouseEvent>) -> Element {
    let mut data = use_app_data();
    let mut modal = use_modal();
    let contact_id = contact.id.clone();

    let activities = data
        .read()
        .activities_for_contact(&contact.id)
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    let deals = data
        .read()
        .deals
        .iter()
        .filter(|d| d.contact_id.as_ref() == Some(&contact.id))
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div { class: "detail-panel",
            // Header
            div { class: "detail-header",
                div { class: "flex items-center gap-4",
                    div { class: "avatar avatar-lg", "{contact.initials()}" }
                    div {
                        h2 { class: "text-lg font-semibold", "{contact.full_name()}" }
                        if let Some(position) = &contact.position {
                            p { class: "text-sm text-secondary", "{position}" }
                        }
                    }
                }
                div { class: "flex gap-2 mt-4",
                    button {
                        class: "btn btn-secondary btn-sm",
                        onclick: {
                            let id = contact_id.clone();
                            move |_| modal.set(Modal::EditContact(id.clone()))
                        },
                        "Edit"
                    }
                    button {
                        class: "btn btn-ghost btn-sm",
                        onclick: move |e| on_close.call(e),
                        "✕"
                    }
                }
            }

            // Body
            div { class: "detail-body",
                // Contact Info Section
                div { class: "detail-section",
                    div { class: "detail-section-title", "Contact Information" }

                    div { class: "detail-row",
                        span { class: "detail-label", "Email" }
                        span { class: "detail-value", "{contact.email}" }
                    }

                    if let Some(phone) = &contact.phone {
                        div { class: "detail-row",
                            span { class: "detail-label", "Phone" }
                            span { class: "detail-value", "{phone}" }
                        }
                    }

                    if let Some(company) = &contact.company {
                        div { class: "detail-row",
                            span { class: "detail-label", "Company" }
                            span { class: "detail-value", "{company}" }
                        }
                    }
                }

                // Tags
                if !contact.tags.is_empty() {
                    div { class: "detail-section",
                        div { class: "detail-section-title", "Tags" }
                        div { class: "flex gap-2 flex-wrap",
                            for tag in &contact.tags {
                                span { class: "tag", "{tag}" }
                            }
                        }
                    }
                }

                // Associated Deals
                if !deals.is_empty() {
                    div { class: "detail-section",
                        div { class: "detail-section-title", "Deals ({deals.len()})" }
                        for deal in &deals {
                            div {
                                class: "deal-card",
                                style: "margin-bottom: 0.5rem;",
                                div { class: "deal-card-title", "{deal.title}" }
                                div { class: "deal-card-footer",
                                    span { class: "deal-card-value", "{deal.format_value()}" }
                                    span { class: "badge {deal.stage.badge_class()}", "{deal.stage}" }
                                }
                            }
                        }
                    }
                }

                // Recent Activities
                div { class: "detail-section",
                    div { class: "detail-section-title", "Activity ({activities.len()})" }
                    if activities.is_empty() {
                        p { class: "text-sm text-muted", "No activities recorded" }
                    } else {
                        div { class: "activity-list",
                            for activity in activities.iter().take(5) {
                                div { class: "activity-item",
                                    div { class: "activity-icon", "{activity.activity_type.icon()}" }
                                    div { class: "activity-content",
                                        div { class: "activity-title", "{activity.title}" }
                                        div { class: "activity-meta", "{activity.format_date()}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Notes
                if let Some(notes) = &contact.notes {
                    div { class: "detail-section",
                        div { class: "detail-section-title", "Notes" }
                        p { class: "text-sm", "{notes}" }
                    }
                }

                // Danger Zone
                div { class: "detail-section mt-6",
                    button {
                        class: "btn btn-ghost btn-sm",
                        style: "color: var(--danger);",
                        onclick: {
                            let id = contact_id.clone();
                            move |_| {
                                delete_contact(&mut data, &id);
                            }
                        },
                        "Delete Contact"
                    }
                }
            }
        }
    }
}
