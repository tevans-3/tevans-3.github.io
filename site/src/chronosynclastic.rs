pub fn reading_content() -> String {
    let mut html = String::new();
    html.push_str("<ul class=\"reading-list\">\n");
    html.push_str("</ul>\n");
    html
}

pub fn quotes_content() -> String {
    let mut html = String::new();
    html.push_str("<div class=\"quotes\">\n");
    html.push_str("</div>\n");
    html
}
