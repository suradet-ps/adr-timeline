//! Canvas timeline rendering — port of the JS `drawTimeline` function.
#![allow(clippy::too_many_arguments)]

use crate::models::{AppData, Item, ItemType};
use crate::utils::format_short_thai;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const A4_WIDTH: f64 = 1123.0;
const A4_HEIGHT: f64 = 794.0;
const DPR: f64 = 4.0;
const DATES_PER_PAGE: usize = 12;
const P_LEFT: f64 = 260.0;
const P_RIGHT: f64 = 100.0;

const COLOR_DRUG: &str = "#3b82f6";
const COLOR_REACTION: &str = "#ef4444";
const COLOR_LAB: &str = "#8b5cf6";
const COLOR_TEXT: &str = "#1e293b";
const COLOR_GRID: &str = "#e2e8f0";
const COLOR_BG: &str = "#ffffff";
const COLOR_HEADER_BG: &str = "#f8fafc";
const COLOR_BORDER: &str = "#cbd5e1";

/// Measure the rendered width of a string on the canvas.
fn measure_text(ctx: &CanvasRenderingContext2d, text: &str) -> f64 {
  ctx.measure_text(text).map(|m| m.width()).unwrap_or(0.0)
}

/// Draw a small circle symbol at (x, y) in the given color.
fn draw_circle(ctx: &CanvasRenderingContext2d, x: f64, y: f64, filled: bool, color: &str) {
  ctx.begin_path();
  ctx
    .arc(x, y, 5.0, 0.0, std::f64::consts::TAU)
    .expect("invariant: arc params valid");
  ctx.set_fill_style_str(COLOR_BG);
  ctx.fill();
  ctx.set_stroke_style_str(color);
  ctx.set_line_width(1.5);
  ctx.stroke();
  if filled {
    ctx.begin_path();
    ctx
      .arc(x, y, 2.5, 0.0, std::f64::consts::TAU)
      .expect("invariant: arc params valid");
    ctx.set_fill_style_str(color);
    ctx.fill();
  }
}

/// Draw a right-pointing arrow at (x, y).
fn draw_arrow_right(ctx: &CanvasRenderingContext2d, x: f64, y: f64, color: &str) {
  ctx.begin_path();
  ctx.move_to(x, y - 5.0);
  ctx.line_to(x + 8.0, y);
  ctx.line_to(x, y + 5.0);
  ctx.close_path();
  ctx.set_fill_style_str(color);
  ctx.fill();
}

/// Draw a left-pointing arrow at (x, y).
fn draw_arrow_left(ctx: &CanvasRenderingContext2d, x: f64, y: f64, color: &str) {
  ctx.begin_path();
  ctx.move_to(x, y - 5.0);
  ctx.line_to(x - 8.0, y);
  ctx.line_to(x, y + 5.0);
  ctx.close_path();
  ctx.set_fill_style_str(color);
  ctx.fill();
}

/// Draw a downward-pointing arrow at (x, y).
fn draw_arrow_down(ctx: &CanvasRenderingContext2d, x: f64, y: f64, color: &str) {
  ctx.begin_path();
  ctx.move_to(x - 5.0, y);
  ctx.line_to(x + 5.0, y);
  ctx.line_to(x, y + 8.0);
  ctx.close_path();
  ctx.set_fill_style_str(color);
  ctx.fill();
}

/// Word-wrap text into lines that fit within `max_width`.
/// Returns the resulting lines. When `simulate` is true only measures.
fn wrap_text(
  ctx: &CanvasRenderingContext2d,
  text: &str,
  max_width: f64,
  simulate: bool,
) -> Vec<String> {
  let mut lines: Vec<String> = Vec::new();
  // Split on explicit newlines first
  for paragraph in text.split('\n') {
    let words: Vec<&str> = paragraph.split_whitespace().collect();
    if words.is_empty() {
      lines.push(String::new());
      continue;
    }
    let mut current = String::new();
    for word in words {
      let test = if current.is_empty() {
        word.to_string()
      } else {
        format!("{current} {word}")
      };
      let w = if simulate {
        test.len() as f64 * 7.0 // rough estimate when simulating
      } else {
        measure_text(ctx, &test)
      };
      if w > max_width && !current.is_empty() {
        lines.push(current);
        current = word.to_string();
      } else {
        current = test;
      }
    }
    if !current.is_empty() {
      lines.push(current);
    }
  }
  lines
}

/// Render multi-line text at (x, y) with the given line height, returning total height.
fn draw_wrapped_text(
  ctx: &CanvasRenderingContext2d,
  text: &str,
  x: f64,
  y: f64,
  max_width: f64,
  line_height: f64,
) -> f64 {
  let lines = wrap_text(ctx, text, max_width, false);
  let n = lines.len() as f64;
  for (i, line) in lines.iter().enumerate() {
    ctx
      .fill_text(line, x, y + i as f64 * line_height)
      .expect("invariant: fill_text coords valid");
  }
  n * line_height
}

/// Estimate the pixel height of wrapped text without drawing.
#[allow(dead_code)]
fn estimate_text_height(text: &str, max_width: f64, line_height: f64, font_size: f64) -> f64 {
  let chars_per_line = (max_width / (font_size * 0.55)).max(1.0) as usize;
  let mut lines = 0usize;
  for paragraph in text.split('\n') {
    let words: Vec<&str> = paragraph.split_whitespace().collect();
    if words.is_empty() {
      lines += 1;
      continue;
    }
    let mut line_chars = 0usize;
    let mut first = true;
    for word in &words {
      let wlen = word.len() + if first { 0 } else { 1 };
      if line_chars + wlen > chars_per_line && !first {
        lines += 1;
        line_chars = word.len();
      } else {
        line_chars += wlen;
        first = false;
      }
    }
    lines += 1;
  }
  lines as f64 * line_height
}

/// Parse "YYYY-MM-DD" → days-since-epoch (approximate, for ordering).
fn date_to_days(s: &str) -> i64 {
  let p: Vec<&str> = s.split('-').collect();
  if p.len() != 3 {
    return 0;
  }
  let y: i64 = p[0].parse().unwrap_or(0);
  let m: i64 = p[1].parse().unwrap_or(1);
  let d: i64 = p[2].parse().unwrap_or(1);
  y * 365 + m * 30 + d
}

/// Collect all unique sorted dates from items.
fn collect_dates(items: &[Item]) -> Vec<String> {
  let mut dates: Vec<String> = Vec::new();
  for it in items {
    if let Some(s) = &it.start
      && !s.is_empty()
      && !dates.contains(s)
    {
      dates.push(s.clone());
    }
    if let Some(e) = &it.end
      && !e.is_empty()
      && !dates.contains(e)
    {
      dates.push(e.clone());
    }
  }
  dates.sort_by_key(|d| date_to_days(d));
  dates.dedup();
  dates
}

struct PageLayout {
  scale: f64,
  row_height: f64,
  total_pages: usize,
  page_dates: Vec<Vec<String>>,
}

fn compute_layout(items: &[Item], all_dates: &[String]) -> PageLayout {
  let page_count = all_dates.len().max(1).div_ceil(DATES_PER_PAGE);
  let page_dates: Vec<Vec<String>> = (0..page_count)
    .map(|p| {
      let start = p * DATES_PER_PAGE;
      let end = ((p + 1) * DATES_PER_PAGE).min(all_dates.len());
      all_dates[start..end].to_vec()
    })
    .collect();

  // Auto-scale: shrink until all rows fit in pageContentMaxH
  let mut scale = 1.0_f64;
  let base_row_height = 28.0_f64;
  loop {
    let row_height = base_row_height * scale;
    let header_h = 110.0 * scale;
    let summary_h = 60.0 * scale;
    let content_max_h = A4_HEIGHT - 150.0 * scale - 40.0;
    let rows_needed = items.len() as f64;
    let content_h = header_h + rows_needed * row_height + summary_h;
    if content_h <= content_max_h || scale <= 0.5 {
      break;
    }
    scale -= 0.05;
  }

  PageLayout {
    scale,
    row_height: base_row_height * scale,
    total_pages: page_count.max(1),
    page_dates,
  }
}

/// Returns the x position of a date on the current page.
fn date_x(date: &str, page_dates: &[String], track_w: f64) -> Option<f64> {
  let n = page_dates.len();
  if n == 0 {
    return None;
  }
  let idx = page_dates.iter().position(|d| d == date)?;
  let spacing = track_w / (DATES_PER_PAGE as f64 + 1.0);
  Some(P_LEFT + spacing * (idx as f64 + 1.0))
}

fn draw_page(
  ctx: &CanvasRenderingContext2d,
  data: &AppData,
  items: &[Item],
  layout: &PageLayout,
  page_idx: usize,
  is_last: bool,
) {
  let s = layout.scale;
  let page_dates = &layout.page_dates[page_idx];
  let track_w = A4_WIDTH - P_LEFT - P_RIGHT;
  let spacing = track_w / (DATES_PER_PAGE as f64 + 1.0);

  let page_top = page_idx as f64 * A4_HEIGHT;

  // ── Background ─────────────────────────────────────────────────────────
  ctx.set_fill_style_str(COLOR_BG);
  ctx.fill_rect(0.0, page_top, A4_WIDTH, A4_HEIGHT);

  // ── Header block ───────────────────────────────────────────────────────
  ctx.set_fill_style_str(COLOR_HEADER_BG);
  ctx.fill_rect(0.0, page_top, A4_WIDTH, 90.0 * s);

  ctx.set_fill_style_str(COLOR_TEXT);
  ctx.set_font(&format!("bold {:.0}px Georgia, serif", 16.0 * s));
  let hospital = if data.header.hospital_name.is_empty() {
    "โรงพยาบาล"
  } else {
    &data.header.hospital_name
  };
  ctx
    .fill_text(hospital, 20.0, page_top + 22.0 * s)
    .expect("invariant: fill_text");

  ctx.set_font(&format!(
    "{:.0}px 'Sarabun', system-ui, sans-serif",
    11.0 * s
  ));
  ctx.set_fill_style_str("#475569");

  let patient_line = format!(
    "ผู้ป่วย: {}   HN: {}   AN: {}   วันที่รายงาน: {}   ผู้รายงาน: {}",
    data.header.patient_name,
    data.header.patient_hn,
    data.header.patient_an,
    data.header.report_date,
    data.header.prepared_by
  );
  ctx
    .fill_text(&patient_line, 20.0, page_top + 40.0 * s)
    .expect("invariant: fill_text");

  // Legend
  let legend_x = A4_WIDTH - 320.0 * s;
  let legend_y = page_top + 20.0 * s;
  ctx.set_font(&format!(
    "{:.0}px 'Sarabun', system-ui, sans-serif",
    10.0 * s
  ));
  for (i, (label, color)) in [
    ("ยา", COLOR_DRUG),
    ("อาการไม่พึงประสงค์", COLOR_REACTION),
    ("ผลแล็บ", COLOR_LAB),
  ]
  .iter()
  .enumerate()
  {
    let lx = legend_x + i as f64 * 100.0 * s;
    ctx.set_fill_style_str(color);
    ctx.fill_rect(lx, legend_y, 12.0 * s, 8.0 * s);
    ctx.set_fill_style_str(COLOR_TEXT);
    ctx
      .fill_text(label, lx + 15.0 * s, legend_y + 9.0 * s)
      .expect("invariant: fill_text");
  }

  // Divider
  ctx.set_stroke_style_str(COLOR_BORDER);
  ctx.set_line_width(1.0);
  ctx.begin_path();
  ctx.move_to(0.0, page_top + 90.0 * s);
  ctx.line_to(A4_WIDTH, page_top + 90.0 * s);
  ctx.stroke();

  // ── Date axis ──────────────────────────────────────────────────────────
  let axis_y = page_top + 100.0 * s;
  ctx.set_font(&format!(
    "{:.0}px 'Sarabun', system-ui, sans-serif",
    9.0 * s
  ));
  ctx.set_fill_style_str("#64748b");

  // Column headers
  for (i, date) in page_dates.iter().enumerate() {
    let x = P_LEFT + spacing * (i as f64 + 1.0);
    ctx.set_text_align("center");
    ctx
      .fill_text(&format_short_thai(date), x, axis_y)
      .expect("invariant: fill_text");

    // Dashed vertical grid line
    ctx.set_stroke_style_str(COLOR_GRID);
    ctx.set_line_width(1.0);
    let _ = ctx.set_line_dash(&js_sys::Array::of2(
      &wasm_bindgen::JsValue::from(3.0_f64),
      &wasm_bindgen::JsValue::from(3.0_f64),
    ));
    ctx.begin_path();
    ctx.move_to(x, axis_y + 4.0);
    ctx.line_to(x, page_top + A4_HEIGHT - 30.0 * s);
    ctx.stroke();
    let _ = ctx.set_line_dash(&js_sys::Array::new());
  }
  ctx.set_text_align("left");

  // ── Item rows ──────────────────────────────────────────────────────────
  let rows_start_y = axis_y + 16.0 * s;

  for (row_idx, item) in items.iter().enumerate() {
    let ry = rows_start_y + row_idx as f64 * layout.row_height;

    // Zebra stripe
    if row_idx % 2 == 0 {
      ctx.set_fill_style_str("rgba(248,250,252,0.6)");
      ctx.fill_rect(0.0, ry, A4_WIDTH, layout.row_height);
    }

    let color = match item.item_type {
      ItemType::Drug => COLOR_DRUG,
      ItemType::Reaction => COLOR_REACTION,
      ItemType::Lab => COLOR_LAB,
    };

    // Row label
    ctx.set_fill_style_str(COLOR_TEXT);
    ctx.set_font(&format!(
      "{:.0}px 'Sarabun', system-ui, sans-serif",
      10.0 * s
    ));
    ctx.set_text_align("right");
    let label = if item.name.len() > 30 {
      format!("{}…", &item.name[..28])
    } else {
      item.name.clone()
    };
    ctx
      .fill_text(&label, P_LEFT - 6.0, ry + layout.row_height * 0.6)
      .expect("invariant: fill_text");
    ctx.set_text_align("left");

    // Determine horizontal span on this page
    let start_x = item
      .start
      .as_deref()
      .and_then(|s| if s.is_empty() { None } else { Some(s) })
      .and_then(|s| date_x(s, page_dates, track_w));

    let end_x = item
      .end
      .as_deref()
      .and_then(|e| if e.is_empty() { None } else { Some(e) })
      .and_then(|e| date_x(e, page_dates, track_w));

    let mid_y = ry + layout.row_height * 0.5;

    match item.item_type {
      ItemType::Reaction | ItemType::Lab => {
        // Point events — just draw arrow down at start date
        if let Some(sx) = start_x {
          draw_arrow_down(ctx, sx, mid_y, color);
        }
      }
      ItemType::Drug => {
        // Determine left and right edge x values
        let left_x = if item.start_unknown || item.start.is_none() {
          // arrow left at P_LEFT
          None
        } else {
          start_x
        };
        let right_x = if item.ongoing || item.end.is_none() {
          // arrow right at right edge of track
          None
        } else {
          end_x
        };

        let line_x1 = left_x.unwrap_or(P_LEFT);
        let line_x2 = right_x.unwrap_or(A4_WIDTH - P_RIGHT);

        if line_x1 < line_x2 {
          ctx.set_stroke_style_str(color);
          ctx.set_line_width(2.0 * s);
          let _ = ctx.set_line_dash(&js_sys::Array::new());
          ctx.begin_path();
          ctx.move_to(line_x1, mid_y);
          ctx.line_to(line_x2, mid_y);
          ctx.stroke();
        }

        if item.start_unknown || item.start.is_none() {
          draw_arrow_left(ctx, P_LEFT, mid_y, color);
        } else if let Some(sx) = start_x {
          draw_circle(ctx, sx, mid_y, false, color);
        }

        if item.ongoing || item.end.is_none() {
          draw_arrow_right(ctx, A4_WIDTH - P_RIGHT, mid_y, color);
        } else if let Some(ex) = end_x {
          draw_circle(ctx, ex, mid_y, true, color);
        }
      }
    }
  }

  // ── Summary / pharma note on last page ─────────────────────────────────
  if is_last && !data.header.pharma_note.is_empty() {
    let note_y = rows_start_y + items.len() as f64 * layout.row_height + 10.0 * s;
    ctx.set_stroke_style_str(COLOR_BORDER);
    ctx.set_line_width(1.0);
    ctx.begin_path();
    ctx.move_to(20.0, note_y);
    ctx.line_to(A4_WIDTH - 20.0, note_y);
    ctx.stroke();

    ctx.set_font(&format!(
      "bold {:.0}px 'Sarabun', system-ui, sans-serif",
      10.0 * s
    ));
    ctx.set_fill_style_str(COLOR_TEXT);
    ctx
      .fill_text("บันทึกเภสัชกร:", 20.0, note_y + 16.0 * s)
      .expect("invariant: fill_text");

    ctx.set_font(&format!(
      "{:.0}px 'Sarabun', system-ui, sans-serif",
      10.0 * s
    ));
    draw_wrapped_text(
      ctx,
      &data.header.pharma_note,
      20.0,
      note_y + 30.0 * s,
      A4_WIDTH - 40.0,
      14.0 * s,
    );
  }

  // Page number
  ctx.set_font(&format!(
    "{:.0}px 'Sarabun', system-ui, sans-serif",
    9.0 * s
  ));
  ctx.set_fill_style_str("#94a3b8");
  ctx.set_text_align("center");
  ctx
    .fill_text(
      &format!("หน้า {} / {}", page_idx + 1, layout.total_pages),
      A4_WIDTH / 2.0,
      page_top + A4_HEIGHT - 10.0,
    )
    .expect("invariant: fill_text");
  ctx.set_text_align("left");
}

/// Render the full timeline onto the given canvas element.
pub(crate) fn draw_timeline(canvas: &HtmlCanvasElement, data: &AppData) {
  let items = &data.items;
  let all_dates = collect_dates(items);

  let layout = compute_layout(items, &all_dates);

  let canvas_w = (A4_WIDTH * DPR) as u32;
  let canvas_h = (A4_HEIGHT * DPR * layout.total_pages as f64) as u32;
  canvas.set_width(canvas_w);
  canvas.set_height(canvas_h);

  let ctx: CanvasRenderingContext2d = canvas
    .get_context("2d")
    .expect("invariant: getContext supported")
    .expect("invariant: 2d context exists")
    .dyn_into()
    .expect("invariant: cast to CanvasRenderingContext2d");

  ctx.scale(DPR, DPR).expect("invariant: scale valid");

  for page in 0..layout.total_pages {
    let is_last = page == layout.total_pages - 1;
    draw_page(&ctx, data, items, &layout, page, is_last);
  }
}
