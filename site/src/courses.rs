use crate::course::{Season, Semester, Year};

pub fn all_years() -> Vec<Year> {
    vec![
        Year::new(2027)
            .semester(
                Semester::new(Season::Winter)
                    .course("TODO- Theory of Probability", "MATH 467")
                    .course("TODO- Group Theory in Physics", "MAPH 364")
            ),
        Year::new(2026)
            .semester(
                Semester::new(Season::Fall)
                    .course("TODO- Compiler Design", "CMPUT 415")
                    .course("TODO- Partial Differential Equations", "MATH 337")
            )
            .semester(
                Semester::new(Season::Summer)
                    .course("TODO- Calculus IV", "MATH 315")
            )
            .semester(
                Semester::new(Season::Spring)
                    .course("TODO- Ordinary Differential Equations", "MATH 334")
            )
            .semester(
                Semester::new(Season::Winter)
                    .course("Parallel and Distributed Programming", "CMPUT 481")
                    .course("Distributed Systems Architecture", "CMPUT 398")
                    .course("Exploring Software Development Domains", "ECE 421")
            ),
        Year::new(2025)
            .semester(
                Semester::new(Season::Fall)
                    .course("Cryptography and Digital Privacy", "CMPUT 496")
                    .course("Operating Systems", "CMPUT 379")
                    .course("Machine Learning I", "CMPUT 267")
            )
            .semester(
                Semester::new(Season::Spring)
                    .course("Probability and Statistics I", "STAT 266")
                    .course("Graph Theory", "MATH 322")
            )
            .semester(
                Semester::new(Season::Winter)
                    .course("Introduction to Database and File Management", "CMPUT 291")
                    .course("Computer Organization and Architecture", "CMPUT 229")
                    .course("Algorithms I", "CMPUT 204")
            ),
        Year::new(2024)
            .semester(
                Semester::new(Season::Fall)
                    .course("Honors Calculus III", "MATH 217")
                    .course("Group Theory", "MATH 328")
                    .course("Formal Systems and Logic in Computing Science", "CMPUT 272")
            ).semester(
                Semester::new(Season::Winter)
                    .course("Practical Programming Methodology", "CMPUT 201")
                    .course("Honors Calculus II", "MATH 118")
                    .course("Honors Linear Algebra II", "MATH 227")
            ), 
        Year::new(2023)
            .semester(
                Semester::new(Season::Fall)
                    .course("Honors Calculus I", "MATH 117")
                    .course("Honors Linear Algebra I", "MATH 127")
                    .course("Introduction to Foundations of Computation II", "CMPUT 175")
            ), 
    ]
}
