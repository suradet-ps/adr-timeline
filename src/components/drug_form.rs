//! Drug entry form component.

use crate::app::AppState;
use crate::models::{Item, ItemType};
use crate::utils::today_iso;
use leptos::prelude::*;

/// Form for adding a new drug item to the timeline.
#[component]
pub(crate) fn DrugForm() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");

  let drug_name = RwSignal::new(String::new());
  let dose = RwSignal::new(String::new());
  let start_date = RwSignal::new(today_iso());
  let end_date = RwSignal::new(String::new());
  let ongoing = RwSignal::new(false);
  let start_unknown = RwSignal::new(false);
  let first_dose = RwSignal::new(String::new());
  let last_dose = RwSignal::new(String::new());
  let error = RwSignal::new(String::new());

  let submit = move |ev: leptos::ev::SubmitEvent| {
    ev.prevent_default();
    let name = drug_name.get();
    if name.trim().is_empty() {
      error.set("กรุณากรอกชื่อยา".to_string());
      return;
    }
    let d = dose.get();
    let full_name = if d.is_empty() {
      name.clone()
    } else {
      format!("{} ({})", name, d)
    };
    let id = (js_sys::Date::now() as u64).wrapping_add(fastrand_id());
    let item = Item {
      id,
      name: full_name,
      raw_name: name,
      dose: d,
      start: {
        let s = start_date.get();
        if s.is_empty() { None } else { Some(s) }
      },
      end: if ongoing.get() {
        None
      } else {
        let e = end_date.get();
        if e.is_empty() { None } else { Some(e) }
      },
      ongoing: ongoing.get(),
      start_unknown: start_unknown.get(),
      item_type: ItemType::Drug,
      first_dose: first_dose.get(),
      last_dose: last_dose.get(),
    };
    state.items.update(|v| v.push(item));
    // Reset
    drug_name.set(String::new());
    dose.set(String::new());
    start_date.set(today_iso());
    end_date.set(String::new());
    ongoing.set(false);
    start_unknown.set(false);
    first_dose.set(String::new());
    last_dose.set(String::new());
    error.set(String::new());
  };

  view! {
      <section class="form-card">
          <h2 class="form-card-title">"เพิ่มยา"</h2>
          <form on:submit=submit>
              <div class="field-group">
                  <label class="field-label">"ชื่อยา *"</label>
                  <input
                      type="text"
                      class="field-input"
                      placeholder="ชื่อยา"
                      prop:value=move || drug_name.get()
                      on:input=move |ev| { drug_name.set(event_target_value(&ev)); error.set(String::new()); }
                  />
              </div>
              <div class="field-group">
                  <label class="field-label">"ขนาด/วิธีใช้"</label>
                  <input
                      type="text"
                      class="field-input"
                      placeholder="เช่น 500 mg OD"
                      prop:value=move || dose.get()
                      on:input=move |ev| dose.set(event_target_value(&ev))
                  />
              </div>
              <div class="field-row">
                  <div class="field-group">
                      <label class="field-label">"วันเริ่มใช้"</label>
                      <input
                          type="date"
                          class="field-input"
                          prop:value=move || start_date.get()
                          on:input=move |ev| start_date.set(event_target_value(&ev))
                      />
                  </div>
                  <div class="field-group">
                      <label class="field-label">"วันหยุดยา"</label>
                      <input
                          type="date"
                          class="field-input"
                          prop:value=move || end_date.get()
                          on:input=move |ev| end_date.set(event_target_value(&ev))
                      />
                  </div>
              </div>
              <div class="field-row field-checks">
                  <label class="checkbox-label">
                      <input
                          type="checkbox"
                          prop:checked=move || ongoing.get()
                          on:change=move |ev| ongoing.set(event_target_checked(&ev))
                      />
                      " ยังใช้อยู่"
                  </label>
                  <label class="checkbox-label">
                      <input
                          type="checkbox"
                          prop:checked=move || start_unknown.get()
                          on:change=move |ev| start_unknown.set(event_target_checked(&ev))
                      />
                      " ไม่ทราบวันเริ่มใช้"
                  </label>
              </div>
              <div class="field-row">
                  <div class="field-group">
                      <label class="field-label">"ครั้งแรก (dose)"</label>
                      <input
                          type="text"
                          class="field-input"
                          placeholder="ยาครั้งแรก"
                          prop:value=move || first_dose.get()
                          on:input=move |ev| first_dose.set(event_target_value(&ev))
                      />
                  </div>
                  <div class="field-group">
                      <label class="field-label">"ครั้งสุดท้าย (dose)"</label>
                      <input
                          type="text"
                          class="field-input"
                          placeholder="ยาครั้งสุดท้าย"
                          prop:value=move || last_dose.get()
                          on:input=move |ev| last_dose.set(event_target_value(&ev))
                      />
                  </div>
              </div>
              <Show when=move || !error.get().is_empty()>
                  <p class="form-error">{move || error.get()}</p>
              </Show>
              <button type="submit" class="btn btn-primary">"+ เพิ่มยา"</button>
          </form>
      </section>
  }
}

fn fastrand_id() -> u64 {
  (js_sys::Math::random() * 1_000_000.0) as u64
}
