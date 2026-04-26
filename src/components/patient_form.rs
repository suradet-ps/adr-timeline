//! Patient information form component.

use crate::app::AppState;
use leptos::prelude::*;

/// Form for entering patient and header information.
#[component]
pub(crate) fn PatientForm() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");
  let form = state.form;

  view! {
      <section class="form-card">
          <h2 class="form-card-title">"ข้อมูลผู้ป่วย"</h2>

          <div class="field-group">
              <label class="field-label">"ชื่อโรงพยาบาล"</label>
              <input
                  type="text"
                  class="field-input"
                  placeholder="ชื่อโรงพยาบาล"
                  prop:value=move || form.get().hospital_name
                  on:input=move |ev| form.update(|f| f.hospital_name = event_target_value(&ev))
              />
          </div>

          <div class="field-group">
              <label class="field-label">"ชื่อผู้ป่วย"</label>
              <input
                  type="text"
                  class="field-input"
                  placeholder="ชื่อ-นามสกุล"
                  prop:value=move || form.get().patient_name
                  on:input=move |ev| form.update(|f| f.patient_name = event_target_value(&ev))
              />
          </div>

          <div class="field-row">
              <div class="field-group">
                  <label class="field-label">"HN"</label>
                  <input
                      type="text"
                      class="field-input"
                      placeholder="HN"
                      prop:value=move || form.get().patient_hn
                      on:input=move |ev| form.update(|f| f.patient_hn = event_target_value(&ev))
                  />
              </div>
              <div class="field-group">
                  <label class="field-label">"AN"</label>
                  <input
                      type="text"
                      class="field-input"
                      placeholder="AN"
                      prop:value=move || form.get().patient_an
                      on:input=move |ev| form.update(|f| f.patient_an = event_target_value(&ev))
                  />
              </div>
          </div>

          <div class="field-group">
              <label class="field-label">"วันที่รายงาน"</label>
              <input
                  type="date"
                  class="field-input"
                  prop:value=move || form.get().report_date
                  on:input=move |ev| form.update(|f| f.report_date = event_target_value(&ev))
              />
          </div>

          <div class="field-group">
              <label class="field-label">"ผู้รายงาน"</label>
              <input
                  type="text"
                  class="field-input"
                  placeholder="ชื่อเภสัชกร"
                  prop:value=move || form.get().prepared_by
                  on:input=move |ev| form.update(|f| f.prepared_by = event_target_value(&ev))
              />
          </div>

          <div class="field-group">
              <label class="field-label">"บันทึกเภสัชกร"</label>
              <textarea
                  class="field-input field-textarea"
                  rows="3"
                  placeholder="บันทึกเพิ่มเติม"
                  prop:value=move || form.get().pharma_note
                  on:input=move |ev| form.update(|f| f.pharma_note = event_target_value(&ev))
              />
          </div>
      </section>
  }
}
