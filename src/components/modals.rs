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

    }
}
