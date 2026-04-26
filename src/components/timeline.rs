//! Timeline canvas component — renders the ADR timeline via HTML5 Canvas.

use crate::app::AppState;
use crate::canvas::draw_timeline;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Canvas element that re-draws whenever items or form data change.
#[component]
pub(crate) fn TimelineCanvas() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");
  let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

  Effect::new(move |_| {
    let items = state.items.get();
    let form = state.form.get();
    let data = crate::models::AppData {
      header: form,
      items,
    };
    if let Some(el) = canvas_ref.get() {
      let canvas: web_sys::HtmlCanvasElement =
        (*el).clone().dyn_into().expect("invariant: canvas element");
      draw_timeline(&canvas, &data);
    }
  });

  view! {
      <div class="canvas-wrap">
          <canvas
              id="timeline-canvas"
              node_ref=canvas_ref
              class="timeline-canvas"
          />
      </div>
  }
}
