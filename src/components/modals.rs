use dioxus::prelude::*;
use crate::models::{Contact, Deal, DealStage, Activity, ActivityType};
use crate::state::{
    Modal, use_modal, use_app_data, use_search_query,
    add_contact, update_contact, add_deal, update_deal, add_activity,
    search, SearchResult,
};
use chrono::Utc;

#[component]
pub fn ModalContainer() -> Element {
    let modal = use_modal();
    
    match &*modal.read() {
        Modal::None => rsx! {},
        Modal::NewContact => rsx! { ContactModal { contact: None } },
        Modal::EditContact(id) => {
            let data = use_app_data();
            let contact = data.read().contact_by_id(id).cloned();
            rsx! { ContactModal { contact: contact } }
        },
        Modal::NewDeal => rsx! { DealModal { deal: None } },
        Modal::EditDeal(id) => {
            let data = use_app_data();
            let deal = data.read().deal_by_id(id).cloned();
            rsx! { DealModal { deal: deal } }
        },
        Modal::NewActivity => rsx! { ActivityModal {} },
        Modal::Search => rsx! { SearchModal {} },
        Modal::ContactDetail(_) | Modal::DealDetail(_) => rsx! {},
    }
}

// ============================================================================
// Contact Modal
// ============================================================================

#[component]
fn ContactModal(contact: Option<Contact>) -> Element {
    let mut modal = use_modal();
    let mut data = use_app_data();
    
    let is_edit = contact.is_some();
    let title = if is_edit { "Edit Contact" } else { "New Contact" };
    
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
            contact.id = contact_id.clone();
            contact.created_at = initial.created_at;
        }
        
        contact.phone = if phone.read().is_empty() { None } else { Some(phone.read().clone()) };
        contact.company = if company.read().is_empty() { None } else { Some(company.read().clone()) };
        contact.position = if position.read().is_empty() { None } else { Some(position.read().clone()) };
        contact.notes = if notes.read().is_empty() { None } else { Some(notes.read().clone()) };
        contact.tags = tags_str.read()
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
        // Backdrop
        div { 
            class: "fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50",
            onclick: move |_| modal.set(Modal::None),
            
            // Modal
            div { 
                class: "bg-dark-800 border border-zinc-700 rounded-xl w-full max-w-lg max-h-[90vh] overflow-hidden shadow-2xl",
                onclick: |e| e.stop_propagation(),
                
                // Header
                div { class: "flex items-center justify-between px-5 py-4 border-b border-zinc-700",
                    h3 { class: "text-base font-semibold text-zinc-100", "{title}" }
                    button {
                        class: "w-8 h-8 flex items-center justify-center rounded-md text-zinc-400 hover:bg-zinc-700 hover:text-zinc-100 transition-colors",
                        onclick: move |_| modal.set(Modal::None),
                        "✕"
                    }
                }
                
                // Body
                div { class: "p-5 overflow-y-auto",
                    div { class: "grid grid-cols-2 gap-4",
                        FormField { label: "First Name *",
                            input {
                                class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                        focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                                r#type: "text",
                                value: "{first_name}",
                                oninput: move |e| first_name.set(e.value()),
                            }
                        }
                        FormField { label: "Last Name *",
                            input {
                                class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                        focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                                r#type: "text",
                                value: "{last_name}",
                                oninput: move |e| last_name.set(e.value()),
                            }
                        }
                    }
                    
                    FormField { label: "Email *",
                        input {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            r#type: "email",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                        }
                    }
                    
                    FormField { label: "Phone",
                        input {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            r#type: "tel",
                            value: "{phone}",
                            oninput: move |e| phone.set(e.value()),
                        }
                    }
                    
                    div { class: "grid grid-cols-2 gap-4",
                        FormField { label: "Company",
                            input {
                                class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                        focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                                r#type: "text",
                                value: "{company}",
                                oninput: move |e| company.set(e.value()),
                            }
                        }
                        FormField { label: "Position",
                            input {
                                class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                        focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                                r#type: "text",
                                value: "{position}",
                                oninput: move |e| position.set(e.value()),
                            }
                        }
                    }
                    
                    FormField { label: "Tags (comma separated)",
                        input {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    placeholder-zinc-500 focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            r#type: "text",
                            placeholder: "enterprise, hot-lead, referral",
                            value: "{tags_str}",
                            oninput: move |e| tags_str.set(e.value()),
                        }
                    }
                    
                    FormField { label: "Notes",
                        textarea {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    min-h-24 resize-y focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            value: "{notes}",
                            oninput: move |e| notes.set(e.value()),
                        }
                    }
                }
                
                // Footer
                div { class: "flex justify-end gap-3 px-5 py-4 border-t border-zinc-700",
                    button {
                        class: "px-4 py-2 bg-dark-700 border border-zinc-700 text-zinc-100 text-sm font-medium
                                rounded-md hover:bg-zinc-700 transition-colors",
                        onclick: move |_| modal.set(Modal::None),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-accent text-dark-900 text-sm font-medium rounded-md 
                                hover:bg-accent-dim transition-colors",
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
        deal.notes = if notes.read().is_empty() { None } else { Some(notes.read().clone()) };
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
            class: "fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50",
            onclick: move |_| modal.set(Modal::None),
            
            div { 
                class: "bg-dark-800 border border-zinc-700 rounded-xl w-full max-w-lg max-h-[90vh] overflow-hidden shadow-2xl",
                onclick: |e| e.stop_propagation(),
                
                div { class: "flex items-center justify-between px-5 py-4 border-b border-zinc-700",
                    h3 { class: "text-base font-semibold text-zinc-100", "{title}" }
                    button {
                        class: "w-8 h-8 flex items-center justify-center rounded-md text-zinc-400 hover:bg-zinc-700 hover:text-zinc-100 transition-colors",
                        onclick: move |_| modal.set(Modal::None),
                        "✕"
                    }
                }
                
                div { class: "p-5 overflow-y-auto",
                    FormField { label: "Deal Title *",
                        input {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    placeholder-zinc-500 focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            r#type: "text",
                            placeholder: "e.g., Enterprise License Agreement",
                            value: "{deal_title}",
                            oninput: move |e| deal_title.set(e.value()),
                        }
                    }
                    
                    FormField { label: "Company *",
                        input {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            r#type: "text",
                            value: "{company}",
                            oninput: move |e| company.set(e.value()),
                        }
                    }
                    
                    div { class: "grid grid-cols-2 gap-4",
                        FormField { label: "Value ($)",
                            input {
                                class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                        focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                                r#type: "number",
                                value: "{value}",
                                oninput: move |e| value.set(e.value()),
                            }
                        }
                        FormField { label: "Probability (%)",
                            input {
                                class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                        focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                                r#type: "number",
                                min: "0",
                                max: "100",
                                value: "{probability}",
                                oninput: move |e| probability.set(e.value()),
                            }
                        }
                    }
                    
                    FormField { label: "Stage",
                        select {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
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
                    
                    FormField { label: "Contact",
                        select {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
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
                    
                    FormField { label: "Notes",
                        textarea {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    min-h-24 resize-y focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            value: "{notes}",
                            oninput: move |e| notes.set(e.value()),
                        }
                    }
                }
                
                div { class: "flex justify-end gap-3 px-5 py-4 border-t border-zinc-700",
                    button {
                        class: "px-4 py-2 bg-dark-700 border border-zinc-700 text-zinc-100 text-sm font-medium
                                rounded-md hover:bg-zinc-700 transition-colors",
                        onclick: move |_| modal.set(Modal::None),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-accent text-dark-900 text-sm font-medium rounded-md 
                                hover:bg-accent-dim transition-colors",
                        onclick: handle_save,
                        if is_edit { "Save Changes" } else { "Create Deal" }
                    }
                }
            }
        }
    }
}

// ============================================================================
// Activity Modal
// ============================================================================

#[component]
fn ActivityModal() -> Element {
    let mut modal = use_modal();
    let mut data = use_app_data();
    
    let mut activity_type = use_signal(|| ActivityType::Task);
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut contact_id = use_signal(|| None::<String>);
    let mut deal_id = use_signal(|| None::<String>);

    let contacts = data.read().contacts.clone();
    let deals = data.read().deals.clone();

    let handle_save = move |_| {
        let mut activity = Activity::new(
            *activity_type.read(),
            title.read().clone(),
        );
        
        activity.description = if description.read().is_empty() { None } else { Some(description.read().clone()) };
        activity.contact_id = contact_id.read().clone();
        activity.deal_id = deal_id.read().clone();
        
        add_activity(&mut data, activity);
        modal.set(Modal::None);
    };

    rsx! {
        div { 
            class: "fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50",
            onclick: move |_| modal.set(Modal::None),
            
            div { 
                class: "bg-dark-800 border border-zinc-700 rounded-xl w-full max-w-lg max-h-[90vh] overflow-hidden shadow-2xl",
                onclick: |e| e.stop_propagation(),
                
                div { class: "flex items-center justify-between px-5 py-4 border-b border-zinc-700",
                    h3 { class: "text-base font-semibold text-zinc-100", "New Activity" }
                    button {
                        class: "w-8 h-8 flex items-center justify-center rounded-md text-zinc-400 hover:bg-zinc-700 hover:text-zinc-100 transition-colors",
                        onclick: move |_| modal.set(Modal::None),
                        "✕"
                    }
                }
                
                div { class: "p-5 overflow-y-auto",
                    // Activity type selector
                    div { class: "mb-4",
                        label { class: "block text-xs font-medium text-zinc-400 mb-2", "Type" }
                        div { class: "flex gap-2 flex-wrap",
                            for at in [ActivityType::Task, ActivityType::Call, ActivityType::Email, ActivityType::Meeting, ActivityType::Note] {
                                button {
                                    class: if *activity_type.read() == at { 
                                        "px-3 py-1.5 text-sm rounded-md bg-dark-600 border border-zinc-600 text-zinc-100" 
                                    } else { 
                                        "px-3 py-1.5 text-sm rounded-md bg-transparent text-zinc-400 hover:bg-dark-700 transition-colors" 
                                    },
                                    onclick: move |_| activity_type.set(at),
                                    "{at.icon()} {at.display_name()}"
                                }
                            }
                        }
                    }
                    
                    FormField { label: "Title *",
                        input {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    placeholder-zinc-500 focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            r#type: "text",
                            placeholder: "What needs to be done?",
                            value: "{title}",
                            oninput: move |e| title.set(e.value()),
                        }
                    }
                    
                    FormField { label: "Description",
                        textarea {
                            class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                    placeholder-zinc-500 min-h-24 resize-y focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                            placeholder: "Add more details...",
                            value: "{description}",
                            oninput: move |e| description.set(e.value()),
                        }
                    }
                    
                    div { class: "grid grid-cols-2 gap-4",
                        FormField { label: "Contact",
                            select {
                                class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                        focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                                onchange: move |e| {
                                    let val = e.value();
                                    contact_id.set(if val.is_empty() { None } else { Some(val) });
                                },
                                option { value: "", "Select contact..." }
                                for c in &contacts {
                                    option { value: "{c.id}", "{c.full_name()}" }
                                }
                            }
                        }
                        FormField { label: "Deal",
                            select {
                                class: "w-full px-3 py-2 bg-dark-700 border border-zinc-700 rounded-md text-zinc-100 text-sm
                                        focus:border-accent focus:ring-1 focus:ring-accent/20 outline-none transition-all",
                                onchange: move |e| {
                                    let val = e.value();
                                    deal_id.set(if val.is_empty() { None } else { Some(val) });
                                },
                                option { value: "", "Select deal..." }
                                for d in &deals {
                                    option { value: "{d.id}", "{d.title}" }
                                }
                            }
                        }
                    }
                }
                
                div { class: "flex justify-end gap-3 px-5 py-4 border-t border-zinc-700",
                    button {
                        class: "px-4 py-2 bg-dark-700 border border-zinc-700 text-zinc-100 text-sm font-medium
                                rounded-md hover:bg-zinc-700 transition-colors",
                        onclick: move |_| modal.set(Modal::None),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-accent text-dark-900 text-sm font-medium rounded-md 
                                hover:bg-accent-dim transition-colors",
                        onclick: handle_save,
                        "Create Activity"
                    }
                }
            }
        }
    }
}

// ============================================================================
// Search Modal
// ============================================================================

#[component]
fn SearchModal() -> Element {
    let mut modal = use_modal();
    let mut search_query = use_search_query();
    let data = use_app_data();
    
    let results = search(&data.read(), &search_query.read());

    rsx! {
        div { 
            class: "fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50",
            onclick: move |_| {
                search_query.set(String::new());
                modal.set(Modal::None);
            },
            
            div { 
                class: "bg-dark-800 border border-zinc-700 rounded-xl w-full max-w-xl overflow-hidden shadow-2xl",
                onclick: |e| e.stop_propagation(),
                
                // Search input
                div { class: "p-4 border-b border-zinc-700",
                    div { 
                        class: "flex items-center gap-2 bg-dark-700 border border-zinc-700 rounded-lg px-4 py-2 w-full
                                focus-within:border-accent focus-within:ring-1 focus-within:ring-accent/20",
                        span { class: "text-zinc-500", "⌕" }
                        input {
                            class: "flex-1 bg-transparent border-none outline-none text-zinc-100 text-sm placeholder-zinc-500",
                            r#type: "text",
                            placeholder: "Search contacts, deals, activities...",
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value()),
                            autofocus: true,
                        }
                        span { class: "font-mono text-[10px] text-zinc-500 bg-dark-600 px-1.5 py-0.5 rounded", "ESC" }
                    }
                }
                
                // Results
                div { class: "max-h-96 overflow-y-auto",
                    if results.is_empty() && !search_query.read().is_empty() {
                        div { class: "flex flex-col items-center justify-center py-12 text-center",
                            div { class: "text-zinc-100 font-medium mb-1", "No results found" }
                            div { class: "text-sm text-zinc-500", "Try a different search term" }
                        }
                    } else if results.is_empty() {
                        div { class: "p-4 text-center text-zinc-500 text-sm",
                            "Start typing to search..."
                        }
                    } else {
                        for result in results {
                            SearchResultItem {
                                result: result,
                                on_select: move |_| {
                                    search_query.set(String::new());
                                    modal.set(Modal::None);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SearchResultItem(result: SearchResult, on_select: EventHandler<MouseEvent>) -> Element {
    let (icon, type_label) = match &result {
        SearchResult::Contact(_) => ("◎", "Contact"),
        SearchResult::Deal(_) => ("◈", "Deal"),
        SearchResult::Activity(_) => ("◇", "Activity"),
    };

    rsx! {
        div {
            class: "flex items-center gap-4 px-4 py-3 cursor-pointer hover:bg-dark-700 transition-colors border-b border-zinc-800 last:border-b-0",
            onclick: move |e| on_select.call(e),
            
            div { class: "w-8 h-8 rounded-full bg-dark-700 flex items-center justify-center text-zinc-400",
                "{icon}"
            }
            
            div { class: "flex-1 min-w-0",
                div { class: "font-medium text-zinc-100 truncate", "{result.title()}" }
                div { class: "text-sm text-zinc-500 truncate", "{result.subtitle()}" }
            }
            
            span { class: "text-xs bg-dark-700 border border-zinc-700 px-2 py-0.5 rounded text-zinc-500",
                "{type_label}"
            }
        }
    }
}

// ============================================================================
// Shared Components
// ============================================================================

#[component]
fn FormField(label: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "mb-4",
            label { class: "block text-xs font-medium text-zinc-400 mb-2", "{label}" }
            {children}
        }
    }
}
