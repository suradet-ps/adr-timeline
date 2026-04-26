//! Adverse reaction form component.

use crate::app::AppState;
use crate::models::{Item, ItemType};
use crate::utils::today_iso;
use leptos::prelude::*;

/// Form for recording an adverse drug reaction event.
#[component]
pub(crate) fn ReactionForm() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");

  let reaction_name = RwSignal::new(String::new());
  let onset_date = RwSignal::new(today_iso());
  let error = RwSignal::new(String::new());

  let submit = move |ev: leptos::ev::SubmitEvent| {
    ev.prevent_default();
    let name = reaction_name.get();
    if name.trim().is_empty() {
      error.set("กรุณากรอกชื่ออาการ".to_string());
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
        let s = onset_date.get();
        if s.is_empty() { None } else { Some(s) }
      },
      end: None,
      ongoing: false,
      start_unknown: false,
      item_type: ItemType::Reaction,
      first_dose: String::new(),
      last_dose: String::new(),
    };
    state.items.update(|v| v.push(item));
    reaction_name.set(String::new());
    onset_date.set(today_iso());
    error.set(String::new());
  };

  view! {
      <section class="form-card">
          <h2 class="form-card-title reaction-title">"เพิ่มอาการไม่พึงประสงค์"</h2>
          <form on:submit=submit>
              <div class="field-group">
                  <label class="field-label">"ชื่ออาการ *"</label>
                  <input
                      type="text"
                      class="field-input"
                      placeholder="เช่น ผื่น, คลื่นไส้"
                      prop:value=move || reaction_name.get()
                      on:input=move |ev| { reaction_name.set(event_target_value(&ev)); error.set(String::new()); }
                  />
              </div>
              <div class="field-group">
                  <label class="field-label">"วันเกิดอาการ"</label>
                  <input
                      type="date"
                      class="field-input"
                      prop:value=move || onset_date.get()
                      on:input=move |ev| onset_date.set(event_target_value(&ev))
                  />
              </div>
              <Show when=move || !error.get().is_empty()>
                  <p class="form-error">{move || error.get()}</p>
              </Show>
              <button type="submit" class="btn btn-reaction">"+ เพิ่มอาการ"</button>
          </form>
      </section>
  }
}
