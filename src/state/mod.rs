use crate::models::{Activity, ActivityType, AppData, Contact, Deal, DealStage, save_data};
use dioxus::prelude::*;

// ============================================================================
// Global App State
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Dashboard,
    Contacts,
    Deals,
    Activities,
}

impl Default for View {
    fn default() -> Self {
        View::Dashboard
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Modal {
    None,
    NewContact,
    EditContact(String),
    NewDeal,
    EditDeal(String),
    NewActivity,
    ContactDetail(String),
    DealDetail(String),
    Search,
}

impl Default for Modal {
    fn default() -> Self {
        Modal::None
    }
}
