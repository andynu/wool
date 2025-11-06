use std::process::Command;
use std::fs;
use std::io::Write;
use regex::Regex;

/// Check if D2 is available on the system
pub fn check_d2_available() -> bool {
    Command::new("d2")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Render D2 code to SVG using the d2 command
pub fn render_d2_to_svg(d2_code: &str) -> Result<String, String> {
    // Create temporary files
    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join(format!("d2_input_{}.d2", std::process::id()));
    let output_path = temp_dir.join(format!("d2_output_{}.svg", std::process::id()));

    // Write D2 code to temp file
    let mut input_file = fs::File::create(&input_path)
        .map_err(|e| format!("Failed to create temp input file: {}", e))?;
    input_file.write_all(d2_code.as_bytes())
        .map_err(|e| format!("Failed to write D2 code: {}", e))?;

    // Execute d2 command
    let output = Command::new("d2")
        .arg(&input_path)
        .arg(&output_path)
        .output()
        .map_err(|e| format!("Failed to execute d2 command: {}", e))?;

    // Clean up input file
    let _ = fs::remove_file(&input_path);

    // Check if command succeeded
    if !output.status.success() {
        let _ = fs::remove_file(&output_path);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("D2 rendering failed: {}", stderr));
    }

    // Read SVG output
    let svg_content = fs::read_to_string(&output_path)
        .map_err(|e| format!("Failed to read SVG output: {}", e))?;

    // Clean up output file
    let _ = fs::remove_file(&output_path);

    Ok(svg_content)
}

/// Process HTML to replace D2 code blocks with rendered SVG
pub fn process_d2_blocks(html: &str) -> String {
    // Regex to match github_pre_lang format: <pre lang="d2"><code>...</code></pre>
    let re = Regex::new(r#"(?s)<pre lang="d2"><code>(.*?)</code></pre>"#)
        .expect("Invalid regex");

    let mut result = html.to_string();
    let mut offset: isize = 0;

    for cap in re.captures_iter(html) {
        let full_match = cap.get(0).unwrap();
        let d2_code_html = cap.get(1).unwrap().as_str();

        // Decode HTML entities in the code block
        let d2_code = decode_html_entities(d2_code_html);

        // Try to render the D2 code
        let replacement = match render_d2_to_svg(&d2_code) {
            Ok(svg) => {
                // Wrap SVG in a div with class for styling
                format!(r#"<div class="d2-diagram">{}</div>"#, svg)
            }
            Err(err) => {
                // On error, keep the code block but add error message
                format!(
                    r#"<pre lang="d2"><code>{}</code></pre><div class="d2-error" style="color: red; border: 1px solid red; padding: 10px; margin: 10px 0;">D2 Rendering Error: {}</div>"#,
                    d2_code_html, err
                )
            }
        };

        // Calculate positions with offset
        let start = (full_match.start() as isize + offset) as usize;
        let end = (full_match.end() as isize + offset) as usize;

        // Replace in result string
        result.replace_range(start..end, &replacement);

        // Update offset for next iteration
        offset += replacement.len() as isize - full_match.len() as isize;
    }

    result
}

/// Decode common HTML entities in code blocks
fn decode_html_entities(html: &str) -> String {
    html.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_html_entities() {
        let input = "x &lt; y &amp;&amp; z &gt; 0";
        let expected = "x < y && z > 0";
        assert_eq!(decode_html_entities(input), expected);
    }

    #[test]
    fn test_check_d2_available() {
        // This test will pass or fail based on whether d2 is installed
        let available = check_d2_available();
        println!("D2 available: {}", available);
    }
}
