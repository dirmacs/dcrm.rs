use dioxus::prelude::*;
use crate::models::Contact;
use crate::state::{use_app_data, use_modal, Modal, delete_contact};

#[component]
pub fn ContactsPage() -> Element {
    let data = use_app_data();
    let mut modal = use_modal();
    let mut selected_id = use_signal(|| None::<String>);

    let contacts = data.read().contacts.clone();
    let selected_contact = selected_id.read().as_ref().and_then(|id| {
        data.read().contact_by_id(id).cloned()
    });

    rsx! {
        div { class: "flex h-full overflow-hidden",
            // Contact List
            div { class: "w-[360px] min-w-[360px] border-r border-zinc-800 flex flex-col overflow-hidden",
                div { class: "p-4 border-b border-zinc-800",
                    div { class: "flex items-center justify-between",
                        span { class: "text-sm text-zinc-500", "{contacts.len()} contacts" }
                        button {
                            class: "px-3 py-1.5 bg-accent text-dark-900 text-sm font-medium rounded-md hover:bg-accent-dim transition-colors",
                            onclick: move |_| modal.set(Modal::NewContact),
                            "+ New"
                        }
                    }
                }
                div { class: "flex-1 overflow-y-auto",
                    if contacts.is_empty() {
                        div { class: "flex flex-col items-center justify-center py-16 px-8 text-center",
                            div { class: "w-16 h-16 bg-dark-700 rounded-full flex items-center justify-center text-2xl text-zinc-500 mb-4",
                                "◎"
                            }
                            div { class: "text-zinc-100 font-medium mb-1", "No contacts yet" }
                            div { class: "text-sm text-zinc-500 mb-5", "Add your first contact to get started" }
                            button {
                                class: "px-4 py-2 bg-accent text-dark-900 text-sm font-medium rounded-md hover:bg-accent-dim transition-colors",
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
            div { class: "flex-1 flex flex-col overflow-hidden",
                if let Some(contact) = selected_contact {
                    ContactDetail { 
                        contact: contact,
                        on_close: move |_| selected_id.set(None),
                    }
                } else {
                    div { class: "flex flex-col items-center justify-center h-full text-center",
                        div { class: "w-16 h-16 bg-dark-700 rounded-full flex items-center justify-center text-2xl text-zinc-500 mb-4",
                            "◎"
                        }
                        div { class: "text-zinc-100 font-medium mb-1", "Select a contact" }
                        div { class: "text-sm text-zinc-500", "Choose a contact from the list to view details" }
                    }
                }
            }
        }
    }
}

#[component]
fn ContactListItem(
    contact: Contact,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let bg = if selected { "bg-dark-700" } else { "hover:bg-dark-700/50" };
    let border = if selected { "border-l-2 border-l-accent" } else { "border-l-2 border-l-transparent" };

    rsx! {
        div {
            class: "flex items-center gap-4 p-4 cursor-pointer transition-colors border-b border-zinc-800 {bg} {border}",
            onclick: move |e| onclick.call(e),
            
            // Avatar
            div { class: "w-10 h-10 rounded-full bg-accent/10 flex items-center justify-center font-semibold text-accent text-sm",
                "{contact.initials()}"
            }
            
            div { class: "flex-1 min-w-0",
                div { class: "font-medium text-zinc-100 truncate", "{contact.full_name()}" }
                div { class: "text-sm text-zinc-500 truncate",
                    if let Some(company) = &contact.company {
                        "{company}"
                    } else {
                        "{contact.email}"
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

    let activities = data.read().activities_for_contact(&contact.id)
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    let deals = data.read().deals.iter()
        .filter(|d| d.contact_id.as_ref() == Some(&contact.id))
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div { class: "bg-dark-800 border-l border-zinc-800 h-full flex flex-col overflow-hidden",
            // Header
            div { class: "p-5 border-b border-zinc-800",
                div { class: "flex items-center gap-4",
                    div { class: "w-16 h-16 rounded-full bg-accent/10 flex items-center justify-center font-semibold text-accent text-xl",
                        "{contact.initials()}"
                    }
                    div {
                        h2 { class: "text-lg font-semibold text-zinc-100", "{contact.full_name()}" }
                        if let Some(position) = &contact.position {
                            p { class: "text-sm text-zinc-400", "{position}" }
                        }
                    }
                }
                div { class: "flex gap-2 mt-4",
                    button {
                        class: "px-3 py-1.5 text-sm bg-dark-700 border border-zinc-700 text-zinc-100 rounded-md hover:bg-zinc-700 transition-colors",
                        onclick: {
                            let id = contact_id.clone();
                            move |_| modal.set(Modal::EditContact(id.clone()))
                        },
                        "Edit"
                    }
                    button {
                        class: "w-8 h-8 flex items-center justify-center rounded-md text-zinc-400 hover:bg-zinc-700 hover:text-zinc-100 transition-colors",
                        onclick: move |e| on_close.call(e),
                        "✕"
                    }
                }
            }

            // Body
            div { class: "flex-1 overflow-y-auto p-5",
                // Contact Info Section
                DetailSection { title: "Contact Information",
                    DetailRow { label: "Email", value: contact.email.clone() }
                    
                    if let Some(phone) = &contact.phone {
                        DetailRow { label: "Phone", value: phone.clone() }
                    }
                    
                    if let Some(company) = &contact.company {
                        DetailRow { label: "Company", value: company.clone() }
                    }
                }

                // Tags
                if !contact.tags.is_empty() {
                    DetailSection { title: "Tags",
                        div { class: "flex gap-2 flex-wrap",
                            for tag in &contact.tags {
                                span { class: "text-xs bg-dark-700 border border-zinc-700 px-2 py-1 rounded text-zinc-400",
                                    "{tag}"
                                }
                            }
                        }
                    }
                }

                // Associated Deals
                if !deals.is_empty() {
                    DetailSection { title: format!("Deals ({})", deals.len()).leak(),
                        for deal in &deals {
                            div { 
                                class: "bg-dark-700 border border-zinc-700 rounded-lg p-3 mb-2",
                                div { class: "font-medium text-sm text-zinc-100 mb-1", "{deal.title}" }
                                div { class: "flex items-center justify-between",
                                    span { class: "font-mono text-sm text-accent", "{deal.format_value()}" }
                                    span { 
                                        class: "text-xs px-2 py-0.5 rounded {deal.stage.badge_class()}",
                                        "{deal.stage}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Recent Activities
                DetailSection { title: format!("Activity ({})", activities.len()).leak(),
                    if activities.is_empty() {
                        p { class: "text-sm text-zinc-500", "No activities recorded" }
                    } else {
                        div { class: "space-y-1",
                            for activity in activities.iter().take(5) {
                                div { class: "flex items-center gap-3 py-2",
                                    div { class: "w-7 h-7 rounded-full bg-dark-700 flex items-center justify-center text-xs",
                                        "{activity.activity_type.icon()}"
                                    }
                                    div { class: "flex-1",
                                        div { class: "text-sm text-zinc-100", "{activity.title}" }
                                        div { class: "text-xs text-zinc-500", "{activity.format_date()}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Notes
                if let Some(notes) = &contact.notes {
                    DetailSection { title: "Notes",
                        p { class: "text-sm text-zinc-300", "{notes}" }
                    }
                }

                // Danger Zone
                div { class: "mt-8",
                    button {
                        class: "text-sm text-red-400 hover:text-red-300 transition-colors",
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

#[component]
fn DetailSection(title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "mb-6",
            div { class: "text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-3",
                "{title}"
            }
            {children}
        }
    }
}

#[component]
fn DetailRow(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "flex justify-between py-2",
            span { class: "text-sm text-zinc-500", "{label}" }
            span { class: "text-sm text-zinc-100", "{value}" }
        }
    }
}
