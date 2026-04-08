use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct BlogPostMeta {
    pub title: String,
    pub date: String,
    pub slug: String,
}

pub enum Element {
    Title(String),
    Subtitle(String),
    Paragraph(String),
    Link { text: String, url: String },
    InlineLatex(String),
    BlockLatex(String),
    CodeBlock { code: String, lang: String },
    Image { path: String, alt: Option<String> },
    Tikz(String),
    Colorbox {
        bg_color: String,
        border_color: Option<String>,
        border_width: Option<u32>,
        content: String,
    },
}

pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub slug: String,
    elements: Vec<Element>,
}

impl BlogPost {
    pub fn new(title: &str, date: &str, slug: &str) -> Self {
        Self {
            title: title.to_string(),
            date: date.to_string(),
            slug: slug.to_string(),
            elements: Vec::new(),
        }
    }

    pub fn title(mut self, t: &str) -> Self {
        self.elements.push(Element::Title(t.to_string()));
        self
    }

    pub fn subtitle(mut self, t: &str) -> Self {
        self.elements.push(Element::Subtitle(t.to_string()));
        self
    }

    pub fn paragraph(mut self, t: &str) -> Self {
        self.elements.push(Element::Paragraph(t.to_string()));
        self
    }

    pub fn link(mut self, text: &str, url: &str) -> Self {
        self.elements.push(Element::Link {
            text: text.to_string(),
            url: url.to_string(),
        });
        self
    }

    pub fn latex(mut self, expr: &str) -> Self {
        self.elements.push(Element::InlineLatex(expr.to_string()));
        self
    }

    pub fn latex_block(mut self, expr: &str) -> Self {
        self.elements.push(Element::BlockLatex(expr.to_string()));
        self
    }

    pub fn code_block(mut self, code: &str, lang: &str) -> Self {
        self.elements.push(Element::CodeBlock {
            code: code.to_string(),
            lang: lang.to_string(),
        });
        self
    }

    pub fn image(mut self, path: &str) -> Self {
        self.elements.push(Element::Image {
            path: format!("/static/images/{}", path),
            alt: None,
        });
        self
    }

    pub fn image_with_alt(mut self, path: &str, alt: &str) -> Self {
        self.elements.push(Element::Image {
            path: format!("/static/images/{}", path),
            alt: Some(alt.to_string()),
        });
        self
    }

    pub fn tikz(mut self, code: &str) -> Self {
        self.elements.push(Element::Tikz(code.to_string()));
        self
    }

    pub fn colorbox(mut self, bg_color: &str, content: &str) -> Self {
        self.elements.push(Element::Colorbox {
            bg_color: bg_color.to_string(),
            border_color: None,
            border_width: None,
            content: content.to_string(),
        });
        self
    }

    pub fn colorbox_styled(
        mut self,
        bg_color: &str,
        border_color: &str,
        border_width: u32,
        content: &str,
    ) -> Self {
        self.elements.push(Element::Colorbox {
            bg_color: bg_color.to_string(),
            border_color: Some(border_color.to_string()),
            border_width: Some(border_width),
            content: content.to_string(),
        });
        self
    }

    pub fn meta(&self) -> BlogPostMeta {
        BlogPostMeta {
            title: self.title.clone(),
            date: self.date.clone(),
            slug: self.slug.clone(),
        }
    }

    pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str(&format!(
            "<header class=\"post-header\"><h1 class=\"post-title\">{}</h1>\
             <time class=\"post-date\">{}</time></header>\n",
            escape(&self.title),
            escape(&self.date)
        ));

        for element in &self.elements {
            match element {
                Element::Title(t) => {
                    html.push_str(&format!("<h2 class=\"section-title\">{}</h2>\n", escape(t)));
                }
                Element::Subtitle(t) => {
                    html.push_str(&format!(
                        "<h3 class=\"subsection-title\">{}</h3>\n",
                        escape(t)
                    ));
                }
                Element::Paragraph(t) => {
                    html.push_str(&format!("<p>{}</p>\n", t));
                }
                Element::Link { text, url } => {
                    html.push_str(&format!(
                        "<p><a href=\"{}\">{}</a></p>\n",
                        escape(url),
                        escape(text)
                    ));
                }
                Element::InlineLatex(expr) => {
                    html.push_str(&format!("<p>\\({}\\)</p>\n", expr));
                }
                Element::BlockLatex(expr) => {
                    html.push_str(&format!(
                        "<div class=\"latex-block\">\\[{}\\]</div>\n",
                        expr
                    ));
                }
                Element::CodeBlock { code, lang } => {
                    html.push_str(&format!(
                        "<pre><code class=\"language-{}\">{}</code></pre>\n",
                        escape(lang),
                        escape(code)
                    ));
                }
                Element::Image { path, alt } => {
                    let alt_text = alt.as_deref().unwrap_or("");
                    html.push_str(&format!(
                        "<figure class=\"post-figure\">\
                         <img src=\"{}\" alt=\"{}\" />\
                         </figure>\n",
                        escape(path),
                        escape(alt_text)
                    ));
                }
                Element::Tikz(code) => {
                    html.push_str(&format!(
                        "<div class=\"tikz-container\">\
                         <script type=\"text/tikz\">{}</script></div>\n",
                        code
                    ));
                }
                Element::Colorbox {
                    bg_color,
                    border_color,
                    border_width,
                    content,
                } => {
                    let border_style = match (border_color, border_width) {
                        (Some(c), Some(w)) => format!("border: {}px solid {};", w, c),
                        (Some(c), None) => format!("border: 2px solid {};", c),
                        _ => String::new(),
                    };
                    html.push_str(&format!(
                        "<div class=\"colorbox\" style=\"background-color: {};{}\">\
                         <p>{}</p></div>\n",
                        bg_color, border_style, content
                    ));
                }
            }
        }
        html
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
