//! Root application component, AppState context, and layout.

use crate::components::drug_form::DrugForm;
use crate::components::item_table::ItemTable;
use crate::components::lab_form::LabForm;
use crate::components::modals::bsa::BsaModal;
use crate::components::modals::dress::DressModal;
use crate::components::modals::naranjo::NaranjoModal;
use crate::components::modals::update::UpdateModal;
use crate::components::patient_form::PatientForm;
use crate::components::reaction_form::ReactionForm;
use crate::components::timeline::TimelineCanvas;
use crate::components::toolbar::Toolbar;
use crate::components::tools_section::ToolsSection;
use crate::models::{AppData, FormData, Item};
use crate::utils::today_iso;
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;

/// Shared application state stored in context.
#[derive(Clone, Copy)]
pub(crate) struct AppState {
  pub(crate) items: RwSignal<Vec<Item>>,
  pub(crate) form: RwSignal<FormData>,
  pub(crate) show_naranjo: RwSignal<bool>,
  pub(crate) show_dress: RwSignal<bool>,
  pub(crate) show_bsa: RwSignal<bool>,
  pub(crate) show_update: RwSignal<bool>,
}

impl AppState {
  fn new() -> Self {
    let saved_hospital: Option<String> = LocalStorage::get("savedHospitalName").ok();
    let saved_prepared: Option<String> = LocalStorage::get("savedPreparedBy").ok();

    let initial_form = FormData {
      hospital_name: saved_hospital.unwrap_or_default(),
      report_date: today_iso(),
      prepared_by: saved_prepared.unwrap_or_default(),
      ..Default::default()
    };

    Self {
      items: RwSignal::new(Vec::new()),
      form: RwSignal::new(initial_form),
      show_naranjo: RwSignal::new(false),
      show_dress: RwSignal::new(false),
      show_bsa: RwSignal::new(false),
      show_update: RwSignal::new(false),
    }
  }

  /// Export current state as AppData.
  pub(crate) fn to_app_data(self) -> AppData {
    AppData {
      header: self.form.get(),
      items: self.items.get(),
    }
  }

  /// Load from AppData, replacing current state.
  pub(crate) fn load_app_data(&self, data: AppData) {
    self.form.set(data.header);
    self.items.set(data.items);
  }
}

/// Root Leptos component.
#[component]
pub(crate) fn App() -> impl IntoView {
  let state = AppState::new();
  provide_context(state);

  view! {
      <div class="app-shell">
          <header class="app-header">
              <h1 class="app-title">"Drug Timeline Generator"</h1>
              <p class="app-subtitle">"ระบบสร้างไทม์ไลน์การใช้ยาและอาการไม่พึงประสงค์"</p>
          </header>

          <main class="app-main">
              <aside class="sidebar">
                  <PatientForm/>
                  <DrugForm/>
                  <ReactionForm/>
                  <LabForm/>
              </aside>

              <section class="content-area">
                  <Toolbar/>
                  <ItemTable/>
                  <TimelineCanvas/>
                  <ToolsSection/>
              </section>
          </main>

          <footer class="app-footer">
              <p>"© 2025 ADR Timeline · ระบบสร้างไทม์ไลน์การใช้ยา"</p>
          </footer>

          <NaranjoModal/>
          <DressModal/>
          <BsaModal/>
          <UpdateModal/>
      </div>
  }
}
