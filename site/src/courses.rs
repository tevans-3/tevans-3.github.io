use crate::course::{Season, Semester, Year};

pub fn all_years() -> Vec<Year> {
    vec![
        Year::new(2026)
            .semester(
                Semester::new(Season::Spring)
                    .course("Abstract Algebra", "MATH 451")
                    .course("Real Analysis", "MATH 447"),
            ),
    ]
}
