use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Season::Spring => write!(f, "Spring"),
            Season::Summer => write!(f, "Summer"),
            Season::Fall => write!(f, "Fall"),
            Season::Winter => write!(f, "Winter"),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Course {
    pub name: String,
    pub number: String,
}

#[derive(Clone, Serialize)]
pub struct Semester {
    pub season: Season,
    courses: Vec<Course>,
}

impl Semester {
    pub fn new(season: Season) -> Self {
        Self {
            season,
            courses: Vec::new(),
        }
    }

    pub fn course(mut self, name: &str, number: &str) -> Self {
        assert!(self.courses.len() < 5, "A semester can contain at most 5 courses");
        self.courses.push(Course {
            name: name.to_string(),
            number: number.to_string(),
        });
        self
    }
}

#[derive(Clone, Serialize)]
pub struct Year {
    pub year: u32,
    semesters: Vec<Semester>,
}

impl Year {
    pub fn new(year: u32) -> Self {
        Self {
            year,
            semesters: Vec::new(),
        }
    }

    pub fn semester(mut self, semester: Semester) -> Self {
        assert!(self.semesters.len() < 4, "A year can contain at most 4 semesters");
        self.semesters.push(semester);
        self
    }

    pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str(&format!(
            "<h2 class=\"section-title\">{}</h2>\n",
            self.year
        ));
        for sem in &self.semesters {
            html.push_str(&format!(
                "<h3 class=\"subsection-title\">{}</h3>\n",
                sem.season
            ));
            html.push_str("<ul class=\"course-list\">\n");
            for course in &sem.courses {
                html.push_str(&format!(
                    "  <li><span class=\"course-number\">{}</span> {}</li>\n",
                    escape(&course.number),
                    escape(&course.name)
                ));
            }
            html.push_str("</ul>\n");
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
