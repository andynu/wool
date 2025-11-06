pub fn format_boilerplate(filename: &str) -> String {
    let template = include_str!("../templates/boilerplate.html");
    template.replace("{filename}", filename)
}

#[allow(dead_code)]
pub fn format_boilerplate_no_preview(_filename: &str) -> String {
    format!(
"
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\">
		<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, minimal-ui\">
		<title>GitHub Markdown CSS demo</title>
  <style>
    .preview-page {{
      margin-top: 64px;
    }}
    /* User-content tweaks */
    .timeline-comment-wrapper > .timeline-comment:after,
    .timeline-comment-wrapper > .timeline-comment:before {{
      content: none;
    }}
    /* User-content overrides */
    .discussion-timeline.wide {{
      width: 920px;
    }}
  </style>
</head>
  <style>

 .top-bar {{
  box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  background-color: #f6f8fa;
  border:1px solid rgba(27,31,35,0.15);
  border-radius:3px 3px 0 0;
  border-style: solid solid none;
  margin: 64px auto 0;
  width: 100%;
  padding: 9px;
  font-family: -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\";
  font-weight: bold;
  font-size: 85%;
}}

.file-name {{
	margin-left: 22px;
}}

.markdown-body {{
  box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  margin: 0 auto 51px auto;
  padding: 45px;
  border:1px solid rgba(27,31,35,0.15);
  border-radius:0 0 3px 3px;
}}

@media (max-width: 767px) {{
  .markdown-body {{
    padding: 15px;
  }}
}}



    /*
     #readme {{
                box-sizing: border-box;
                min-width: 200px;
                margin: 0 auto;
                max-width: 980px;
      }}
      */
    </style>
  <body>
    <article class=\"markdown-body\">
 ")
}

pub static CSS: &str = include_str!("../templates/github-markdown.css");

pub static FOOTER: &str = include_str!("../templates/footer.html");

#[allow(dead_code)]
pub static DARK_CSS: &str = include_str!("../templates/github-markdown-dark.css");
