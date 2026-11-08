use crate::i18n::I18n;
use crate::models::{ItemType, ServiceItem, ServicePlan};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{Duration, NaiveTime};
use egui::{Color32, ColorImage};
use image::GenericImageView;
use std::path::Path;

pub const LOGO_DARK_BYTES: &[u8] = include_bytes!("../assets/logo_dark.png");
pub const LOGO_LIGHT_BYTES: &[u8] = include_bytes!("../assets/logo_light.png");
pub const APP_ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

/// Load an image file from disk and encode it as a PNG base64 string.
pub fn encode_image_file_to_base64(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    encode_image_bytes_to_base64(&bytes)
}

pub fn encode_image_bytes_to_base64(bytes: &[u8]) -> Option<String> {
    let img = image::load_from_memory(bytes).ok()?;
    // Resize if overly large so database stays fast and compact
    let max_dim = 960;
    let (w, h) = img.dimensions();
    let processed = if w > max_dim || h > max_dim {
        img.resize(max_dim, max_dim, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };
    let mut out = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut out);
    processed.write_to(&mut cursor, image::ImageFormat::Png).ok()?;
    Some(format!("data:image/png;base64,{}", BASE64.encode(&out)))
}

pub fn decode_base64_to_bytes(data: &str) -> Option<Vec<u8>> {
    let raw = if let Some(idx) = data.find("base64,") {
        &data[idx + 7..]
    } else {
        data
    };
    BASE64.decode(raw.trim()).ok()
}

pub fn decode_base64_to_color_image(data: &str) -> Option<ColorImage> {
    let bytes = decode_base64_to_bytes(data)?;
    decode_bytes_to_color_image(&bytes)
}

pub fn decode_bytes_to_color_image(bytes: &[u8]) -> Option<ColorImage> {
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    Some(ColorImage::from_rgba_unmultiplied(size, rgba.as_raw()))
}

/// Extracts an ambient background tint color from a Series artwork image,
/// mirroring `extractMeshGradient` in `colorExtractor.js` by sampling (5,5), (35,5), and (20,35) on a 40x40 canvas.
pub fn extract_ambient_tint(artwork_b64: Option<&str>, is_dark: bool) -> Color32 {
    let default_color = if is_dark {
        Color32::from_rgb(24, 25, 34)
    } else {
        Color32::from_rgb(243, 245, 250)
    };

    let Some(b64) = artwork_b64 else {
        return default_color;
    };
    let Some(bytes) = decode_base64_to_bytes(b64) else {
        return default_color;
    };
    let Ok(img) = image::load_from_memory(&bytes) else {
        return default_color;
    };

    let thumb = img
        .resize_exact(40, 40, image::imageops::FilterType::Triangle)
        .to_rgba8();
    let p1 = thumb.get_pixel(5, 5).0;
    let p2 = thumb.get_pixel(35, 5).0;
    let p3 = thumb.get_pixel(20, 35).0;

    let avg_r = ((p1[0] as u32 * 2 + p2[0] as u32 + p3[0] as u32) / 4) as f32;
    let avg_g = ((p1[1] as u32 * 2 + p2[1] as u32 + p3[1] as u32) / 4) as f32;
    let avg_b = ((p1[2] as u32 * 2 + p2[2] as u32 + p3[2] as u32) / 4) as f32;

    if is_dark {
        let base = 20.0;
        let alpha = 0.18;
        Color32::from_rgb(
            (base * (1.0 - alpha) + avg_r * alpha) as u8,
            (base * (1.0 - alpha) + avg_g * alpha) as u8,
            (base * (1.0 - alpha) + avg_b * alpha) as u8,
        )
    } else {
        let base = 246.0;
        let alpha = 0.09;
        Color32::from_rgb(
            (base * (1.0 - alpha) + avg_r * alpha) as u8,
            (base * (1.0 - alpha) + avg_g * alpha) as u8,
            (base * (1.0 - alpha) + avg_b * alpha) as u8,
        )
    }
}

/// Generates a monochrome 1-ink print-ready HTML Run Sheet matching the original CSS print layout
/// and opens it in the user's system browser with automatic print dialog trigger.
pub fn generate_printable_html(
    service: &ServicePlan,
    items: &[ServiceItem],
    i18n: &I18n,
) -> String {
    let logo_b64 = format!("data:image/png;base64,{}", BASE64.encode(LOGO_DARK_BYTES));

    let start_time =
        NaiveTime::parse_from_str(&service.time, "%H:%M").unwrap_or_else(|_| {
            NaiveTime::from_hms_opt(10, 0, 0).unwrap()
        });
    let total_secs: u32 = items.iter().map(|i| i.duration).sum();
    let end_time = start_time + Duration::seconds(total_secs as i64);

    let total_hours = total_secs / 3600;
    let total_mins = (total_secs % 3600) / 60;
    let total_formatted = if total_hours > 0 {
        format!("{}h {}m", total_hours, total_mins)
    } else {
        format!("{} min", total_mins)
    };

    let mut details_parts = Vec::new();
    if !service.series.is_empty() {
        details_parts.push(format!("{}: {}", i18n.t("seriesPrefix"), html_escape(&service.series)));
    }
    if !service.week_number.is_empty() {
        details_parts.push(html_escape(&service.week_number));
    }
    if !service.sermon_title.is_empty() {
        details_parts.push(format!("\"{}\"", html_escape(&service.sermon_title)));
    }
    let sermon_details = details_parts.join(" • ");

    let item_col_header = match i18n.lang {
        crate::models::Language::Es => "ELEMENTO",
        crate::models::Language::En => "ITEM",
    };

    let mut rows_html = String::new();
    for item in items {
        let key_badge = if item.item_type == ItemType::Song && !item.key.is_empty() {
            format!(
                r#"<span class="setlist-key-badge">{}: {}</span>"#,
                html_escape(i18n.t("key")),
                html_escape(&item.key)
            )
        } else {
            String::new()
        };

        rows_html.push_str(&format!(
            r#"<div class="setlist-row">
                <div class="setlist-row-main">
                    <div class="setlist-duration-col">{duration}</div>
                    <div class="setlist-type-icon">{icon}</div>
                    <div class="setlist-title-col">
                        <div>
                            <span>{title}</span>
                            {key_badge}
                        </div>
                    </div>
                    <div class="setlist-notes-col">{notes}</div>
                    <div class="setlist-speaker-col">{vocalist}</div>
                </div>
            </div>"#,
            duration = item.format_duration(),
            icon = item.item_type.icon_symbol(),
            title = html_escape(&item.title),
            key_badge = key_badge,
            notes = if item.notes.is_empty() {
                "—".to_string()
            } else {
                html_escape(&item.notes)
            },
            vocalist = if item.lead_vocalist.is_empty() {
                "—".to_string()
            } else {
                html_escape(&item.lead_vocalist)
            }
        ));
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}">
<head>
<meta charset="UTF-8">
<title>{title} - {date}</title>
<style>
  @page {{
    margin: 1cm 0.8cm;
    size: letter portrait;
  }}
  * {{
    box-sizing: border-box;
    -webkit-print-color-adjust: exact !important;
    print-color-adjust: exact !important;
  }}
  body {{
    background: #ffffff;
    color: #000000;
    font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Arial, sans-serif;
    margin: 0;
    padding: 20px;
  }}
  .setlist-sheet {{
    border: 1.5px solid #000000;
    width: 100%;
    max-width: 960px;
    margin: 0 auto;
    background: #ffffff;
  }}
  .setlist-banner {{
    display: grid;
    grid-template-columns: 1fr auto;
    align-items: center;
    justify-content: space-between;
    border-bottom: 2px solid #000000;
    padding: 16px 20px;
    gap: 20px;
  }}
  .setlist-title {{
    font-size: 20pt;
    font-weight: 800;
    line-height: 1.15;
    margin: 0;
    letter-spacing: -0.02em;
  }}
  .setlist-details {{
    font-size: 11pt;
    font-weight: 600;
    margin-top: 4px;
  }}
  .setlist-meta {{
    font-size: 10pt;
    font-weight: 600;
    margin-top: 8px;
    display: flex;
    gap: 24px;
  }}
  .print-only-logo {{
    height: 70px;
    max-width: 220px;
    object-fit: contain;
  }}
  .setlist-table-header {{
    display: grid;
    grid-template-columns: 80px 32px 2fr 1.4fr 1.2fr;
    background: #f0f0f0;
    border-top: 1px solid #000000;
    border-bottom: 2px solid #000000;
    font-size: 9.5pt;
    font-weight: 800;
    padding: 10px 14px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    gap: 8px;
  }}
  .setlist-row {{
    border-bottom: 1px solid #000000;
    break-inside: avoid;
    page-break-inside: avoid;
  }}
  .setlist-row-main {{
    display: grid;
    grid-template-columns: 80px 32px 2fr 1.4fr 1.2fr;
    padding: 11px 14px;
    align-items: center;
    gap: 8px;
  }}
  .setlist-duration-col {{
    font-family: monospace;
    font-weight: 800;
    font-size: 11.5pt;
  }}
  .setlist-type-icon {{
    text-align: center;
    font-size: 13pt;
    font-weight: 800;
  }}
  .setlist-title-col > div {{
    display: flex;
    flex-direction: column;
    gap: 3px;
    align-items: flex-start;
  }}
  .setlist-title-col > div > span:first-child {{
    font-size: 13.5pt;
    font-weight: 800;
  }}
  .setlist-key-badge {{
    border: 1.5px solid #000000;
    font-weight: 800;
    font-size: 9.5pt;
    padding: 1px 6px;
    border-radius: 3px;
    display: inline-block;
  }}
  .setlist-notes-col {{
    font-size: 11pt;
    font-style: italic;
    font-weight: 600;
  }}
  .setlist-speaker-col {{
    font-size: 12pt;
    font-weight: 800;
  }}
  @media print {{
    body {{ padding: 0; }}
    .no-print-bar {{ display: none !important; }}
  }}
</style>
</head>
<body onload="setTimeout(() => window.print(), 250)">
  <div class="no-print-bar" style="max-width:960px;margin:0 auto 12px;display:flex;justify-content:flex-end;">
    <button onclick="window.print()" style="padding:8px 16px;font-weight:700;cursor:pointer;">🖨 {print_label}</button>
  </div>
  <div class="setlist-sheet">
    <div class="setlist-banner">
      <div>
        <h1 class="setlist-title">{title}</h1>
        {details_block}
        <div class="setlist-meta">
          <span>📅 {long_date}</span>
          <span>🕒 {start_label}: <strong>{start_time}</strong></span>
          <span>⏱ {total_label}: <strong>{total_formatted}</strong> ({est_end_label}: {end_time})</span>
        </div>
      </div>
      <div>
        <img src="{logo_b64}" class="print-only-logo" alt="Logo" />
      </div>
    </div>
    <div class="setlist-table-header">
      <div>{length_label}</div>
      <div></div>
      <div>{item_col_header}</div>
      <div>{notes_label}</div>
      <div>{vocalist_label}</div>
    </div>
    <div class="setlist-table-body">
      {rows_html}
    </div>
  </div>
</body>
</html>"#,
        lang = i18n.lang.as_code(),
        title = html_escape(&service.title),
        date = html_escape(&service.date),
        print_label = html_escape(i18n.t("printRunSheet")),
        details_block = if sermon_details.is_empty() {
            String::new()
        } else {
            format!(r#"<div class="setlist-details">{}</div>"#, sermon_details)
        },
        long_date = html_escape(&i18n.format_long_date(&service.date)),
        start_label = html_escape(i18n.t("start")),
        start_time = I18n::format_naive_time_12h(start_time),
        total_label = html_escape(i18n.t("total")),
        total_formatted = total_formatted,
        est_end_label = html_escape(i18n.t("estEnd")),
        end_time = I18n::format_naive_time_12h(end_time),
        logo_b64 = logo_b64,
        length_label = html_escape(i18n.t("length")),
        item_col_header = item_col_header,
        notes_label = html_escape(i18n.t("notesCues")),
        vocalist_label = html_escape(i18n.t("leadVocalist")),
        rows_html = rows_html,
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Language, ServiceItem, ServicePlan};

    #[test]
    fn test_export_and_generate_ico() {
        let img = image::load_from_memory(APP_ICON_BYTES).expect("valid icon.png");
        let ico_img = img.resize_exact(256, 256, image::imageops::FilterType::Lanczos3);
        ico_img
            .save_with_format("assets/icon.ico", image::ImageFormat::Ico)
            .expect("save assets/icon.ico");
        assert!(std::path::Path::new("assets/icon.ico").exists());

        let tint_dark = extract_ambient_tint(None, true);
        let tint_light = extract_ambient_tint(None, false);
        assert_ne!(tint_dark, tint_light);

        let i18n = I18n::new(Language::Es);
        let svc = ServicePlan::default();
        let items = vec![ServiceItem {
            id: "1".into(),
            service_id: svc.id.clone(),
            item_type: ItemType::Song,
            title: "Way Maker".into(),
            subtitle: "Sinach".into(),
            arrangement_name: "Standard".into(),
            key: "G".into(),
            lead_vocalist: "Ana".into(),
            duration: 300,
            order: 0,
            notes: "Intro suave".into(),
        }];
        let html = generate_printable_html(&svc, &items, &i18n);
        assert!(html.contains("Way Maker"));
        assert!(html.contains("Tono: G"));
    }
}

