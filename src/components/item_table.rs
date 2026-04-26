//! Items table component — list all timeline entries with edit/delete.

use crate::app::AppState;
use crate::models::ItemType;
use leptos::prelude::*;

/// Table listing all added items with delete controls.
#[component]
pub(crate) fn ItemTable() -> impl IntoView {
  let state = use_context::<AppState>().expect("invariant: AppState in context");

  view! {
      <div class="item-table-wrap">
          <Show
              when=move || !state.items.get().is_empty()
              fallback=|| view! {
                  <p class="table-empty">"ยังไม่มีรายการ — เพิ่มยา/อาการ/แล็บจากแถบด้านซ้าย"</p>
              }
          >
              <table class="item-table">
                  <thead>
                      <tr>
                          <th>"ประเภท"</th>
                          <th>"ชื่อ"</th>
                          <th>"เริ่ม"</th>
                          <th>"สิ้นสุด"</th>
                          <th></th>
                      </tr>
                  </thead>
                  <tbody>
                      <For
                          each=move || state.items.get()
                          key=|item| item.id
                          children=move |item| {
                              let item_id = item.id;
                              let type_label = match item.item_type {
                                  ItemType::Drug => "ยา",
                                  ItemType::Reaction => "ADR",
                                  ItemType::Lab => "Lab",
                              };
                              let type_class = match item.item_type {
                                  ItemType::Drug => "badge badge-drug",
                                  ItemType::Reaction => "badge badge-reaction",
                                  ItemType::Lab => "badge badge-lab",
                              };
                              let start_str = item.start.clone().unwrap_or_else(|| "ไม่ระบุ".to_string());
                              let end_str = if item.ongoing {
                                  "ยังใช้อยู่".to_string()
                              } else {
                                  item.end.clone().unwrap_or_else(|| "-".to_string())
                              };
                              let name = item.name.clone();
                              view! {
                                  <tr>
                                      <td><span class=type_class>{type_label}</span></td>
                                      <td>{name}</td>
                                      <td>{start_str}</td>
                                      <td>{end_str}</td>
                                      <td>
                                          <button
                                              class="btn btn-icon btn-danger"
                                              title="ลบรายการ"
                                              on:click=move |_| {
                                                  state.items.update(|v| v.retain(|i| i.id != item_id));
                                              }
                                          >
                                              "✕"
                                          </button>
                                      </td>
                                  </tr>
                              }
                          }
                      />
                  </tbody>
              </table>
          </Show>
      </div>
  }
}
