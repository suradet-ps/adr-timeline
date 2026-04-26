//!  file open, save, clear, and PDF export buttons.Toolbar

use crate::app::AppState;
use crate::models::AppData;
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

/// Toolbar with Save JSON, Load JSON, Clear, and Export PDF actions.
#[component]
pub(crate) fn Toolbar() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");

  // Save JSON
  let save_json = move |_| {
    let data = state.to_app_data();
    let _ = LocalStorage::set("savedHospitalName", &data.header.hospital_name);
    let _ = LocalStorage::set("savedPreparedBy", &data.header.prepared_by);

    match serde_json::to_string_pretty(&data) {
      Ok(json) => {
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&JsValue::from_str(&json));
        let opts = {
          let o = web_sys::BlobPropertyBag::new();
          o.set_type("application/json");
          o
        };
        if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &opts)
          && let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob)
          && let Ok(a) = web_sys::window()
            .expect("invariant: window exists")
            .document()
            .expect("invariant: document exists")
            .create_element("a")
        {
          let a: web_sys::HtmlAnchorElement = a.dyn_into().expect("invariant: cast anchor");
          a.set_href(&url);
          a.set_download("adr-timeline.json");
          a.click();
          let _ = web_sys::Url::revoke_object_url(&url);
        }
      }
      Err(e) => {
        web_sys::console::error_1(&JsValue::from_str(&format!("JSON serialize error: {e}")))
      }
    }
  };

  // Load JSON
  let file_input_ref = NodeRef::<leptos::html::Input>::new();

  let trigger_open = move |_| {
    if let Some(el) = file_input_ref.get() {
      el.click();
    }
  };

  let on_file_change = move |_ev: web_sys::Event| {
    let input: HtmlInputElement = file_input_ref.get().expect("invariant: file input mounted");
    if let Some(files) = input.files()
      && let Some(file) = files.get(0)
    {
      let gloo_file = gloo_file::File::from(file);
      spawn_local(async move {
        match gloo_file::futures::read_as_text(&gloo_file).await {
          Ok(text) => match serde_json::from_str::<AppData>(&text) {
            Ok(data) => state.load_app_data(data),
            Err(e) => {
              web_sys::console::error_1(&JsValue::from_str(&format!("JSON parse error: {e}")))
            }
          },
          Err(e) => {
            web_sys::console::error_1(&JsValue::from_str(&format!("File read error: {e:?}")))
          }
        }
      });
    }
  };

  // Clear
  let clear_all = move |_| {
    let window = web_sys::window().expect("invariant: window exists");
    if window
      .confirm_with_message("Clear all data?")
      .unwrap_or(false)
    {
      state.items.set(Vec::new());
      state.form.update(|f| {
        f.patient_name = String::new();
        f.patient_hn = String::new();
        f.patient_an = String::new();
        f.pharma_note = String::new();
      });
    }
  };

  // Export PDF
  let export_pdf = move |_| {
    let window = web_sys::window().expect("invariant: window exists");
    let document = window.document().expect("invariant: document exists");
    if let Some(canvas_el) = document.get_element_by_id("timeline-canvas") {
      let canvas: web_sys::HtmlCanvasElement =
        canvas_el.dyn_into().expect("invariant: element is canvas");
      export_pdf_from_canvas(&window, canvas);
    } else {
      web_sys::console::warn_1(&JsValue::from_str("timeline-canvas not found"));
    }
  };

  view! {
      <div class="toolbar">
          <input
              node_ref=file_input_ref
              type="file"
              accept=".json"
              style="display:none"
              on:change=on_file_change
          />
          <button class="btn btn-secondary" on:click=trigger_open>
              "Load JSON"
          </button>
          <button class="btn btn-secondary" on:click=save_json>
              "Save JSON"
          </button>
          <button class="btn btn-secondary" on:click=clear_all>
              "Clear"
          </button>
          <button class="btn btn-primary" on:click=export_pdf>
              "Export PDF"
          </button>
      </div>
  }
}

fn js_reflect_apply(
  func: &JsValue,
  this_val: &JsValue,
  args: &js_sys::Array,
) -> Result<JsValue, JsValue> {
  js_sys::Reflect::apply(func.unchecked_ref::<js_sys::Function>(), this_val, args)
}

fn export_pdf_from_canvas(window: &web_sys::Window, canvas: web_sys::HtmlCanvasElement) {
  let jspdf_ns =
    js_sys::Reflect::get(window, &JsValue::from_str("jspdf")).unwrap_or(JsValue::UNDEFINED);
  if jspdf_ns.is_undefined() {
    web_sys::console::error_1(&JsValue::from_str("jsPDF not loaded"));
    return;
  }
  let jspdf_ctor =
    js_sys::Reflect::get(&jspdf_ns, &JsValue::from_str("jsPDF")).unwrap_or(JsValue::UNDEFINED);
  if jspdf_ctor.is_undefined() {
    web_sys::console::error_1(&JsValue::from_str("window.jspdf.jsPDF not found"));
    return;
  }

  let canvas_h = canvas.height();
  let canvas_w = canvas.width();
  let page_h_px = (canvas_w as f64 * 794.0 / 1123.0) as u32;
  let total_pages = (canvas_h as f64 / page_h_px as f64).ceil() as u32;

  let format_arr = js_sys::Array::of2(&JsValue::from(1123.0_f64), &JsValue::from(794.0_f64));
  let args = js_sys::Array::of3(
    &JsValue::from_str("l"),
    &JsValue::from_str("px"),
    &format_arr,
  );
  let ctor: &js_sys::Function = jspdf_ctor.unchecked_ref();
  let pdf = js_sys::Reflect::construct(ctor, &args).unwrap_or(JsValue::UNDEFINED);
  if pdf.is_undefined() {
    web_sys::console::error_1(&JsValue::from_str("Failed to construct jsPDF"));
    return;
  }

  let document = window.document().expect("invariant: document exists");

  for page in 0..total_pages {
    if page > 0 {
      let add_page_fn =
        js_sys::Reflect::get(&pdf, &JsValue::from_str("addPage")).unwrap_or(JsValue::UNDEFINED);
      let _ = js_reflect_apply(&add_page_fn, &pdf, &js_sys::Array::new());
    }

    let off_canvas = document
      .create_element("canvas")
      .expect("invariant: createElement")
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .expect("invariant: canvas");
    off_canvas.set_width(canvas.width());
    off_canvas.set_height(page_h_px);

    let off_ctx: web_sys::CanvasRenderingContext2d = off_canvas
      .get_context("2d")
      .expect("invariant: getContext")
      .expect("invariant: 2d ctx")
      .dyn_into()
      .expect("invariant: cast ctx");

    off_ctx
      .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &canvas,
        0.0,
        (page * page_h_px) as f64,
        canvas.width() as f64,
        page_h_px as f64,
        0.0,
        0.0,
        canvas.width() as f64,
        page_h_px as f64,
      )
      .expect("invariant: drawImage");

    let data_url = off_canvas
      .to_data_url_with_type("image/png")
      .expect("invariant: toDataURL");

    let add_image_args = js_sys::Array::new();
    add_image_args.push(&JsValue::from_str(&data_url));
    add_image_args.push(&JsValue::from_str("PNG"));
    add_image_args.push(&JsValue::from(0.0_f64));
    add_image_args.push(&JsValue::from(0.0_f64));
    add_image_args.push(&JsValue::from(1123.0_f64));
    add_image_args.push(&JsValue::from(794.0_f64));

    let add_image_fn =
      js_sys::Reflect::get(&pdf, &JsValue::from_str("addImage")).unwrap_or(JsValue::UNDEFINED);
    let _ = js_reflect_apply(&add_image_fn, &pdf, &add_image_args);
  }

  let save_fn =
    js_sys::Reflect::get(&pdf, &JsValue::from_str("save")).unwrap_or(JsValue::UNDEFINED);
  let save_args = js_sys::Array::of1(&JsValue::from_str("adr-timeline.pdf"));
  let _ = js_reflect_apply(&save_fn, &pdf, &save_args);
}
