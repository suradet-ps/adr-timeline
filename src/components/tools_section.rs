//! Tools section — buttons to open Naranjo, DRESS, and BSA modals.

use crate::app::AppState;
use leptos::prelude::*;

/// Section with buttons that open scoring/calculation modals.
#[component]
pub(crate) fn ToolsSection() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");

  view! {
      <section class="tools-section">
          <h3 class="tools-title">"เครื่องมือประเมิน"</h3>
          <div class="tools-grid">
              <button
                  class="btn btn-tool"
                  on:click=move |_| state.show_naranjo.set(true)
              >
                  <span class="tool-icon">"🧮"</span>
                  <span class="tool-label">"Naranjo Algorithm"</span>
                  <span class="tool-desc">"ประเมินความสัมพันธ์ ADR"</span>
              </button>
              <button
                  class="btn btn-tool"
                  on:click=move |_| state.show_dress.set(true)
              >
                  <span class="tool-icon">"📋"</span>
                  <span class="tool-label">"DRESS RegiSCAR"</span>
                  <span class="tool-desc">"ประเมิน DRESS syndrome"</span>
              </button>
              <button
                  class="btn btn-tool"
                  on:click=move |_| state.show_bsa.set(true)
              >
                  <span class="tool-icon">"🫀"</span>
                  <span class="tool-label">"BSA (Lund-Browder)"</span>
                  <span class="tool-desc">"ประเมินพื้นที่ผิวหนัง"</span>
              </button>
              <button
                  class="btn btn-tool"
                  on:click=move |_| state.show_update.set(true)
              >
                  <span class="tool-icon">"📝"</span>
                  <span class="tool-label">"Changelog"</span>
                  <span class="tool-desc">"ประวัติการอัปเดต"</span>
              </button>
          </div>
      </section>
  }
}
