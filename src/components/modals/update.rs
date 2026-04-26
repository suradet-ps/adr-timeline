//! Update/Changelog modal.

use crate::app::AppState;
use leptos::prelude::*;

/// Modal showing application version history.
#[component]
pub(crate) fn UpdateModal() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");
  let close = move |_| state.show_update.set(false);

  view! {
      <Show when=move || state.show_update.get()>
          <div class="modal-overlay" on:click=close>
              <div class="modal-box" on:click=|ev| ev.stop_propagation()>
                  <div class="modal-header">
                      <h2 class="modal-title">"Changelog"</h2>
                      <button class="modal-close" on:click=close>"✕"</button>
                  </div>
                  <div class="modal-body update-body">
                      <h3>"v1.2.0 — Rust/Leptos Migration"</h3>
                      <ul>
                          <li>"Migration to Rust + Leptos 0.8 + WASM"</li>
                          <li>"Warm parchment design system (DESIGN.md)"</li>
                          <li>"Canvas-based A4 timeline rendering"</li>
                          <li>"Naranjo, DRESS RegiSCAR, BSA Lund-Browder modals"</li>
                          <li>"JSON save/load, PDF export via jsPDF"</li>
                      </ul>
                      <h3>"v1.1.0"</h3>
                      <ul>
                          <li>"เพิ่มโหมดพิมพ์ PDF หลายหน้า"</li>
                          <li>"เพิ่มการคำนวณ DRESS RegiSCAR"</li>
                      </ul>
                      <h3>"v1.0.0"</h3>
                      <ul>
                          <li>"เวอร์ชันแรก — HTML/JS/CSS"</li>
                      </ul>
                  </div>
              </div>
          </div>
      </Show>
  }
}
