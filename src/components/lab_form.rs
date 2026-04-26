//! Lab result form component.

use crate::app::AppState;
use crate::models::{Item, ItemType};
use crate::utils::today_iso;
use leptos::prelude::*;

/// Form for recording a lab result event.
#[component]
pub(crate) fn LabForm() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");

  let lab_name = RwSignal::new(String::new());
  let lab_date = RwSignal::new(today_iso());
  let error = RwSignal::new(String::new());

  let submit = move |ev: leptos::ev::SubmitEvent| {
    ev.prevent_default();
    let name = lab_name.get();
    if name.trim().is_empty() {
      error.set("กรุณากรอกชื่อรายการตรวจ".to_string());
      return;
    }
    let id =
      (js_sys::Date::now() as u64).wrapping_add((js_sys::Math::random() * 1_000_000.0) as u64);
    let item = Item {
      id,
      name: name.clone(),
      raw_name: name,
      dose: String::new(),
      start: {
        let s = lab_date.get();
        if s.is_empty() { None } else { Some(s) }
      },
      end: None,
      ongoing: false,
      start_unknown: false,
      item_type: ItemType::Lab,
      first_dose: String::new(),
      last_dose: String::new(),
    };
    state.items.update(|v| v.push(item));
    lab_name.set(String::new());
    lab_date.set(today_iso());
    error.set(String::new());
  };

  view! {
      <section class="form-card">
          <h2 class="form-card-title lab-title">"เพิ่มผลแล็บ"</h2>
          <form on:submit=submit>
              <div class="field-group">
                  <label class="field-label">"รายการตรวจ *"</label>
                  <input
                      type="text"
                      class="field-input"
                      placeholder="เช่น CBC, LFT"
                      prop:value=move || lab_name.get()
                      on:input=move |ev| { lab_name.set(event_target_value(&ev)); error.set(String::new()); }
                  />
              </div>
              <div class="field-group">
                  <label class="field-label">"วันที่ตรวจ"</label>
                  <input
                      type="date"
                      class="field-input"
                      prop:value=move || lab_date.get()
                      on:input=move |ev| lab_date.set(event_target_value(&ev))
                  />
              </div>
              <Show when=move || !error.get().is_empty()>
                  <p class="form-error">{move || error.get()}</p>
              </Show>
              <button type="submit" class="btn btn-lab">"+ เพิ่มผลแล็บ"</button>
          </form>
      </section>
  }
}
