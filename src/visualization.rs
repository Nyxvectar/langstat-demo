use crate::languages::LanguageStats;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn generate_treemap(
    language_stats: &HashMap<String, LanguageStats>,
    output_path: &str,
) -> Result<()> {
    let total_bytes: f64 = language_stats.values().map(|s| s.bytes).sum();

    let mut svg_content =
        String::from("<svg width=\"800\" height=\"600\" xmlns=\"http://www.w3.org/2000/svg\">\n");

    svg_content.push_str(
        "  <text x=\"400\" y=\"30\" text-anchor=\"middle\" \
          font-size=\"24\" font-family=\"sans-serif\">Language Statistics</text>\n",
    );

    if total_bytes == 0.0 {
        svg_content.push_str("</svg>");
        let mut file = File::create(output_path)?;
        file.write_all(svg_content.as_bytes())?;
        return Ok(());
    }

    let colors = [
        "#FF6B6B", "#4ECDC4", "#45B7D1", "#FFA07A", "#98D8C8", "#F7DC6F", "#BB8FCE", "#F8C471",
    ];

    let width = 800.0;
    let height = 550.0;
    let x_offset = 0.0;
    let y_offset = 50.0;

    let rects = calculate_treemap_rects(language_stats, width, height, x_offset, y_offset);

    for (i, (language, stats, x, y, w, h)) in rects.iter().enumerate() {
        if *w > 5.0 && *h > 5.0 {
            let color = colors[i % colors.len()];
            let percentage = stats.bytes / total_bytes * 100.0;

            svg_content.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" \
                  fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>\n",
                x, y, w, h, color
            ));

            if *w > 30.0 && *h > 20.0 {
                let text = format!("{} ({:.1}%)", language, percentage);
                let font_size = if *w > 80.0 && *h > 40.0 { 14 } else { 10 };

                svg_content.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" \
                      font-family=\"sans-serif\" font-size=\"{}\" \
                      fill=\"white\">{}</text>\n",
                    x + w / 2.0,
                    y + h / 2.0 + (font_size as f64) / 3.0,
                    font_size,
                    text
                ));
            }
        }
    }

    svg_content.push_str("</svg>");

    let mut file = File::create(output_path)?;
    file.write_all(svg_content.as_bytes())?;

    Ok(())
}

fn calculate_treemap_rects(
    language_stats: &HashMap<String, LanguageStats>,
    width: f64,
    height: f64,
    x_offset: f64,
    y_offset: f64,
) -> Vec<(String, LanguageStats, f64, f64, f64, f64)> {
    let mut items: Vec<(&String, &LanguageStats)> = language_stats
        .iter()
        .filter(|(_, stats)| stats.bytes > 0.0)
        .collect();

    if items.is_empty() {
        return Vec::new();
    }

    items.sort_by(|a, b| b.1.bytes.partial_cmp(&a.1.bytes).unwrap());

    let total: f64 = items.iter().map(|(_, stats)| stats.bytes).sum();

    let mut rects = Vec::new();
    let mut x = x_offset;
    let mut y = y_offset;
    let mut current_width = width;
    let mut current_height = height;

    for (language, stats) in items {
        let percentage = stats.bytes / total;

        if current_width > current_height {
            let rect_width = current_width * percentage;

            rects.push((
                language.clone(),
                (*stats).clone(),
                x,
                y,
                rect_width,
                current_height,
            ));

            x += rect_width;
            current_width -= rect_width;
        } else {
            let rect_height = current_height * percentage;

            rects.push((
                language.clone(),
                (*stats).clone(),
                x,
                y,
                current_width,
                rect_height,
            ));

            y += rect_height;
            current_height -= rect_height;
        }

        if current_width <= 0.0 || current_height <= 0.0 {
            break;
        }
    }

    rects
}
