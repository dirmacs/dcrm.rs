use crate::models::{Activity, ActivityType, Contact, Deal, DealStage};
use crate::state::{
    Modal, SearchResult, add_activity, add_contact, add_deal, search, update_contact, update_deal,
    use_app_data, use_modal, use_search_query,
};
use chrono::Utc;
use dioxus::prelude::*;

#[component]
pub fn ModalContainer() -> Element {
    let modal = use_modal();

    // match &*modal.read() {
    //     Modal::None => rsx! {},
    //     Modal::NewContact => rsx! { Contact { contact: None } },
    //     Modal::EditContact(id) => {
    //         let data = use_app_data();
    //         let contact = data.read().contact_by_id(id).cloned();
    //         rsx! { ContactModal { contact }}
    //     }
    //     Modal::NewDeal => rsx! { DealModal { deal: None } },
    //     Modal::EditDeal(id) => {
    //         let data = use_app_data();
    //         let deal = data.read().deal_by_id(id).cloned();
    //         rsx! { DealModal { deal } }
    //     }
    //     Modal::NewActivity => rsx! { ActivityModal {} },
    //     Modal::Search => rsx! { SearchModal {} },
    //     Modal::ContactDetail(_) | Modal::DealDetail(_) => rsx! {},
    // }
}

// ============================================================================
// Contact Modal
// ============================================================================

#[component]
fn ContactModal(contact: Option<Contact>) -> Element {
    let mut modal = use_modal();
    let mut data = use_app_data();

    let is_edit = contact.is_some();
    let title = if is_edit {
        "Edit Contact"
    } else {
        "New Contact"
    };

    let initial = contact.unwrap_or_default();

    let mut first_name = use_signal(|| initial.first_name.clone());
    let mut last_name = use_signal(|| initial.last_name.clone());
    let mut email = use_signal(|| initial.email.clone());
    let mut phone = use_signal(|| initial.phone.clone().unwrap_or_default());
    let mut company = use_signal(|| initial.company.clone().unwrap_or_default());
    let mut position = use_signal(|| initial.position.clone().unwrap_or_default());
    let mut tags_str = use_signal(|| initial.tags.join(", "));
    let mut notes = use_signal(|| initial.notes.clone().unwrap_or_default());
    let contact_id = initial.id.clone();

    let handle_save = move |_| {
        let mut contact = Contact::new(
            first_name.read().clone(),
            last_name.read().clone(),
            email.read().clone(),
        );

        if is_edit {
            contact.id == contact_id.clone();
            contact.created_at = initial.created_at;
        }

        contact.phone = if phone.read().is_empty() {
            None
        } else {
            Some(phone.read().clone())
        };
        contact.company = if company.read().is_empty() {
            None
        } else {
            Some(company.read().clone())
        };
        contact.position = if position.read().is_empty() {
            None
        } else {
            Some(position.read().clone())
        };
        contact.notes = if notes.read().is_empty() {
            None
        } else {
            Some(notes.read().clone())
        };
        contact.tags = tags_str
            .read()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        contact.updated_at = Utc::now();

        if is_edit {
            update_contact(&mut data, contact);
        } else {
            add_contact(&mut data, contact);
        }

        modal.set(Modal::None);
    };

    rsx! {
        div {
            class: "modal-backdrop",
            onclick: move |_| modal.set(Modal::None),

            div {
                class: "modal",
                onclick: |e| e.stop_propagation(),

                div { class: "modal-header",
                    h3 { class: "modal-title", "{title}" }
                    button {
                        class: "btn btn-ghost btn-icon",
                        onclick: move |_| modal.set(Modal::None),
                        "✕"
                    }
                }

                div { class: "modal-body",
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "form-group",
                            label { class: "form-label", "First Name *" }
                            input {
                                class: "form-input",
                                r#type: "text",
                                value: "{first_name}",
                                oninput: move |e| first_name.set(e.value()),
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "Last Name *" }
                            input {
                                class: "form-input",
                                r#type: "text",
                                value: "{last_name}",
                                oninput: move |e| last_name.set(e.value()),
                            }
                        }
                    }


                    div { class: "form-group",
                        label { class: "form-label", "Email *" }
                        input {
                            class: "form-input",
                            r#type: "email",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Phone"}
                        input {
                            class: "form-input",
                            r#type: "tel",
                            value: "{phone}",
                            oninput: move |e| phone.set(e.value()),
                        }
                    }

                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "form-group",
                            label { class: "form-label", "Company" }
                            input {
                                class: "form-input",
                                r#type: "text",
                                value: "{company}",
                                oninput: move |e| company.set(e.value()),
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "Position" }
                            input {
                                class: "form-input",
                                r#type: "text",
                                value: "{position}",
                                oninput: move |e| position.set(e.value()),
                            }
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Tags (comma separated)" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "enterprise, hot-lead, referral",
                            value: "{tags_str}",
                            oninput: move |e| tags_str.set(e.value()),
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Notes" }
                        textarea {
                            class: "form-input",
                            value: "{notes}",
                            oninput: move |e| notes.set(e.value()),
                        }
                    }
                }


                div { class: "modal-footer",
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| modal.set(Modal::None),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: handle_save,
                        if is_edit { "Save Changes" } else { "Create Contact" }
                    }
                }
            }
        }
    }
}

// ============================================================================
// Deal Modal
// ============================================================================

#[component]
fn DealModal(deal: Option<Deal>) -> Element {
    let mut modal = use_modal();
    let mut data = use_app_data();

    let is_edit = deal.is_some();
    let title = if is_edit { "Edit Deal" } else { "New Deal" };

    let initial = deal.unwrap_or_default();

    let mut deal_title = use_signal(|| initial.title.clone());
    let mut company = use_signal(|| initial.company.clone());
    let mut value = use_signal(|| initial.value.to_string());
    let mut stage = use_signal(|| initial.stage);
    let mut probability = use_signal(|| initial.probability.to_string());
    let mut contact_id = use_signal(|| initial.contact_id.clone());
    let mut notes = use_signal(|| initial.notes.clone().unwrap_or_default());
    let deal_id = initial.id.clone();

    let contacts = data.read().contacts.clone();

    let handle_save = move |_| {
        let mut deal = Deal::new(
            deal_title.read().clone(),
            company.read().clone(),
            value.read().parse().unwrap_or(0.0),
        );

        if is_edit {
            deal.id = deal_id.clone();
            deal.created_at = initial.created_at;
        }

        deal.stage = *stage.read();
        deal.probability = probability.read().parse().unwrap_or(10);
        deal.contact_id = contact_id.read().clone();
        deal.notes = if notes.read().is_empty() {
            None
        } else {
            Some(notes.read().clone())
        };
        deal.updated_at = Utc::now();

        if is_edit {
            update_deal(&mut data, deal);
        } else {
            add_deal(&mut data, deal);
        }

        modal.set(Modal::None);
    };

    rsx! {
        div {
            class: "modal-backdrop",
            onclick: move |_| modal.set(Modal::None),

            div {
                class: "modal",
                onclick: |e| e.stop_propagation(),

                div { class: "modal-header",
                    h3 { class: "modal-title", "{title}" }
                    button {
                        class: "btn btn-ghost btn-icon",
                        onclick: move |_| modal.set(Modal::None),
                        "✕"
                    }
                }

                div { class: "modal-body",
                    div { class: "form-group",
                        label { class: "form-label", "Deal Title *" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "e.g., Enterprise License Agreement",
                            value: "{deal_title}",
                            oninput: move |e| deal_title.set(e.value()),
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Company *" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            value: "{company}",
                            oninput: move |e| company.set(e.value()),
                        }
                    }

                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "form-group",
                            label { class: "form-label", "Value ($)" }
                            input {
                                class: "form-input",
                                r#type: "number",
                                value: "{value}",
                                oninput: move |e| value.set(e.value()),
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "Probability (%)" }
                            input {
                                class: "form-input",
                                r#type: "number",
                                min: "0",
                                max: "100",
                                value: "{probability}",
                                oninput: move |e| probability.set(e.value()),
                            }
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Stage" }
                        select {
                            class: "form-input form-select",
                            value: "{stage.read().display_name()}",
                            onchange: move |e| {
                                let new_stage = match e.value().as_str() {
                                    "Lead" => DealStage::Lead,
                                    "Qualified" => DealStage::Qualified,
                                    "Proposal" => DealStage::Proposal,
                                    "Negotiation" => DealStage::Negotiation,
                                    "Won" => DealStage::Won,
                                    "Lost" => DealStage::Lost,
                                    _ => DealStage::Lead,
                                };
                                stage.set(new_stage);
                            },
                            for s in DealStage::all() {
                                option {
                                    value: "{s.display_name()}",
                                    selected: *stage.read() == s,
                                    "{s.display_name()}"
                                }
                            }
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Contact" }
                        select {
                            class: "form-input form-select",
                            onchange: move |e| {
                                let val = e.value();
                                contact_id.set(if val.is_empty() { None } else { Some(val) });
                            },
                            option { value: "", "Select a contact..." }
                            for c in &contacts {
                                option {
                                    value: "{c.id}",
                                    selected: contact_id.read().as_ref() == Some(&c.id),
                                    "{c.full_name()}"
                                }
                            }
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Notes" }
                        textarea {
                            class: "form-input",
                            value: "{notes}",
                            oninput: move |e| notes.set(e.value()),
                        }
                    }
                }

                div { class: "modal-footer",
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| modal.set(Modal::None),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: handle_save,
                        if is_edit { "Save Changes" } else { "Create Deal" }
                    }
                }
            }
        }
    }
}
