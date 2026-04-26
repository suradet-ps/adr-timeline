//! BSA (Lund-Browder) body surface area modal.

use crate::app::AppState;
use leptos::prelude::*;

struct BodyPart {
  label: &'static str,
  pct: f64,
}

const FRONT_PARTS: &[BodyPart] = &[
  BodyPart {
    label: "ศีรษะ (หน้า)",
    pct: 4.5,
  },
  BodyPart {
    label: "คอ (หน้า)",
    pct: 1.0,
  },
  BodyPart {
    label: "หน้าอก",
    pct: 9.0,
  },
  BodyPart {
    label: "ท้อง",
    pct: 9.0,
  },
  BodyPart {
    label: "แขนซ้าย (หน้า)",
    pct: 2.0,
  },
  BodyPart {
    label: "แขนขวา (หน้า)",
    pct: 2.0,
  },
  BodyPart {
    label: "แขนท่อนล่างซ้าย",
    pct: 1.5,
  },
  BodyPart {
    label: "แขนท่อนล่างขวา",
    pct: 1.5,
  },
  BodyPart {
    label: "มือซ้าย",
    pct: 1.25,
  },
  BodyPart {
    label: "มือขวา",
    pct: 1.25,
  },
  BodyPart {
    label: "อวัยวะสืบพันธุ์",
    pct: 1.0,
  },
  BodyPart {
    label: "ต้นขาซ้าย (หน้า)",
    pct: 4.75,
  },
  BodyPart {
    label: "ต้นขาขวา (หน้า)",
    pct: 4.75,
  },
  BodyPart {
    label: "แข้งซ้าย (หน้า)",
    pct: 3.5,
  },
  BodyPart {
    label: "แข้งขวา (หน้า)",
    pct: 3.5,
  },
  BodyPart {
    label: "เท้าซ้าย",
    pct: 1.75,
  },
  BodyPart {
    label: "เท้าขวา",
    pct: 1.75,
  },
];

const BACK_PARTS: &[BodyPart] = &[
  BodyPart {
    label: "ศีรษะ (หลัง)",
    pct: 4.5,
  },
  BodyPart {
    label: "คอ (หลัง)",
    pct: 1.0,
  },
  BodyPart {
    label: "หลังส่วนบน",
    pct: 9.0,
  },
  BodyPart {
    label: "หลังส่วนล่าง",
    pct: 9.0,
  },
  BodyPart {
    label: "แขนซ้าย (หลัง)",
    pct: 2.0,
  },
  BodyPart {
    label: "แขนขวา (หลัง)",
    pct: 2.0,
  },
  BodyPart {
    label: "ก้นซ้าย",
    pct: 2.5,
  },
  BodyPart {
    label: "ก้นขวา",
    pct: 2.5,
  },
  BodyPart {
    label: "ต้นขาซ้าย (หลัง)",
    pct: 4.75,
  },
  BodyPart {
    label: "ต้นขาขวา (หลัง)",
    pct: 4.75,
  },
  BodyPart {
    label: "แข้งซ้าย (หลัง)",
    pct: 3.5,
  },
  BodyPart {
    label: "แข้งขวา (หลัง)",
    pct: 3.5,
  },
];

fn bsa_level(pct: f64) -> &'static str {
  if pct >= 30.0 {
    "TEN (≥ 30%)"
  } else if pct >= 10.0 {
    "SJS/TEN Overlap (10–30%)"
  } else {
    "SJS (< 10%)"
  }
}

/// BSA Lund-Browder body surface area modal.
#[component]
pub(crate) fn BsaModal() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");
  // Each body part: checked or not
  let front_checks: Vec<RwSignal<bool>> =
    FRONT_PARTS.iter().map(|_| RwSignal::new(false)).collect();
  let back_checks: Vec<RwSignal<bool>> = BACK_PARTS.iter().map(|_| RwSignal::new(false)).collect();
  let front_checks = StoredValue::new(front_checks);
  let back_checks = StoredValue::new(back_checks);

  let total_pct = move || {
    let f: f64 = front_checks
      .get_value()
      .iter()
      .enumerate()
      .map(|(i, s)| if s.get() { FRONT_PARTS[i].pct } else { 0.0 })
      .sum();
    let b: f64 = back_checks
      .get_value()
      .iter()
      .enumerate()
      .map(|(i, s)| if s.get() { BACK_PARTS[i].pct } else { 0.0 })
      .sum();
    f + b
  };

  let close = move |_| state.show_bsa.set(false);

  view! {
      <Show when=move || state.show_bsa.get()>
          <div class="modal-overlay" on:click=close>
              <div class="modal-box modal-box-wide" on:click=|ev| ev.stop_propagation()>
                  <div class="modal-header">
                      <h2 class="modal-title">"BSA Estimation (Lund-Browder)"</h2>
                      <button class="modal-close" on:click=close>"✕"</button>
                  </div>
                  <div class="modal-body">
                      <div class="bsa-columns">
                          <div class="bsa-col">
                              <h4 class="bsa-col-title">"ด้านหน้า"</h4>
                              {front_checks.get_value().iter().enumerate().map(|(i, sig)| {
                                  let sig = *sig;
                                  let part = &FRONT_PARTS[i];
                                  view! {
                                      <label class="bsa-part">
                                          <input
                                              type="checkbox"
                                              prop:checked=move || sig.get()
                                              on:change=move |ev| sig.set(event_target_checked(&ev))
                                          />
                                          {format!(" {} ({:.2}%)", part.label, part.pct)}
                                      </label>
                                  }
                              }).collect::<Vec<_>>()}
                          </div>
                          <div class="bsa-col">
                              <h4 class="bsa-col-title">"ด้านหลัง"</h4>
                              {back_checks.get_value().iter().enumerate().map(|(i, sig)| {
                                  let sig = *sig;
                                  let part = &BACK_PARTS[i];
                                  view! {
                                      <label class="bsa-part">
                                          <input
                                              type="checkbox"
                                              prop:checked=move || sig.get()
                                              on:change=move |ev| sig.set(event_target_checked(&ev))
                                          />
                                          {format!(" {} ({:.2}%)", part.label, part.pct)}
                                      </label>
                                  }
                              }).collect::<Vec<_>>()}
                          </div>
                      </div>
                      <div class="score-result">
                          <span class="score-label">"พื้นที่ผิวที่ได้รับผลกระทบ: "</span>
                          <span class="score-value">{move || format!("{:.2}%", total_pct())}</span>
                          <span class="score-level">" → " {move || bsa_level(total_pct())}</span>
                      </div>
                  </div>
              </div>
          </div>
      </Show>
  }
}
