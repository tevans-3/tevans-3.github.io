use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub progress: f64,
    pub link: String,
}

impl Project {
    pub fn planned(title: &str, description: &str, link: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            progress: 0.0,
            link: link.to_string(),
        }
    }

    pub fn started(title: &str, description: &str, progress: f64, link: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            progress,
            link: link.to_string(),
        }
    }

    pub fn finished(title: &str, description: &str, link: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            progress: 100.0,
            link: link.to_string(),
        }
    }
}
