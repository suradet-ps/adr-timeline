//! Date utilities for Thai Buddhist calendar formatting.

/// Formats a date string (YYYY-MM-DD) as Thai Buddhist Era short date (D/M/YY).
pub(crate) fn format_short_thai(date_str: &str) -> String {
  let parts: Vec<&str> = date_str.split('-').collect();
  if parts.len() != 3 {
    return "-".to_string();
  }
  let year: i32 = parts[0].parse().unwrap_or(0);
  let month: i32 = parts[1].parse().unwrap_or(0);
  let day: i32 = parts[2].parse().unwrap_or(0);
  let be_year = year + 543;
  format!("{}/{}/{}", day, month, be_year % 100)
}

/// Formats a date string (YYYY-MM-DD) as Thai Buddhist Era long date.
#[allow(dead_code)]
pub(crate) fn format_long_thai(date_str: &str) -> String {
  let months = [
    "มกราคม",
    "กุมภาพันธ์",
    "มีนาคม",
    "เมษายน",
    "พฤษภาคม",
    "มิถุนายน",
    "กรกฎาคม",
    "สิงหาคม",
    "กันยายน",
    "ตุลาคม",
    "พฤศจิกายน",
    "ธันวาคม",
  ];
  let parts: Vec<&str> = date_str.split('-').collect();
  if parts.len() != 3 {
    return "วันที่ ......... เดือน ............................ ปี ...............".to_string();
  }
  let year: i32 = parts[0].parse().unwrap_or(0);
  let month: usize = parts[1].parse().unwrap_or(0);
  let day: i32 = parts[2].parse().unwrap_or(0);
  let be_year = year + 543;
  if month == 0 || month > 12 {
    return "-".to_string();
  }
  format!("วันที่ {} เดือน {} ปี {}", day, months[month - 1], be_year)
}

/// Returns today's date as "YYYY-MM-DD".
pub(crate) fn today_iso() -> String {
  let date = js_sys::Date::new_0();
  let y = date.get_full_year();
  let m = date.get_month() + 1;
  let d = date.get_date();
  format!("{}-{:02}-{:02}", y, m, d)
}
