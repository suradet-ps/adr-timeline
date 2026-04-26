//! DRESS RegiSCAR scoring modal.

use crate::app::AppState;
use leptos::prelude::*;

struct DressGroup {
  label: &'static str,
  options: &'static [(&'static str, i32)],
  default_val: i32,
}

const GROUPS: &[DressGroup] = &[
  DressGroup {
    label: "Fever ≥ 38.5°C",
    options: &[("ไม่มี", -1), ("มี", 1)],
    default_val: -1,
  },
  DressGroup {
    label: "Enlarged lymph nodes (≥2 sites, >1 cm)",
    options: &[("ไม่มี", 0), ("มี", 1)],
    default_val: 0,
  },
  DressGroup {
    label: "Atypical lymphocytes",
    options: &[("ไม่มี", 0), ("มี", 1)],
    default_val: 0,
  },
  DressGroup {
    label: "Skin rash extent > 50%",
    options: &[("< 50%", 0), ("≥ 50%", 1)],
    default_val: 0,
  },
  DressGroup {
    label: "Skin rash — suggestive DRESS",
    options: &[("ไม่ใช่", 0), ("ใช่", 1)],
    default_val: 0,
  },
  DressGroup {
    label: "Biopsy — suggestive DRESS",
    options: &[("ไม่ใช่", -1), ("ใช่", 1)],
    default_val: -1,
  },
  DressGroup {
    label: "Eosinophilia",
    options: &[("ไม่มี", -1), ("700–1499/mm³", 1), ("≥ 1500/mm³", 2)],
    default_val: -1,
  },
  DressGroup {
    label: "Atypical lymphocytes (%)",
    options: &[("ไม่มี", 0), ("มี", 1)],
    default_val: 0,
  },
  DressGroup {
    label: "Liver/kidney/lung/muscle involvement",
    options: &[("ไม่มี", -1), ("1 อวัยวะ", 1), ("≥ 2 อวัยวะ", 2)],
    default_val: -1,
  },
  DressGroup {
    label: "Resolution > 15 days",
    options: &[("ไม่ใช่", 0), ("ใช่", 1)],
    default_val: 0,
  },
];

fn dress_level(score: i32) -> &'static str {
  match score {
    s if s >= 6 => "Definite DRESS",
    s if s >= 4 => "Probable DRESS",
    s if s >= 2 => "Possible DRESS",
    _ => "No case",
  }
}

/// DRESS RegiSCAR scoring modal.
#[component]
pub(crate) fn DressModal() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");
  let selections: Vec<RwSignal<i32>> = GROUPS
    .iter()
    .map(|g| RwSignal::new(g.default_val))
    .collect();
  let selections = StoredValue::new(selections);

  let score = move || selections.get_value().iter().map(|s| s.get()).sum::<i32>();

  let close = move |_| state.show_dress.set(false);

  view! {
      <Show when=move || state.show_dress.get()>
          <div class="modal-overlay" on:click=close>
              <div class="modal-box modal-box-wide" on:click=|ev| ev.stop_propagation()>
                  <div class="modal-header">
                      <h2 class="modal-title">"DRESS RegiSCAR Score"</h2>
                      <button class="modal-close" on:click=close>"✕"</button>
                  </div>
                  <div class="modal-body">
                      {selections.get_value().iter().enumerate().map(|(i, sel)| {
                          let sel = *sel;
                          let group = &GROUPS[i];
                          view! {
                              <div class="dress-group">
                                  <p class="dress-q">{group.label}</p>
                                  <div class="dress-btn-group">
                                      {group.options.iter().map(|(label, val)| {
                                          let v = *val;
                                          view! {
                                              <button
                                                  class=move || if sel.get() == v { "dress-btn active" } else { "dress-btn" }
                                                  on:click=move |_| sel.set(v)
                                              >
                                                  {format!("{} ({}{})", label, if v >= 0 { "+" } else { "" }, v)}
                                              </button>
                                          }
                                      }).collect::<Vec<_>>()}
                                  </div>
                              </div>
                          }
                      }).collect::<Vec<_>>()}
                      <div class="score-result">
                          <span class="score-label">"คะแนนรวม: "</span>
                          <span class="score-value">{score}</span>
                          <span class="score-level">" → " {move || dress_level(score())}</span>
                      </div>
                  </div>
              </div>
          </div>
      </Show>
  }
}
