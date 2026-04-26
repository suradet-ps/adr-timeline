//! Naranjo Algorithm scoring modal.

use crate::app::AppState;
use leptos::prelude::*;

struct NaranjoQuestion {
  text: &'static str,
  yes: i32,
  no: i32,
}

const QUESTIONS: &[NaranjoQuestion] = &[
  NaranjoQuestion {
    text: "1. มีรายงานการเกิดอาการนี้จากยาชนิดนี้ก่อนหน้านี้หรือไม่?",
    yes: 1,
    no: 0,
  },
  NaranjoQuestion {
    text: "2. อาการไม่พึงประสงค์เกิดขึ้นหลังจากได้รับยาที่สงสัยหรือไม่?",
    yes: 2,
    no: -1,
  },
  NaranjoQuestion {
    text: "3. อาการดีขึ้นหลังหยุดยาหรือให้ยาแก้พิษหรือไม่?",
    yes: 1,
    no: 0,
  },
  NaranjoQuestion {
    text: "4. อาการกลับเป็นซ้ำเมื่อให้ยาซ้ำหรือไม่?",
    yes: 2,
    no: -1,
  },
  NaranjoQuestion {
    text: "5. มีสาเหตุอื่นที่อาจก่อให้เกิดอาการนี้ได้หรือไม่?",
    yes: -1,
    no: 2,
  },
  NaranjoQuestion {
    text: "6. อาการนี้เกิดขึ้นอีกเมื่อให้ยาหลอกหรือไม่?",
    yes: -1,
    no: 1,
  },
  NaranjoQuestion {
    text: "7. ระดับยาในเลือดมีค่าที่ทราบว่าเป็นพิษหรือไม่?",
    yes: 1,
    no: 0,
  },
  NaranjoQuestion {
    text: "8. ความรุนแรงของอาการเพิ่มขึ้นเมื่อเพิ่มขนาดยา หรือลดลงเมื่อลดขนาดยาหรือไม่?",
    yes: 1,
    no: 0,
  },
  NaranjoQuestion {
    text: "9. ผู้ป่วยเคยมีอาการคล้ายกันจากยาชนิดเดียวกันหรือยาที่คล้ายกันหรือไม่?",
    yes: 1,
    no: 0,
  },
  NaranjoQuestion {
    text: "10. อาการไม่พึงประสงค์ได้รับการยืนยันด้วยหลักฐานทางวัตถุประสงค์หรือไม่?",
    yes: 1,
    no: 0,
  },
];

fn naranjo_level(score: i32) -> &'static str {
  match score {
    s if s >= 9 => "Definite (แน่นอน)",
    s if s >= 5 => "Probable (น่าจะเป็น)",
    s if s >= 1 => "Possible (อาจเป็น)",
    _ => "Doubtful (สงสัยน้อย)",
  }
}

/// Naranjo Algorithm scoring modal component.
#[component]
pub(crate) fn NaranjoModal() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");
  // 0 = Unknown, 1 = Yes, 2 = No
  let answers: Vec<RwSignal<u8>> = (0..10).map(|_| RwSignal::new(0u8)).collect();
  let answers = StoredValue::new(answers);

  let score = move || {
    answers
      .get_value()
      .iter()
      .enumerate()
      .map(|(i, sig)| {
        let q = &QUESTIONS[i];
        match sig.get() {
          1 => q.yes,
          2 => q.no,
          _ => 0,
        }
      })
      .sum::<i32>()
  };

  let close = move |_| state.show_naranjo.set(false);

  view! {
      <Show when=move || state.show_naranjo.get()>
          <div class="modal-overlay" on:click=close>
              <div class="modal-box" on:click=|ev| ev.stop_propagation()>
                  <div class="modal-header">
                      <h2 class="modal-title">"Naranjo Algorithm"</h2>
                      <button class="modal-close" on:click=close>"✕"</button>
                  </div>
                  <div class="modal-body">
                      <table class="naranjo-table">
                          <thead>
                              <tr>
                                  <th>"คำถาม"</th>
                                  <th>"ใช่"</th>
                                  <th>"ไม่ใช่"</th>
                                  <th>"ไม่ทราบ"</th>
                              </tr>
                          </thead>
                          <tbody>
                              {answers.get_value().iter().enumerate().map(|(i, sig)| {
                                  let sig = *sig;
                                  let q = &QUESTIONS[i];
                                  view! {
                                      <tr>
                                          <td class="nara-q">{q.text}</td>
                                          <td class="nara-cell">
                                              <button
                                                  class=move || if sig.get() == 1 { "nara-btn active" } else { "nara-btn" }
                                                  on:click=move |_| sig.set(if sig.get() == 1 { 0 } else { 1 })
                                              >
                                                  {format!("+{}", QUESTIONS[i].yes)}
                                              </button>
                                          </td>
                                          <td class="nara-cell">
                                              <button
                                                  class=move || if sig.get() == 2 { "nara-btn active" } else { "nara-btn" }
                                                  on:click=move |_| sig.set(if sig.get() == 2 { 0 } else { 2 })
                                              >
                                                  {format!("{}", QUESTIONS[i].no)}
                                              </button>
                                          </td>
                                          <td class="nara-cell">
                                              <button
                                                  class=move || if sig.get() == 0 { "nara-btn active" } else { "nara-btn" }
                                                  on:click=move |_| sig.set(0)
                                              >"0"</button>
                                          </td>
                                      </tr>
                                  }
                              }).collect::<Vec<_>>()}
                          </tbody>
                      </table>
                      <div class="score-result">
                          <span class="score-label">"คะแนนรวม: "</span>
                          <span class="score-value">{score}</span>
                          <span class="score-level">" → " {move || naranjo_level(score())}</span>
                      </div>
                  </div>
              </div>
          </div>
      </Show>
  }
}
