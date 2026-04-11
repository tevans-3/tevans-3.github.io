use crate::course::{Season, Semester, Year};

pub fn all_years() -> Vec<Year> {
    vec![
        Year::new(2027)
            .semester(
                Semester::new(Season::Winter)
                    .course("TODO- Theory of Probability", "MATH 467",
                        "Probability measures, random variables, distributions, Lebesgue-Stieltjes integration, independence, convergence, characteristic functions, the Law of Large Numbers, and the Central Limit Theorem.")
                    .course("TODO- Group Theory in Physics", "MAPH 364",
                        "Symmetries in physics; basic concepts of group theory and representation theory; finite groups; continuous groups; orthogonal and unitary groups; Lie groups; spinor representations; Lorentz and Poincare groups.")
            ),
        Year::new(2026)
            .semester(
                Semester::new(Season::Fall)
                    .course("TODO- Compiler Design", "CMPUT 415",
                        "Compilers, interpreters, lexical analysis, syntax analysis, syntax-directed translation, symbol tables, type checking, flow analysis, code generation, and code optimization.")
                    .course("TODO- Partial Differential Equations", "MATH 337",
                        "Boundary value problems of classical mathematical physics, orthogonal expansions, classical special functions, and advanced transform techniques.")
            )
            .semester(
                Semester::new(Season::Summer)
                    .course("TODO- Calculus IV", "MATH 315",
                        "Vector calculus, line and surface integrals, the divergence, Green's, and Stokes' theorems, and differential forms.")
            )
            .semester(
                Semester::new(Season::Spring)
                    .course("TODO- Ordinary Differential Equations", "MATH 334",
                        "First order equations, linear equations of higher order, power series solutions, Laplace transform methods, introduction to special functions, and introduction to linear systems.")
            )
            .semester(
                Semester::new(Season::Winter)
                    .course("Parallel and Distributed Programming", "CMPUT 481",
                        "Introduction to parallel programming, parallel and distributed systems, and high-performance computing, covering both shared-memory parallel computers and distributed-memory multicomputers.")
                    .course("Distributed Systems Architecture", "CMPUT 398",
                        "Topics course covering distributed systems architecture, focusing on client-server computing.")
                    .course("Exploring Software Development Domains", "ECE 421",
                        "Advanced programming concepts including productivity, components and re-use, object-oriented construction, systems programming, concurrent programming, distributed programming, and GUI programming.")
            ),
        Year::new(2025)
            .semester(
                Semester::new(Season::Fall)
                    .course("Cryptography and Digital Privacy", "CMPUT 496",
                        "Topics course covering cryptographic primitives, public-key cryptography, digital signatures, and privacy-preserving protocols.")
                    .course("Operating Systems", "CMPUT 379",
                        "Process management, interrupt processing, resource allocation, semaphores, deadlock, memory management, virtual memory, paging, scheduling, file systems, and OS security.")
                    .course("Machine Learning I", "CMPUT 267",
                        "Introduction to the fundamental statistical, mathematical, and computational concepts in analyzing data, providing a solid foundation in the mathematics of machine learning.")
            )
            .semester(
                Semester::new(Season::Spring)
                    .course("Probability and Statistics I", "STAT 266",
                        "Combinatorial probability, conditional probability, Bayes' Theorem, random variables, discrete and continuous distributions, expected values, moment generating functions, and multivariate distributions.")
                    .course("Graph Theory", "MATH 322",
                        "Graphs, paths and cycles, trees, planarity and duality, coloring problems, digraphs, matching problems, and matroid theory.")
            )
            .semester(
                Semester::new(Season::Winter)
                    .course("Introduction to Database and File Management", "CMPUT 291",
                        "Entity-relationship model, relational model, SQL and relational query languages, storage architecture, physical organization of data, and access methods for relational data.")
                    .course("Computer Organization and Architecture", "CMPUT 229",
                        "Number representation, instruction-set architecture, assembly-level programming, procedures, stack frames, memory access, exception handling, computer arithmetic, datapath, control logic, pipelining, and memory hierarchy.")
                    .course("Algorithms I", "CMPUT 204",
                        "Fundamentals of searching, sorting, and graph algorithms, covering divide and conquer, dynamic programming, greedy methods, backtracking, and local search, with analysis techniques to estimate program efficiency.")
            ),
        Year::new(2024)
            .semester(
                Semester::new(Season::Fall)
                    .course("Honors Calculus III", "MATH 217",
                        "Axiomatic development of the real number system, topology of Rn, sequences, limits and continuity, multi-variable calculus including differentiation and integration, the chain rule, Taylor's formula, and vector field theory.")
                    .course("Group Theory", "MATH 328",
                        "Groups, subgroups, homomorphisms, symmetry groups, permutations, Cayley's Theorem, group actions, cosets and Lagrange's Theorem, normal subgroups, quotient groups, isomorphism theorems, and finite Abelian groups.")
                    .course("Formal Systems and Logic in Computing Science", "CMPUT 272",
                        "Introduction to set theory, logic, and induction, and their use in reasoning about algorithms and programs; propositional and predicate logic, proof systems, inductive definitions, and program specification and correctness.")
            ).semester(
                Semester::new(Season::Winter)
                    .course("Practical Programming Methodology", "CMPUT 201",
                        "Principles, methods, tools, and practices of the professional programmer, focusing on abstract data types and their implementations, with an intensive apprenticeship using C and Unix tools.")
                    .course("Honors Calculus II", "MATH 118",
                        "Integration and the Fundamental Theorem, techniques and applications of integration, derivatives and integrals of exponential and trigonometric functions, infinite series, and introduction to partial derivatives.")
                    .course("Honors Linear Algebra II", "MATH 227",
                        "Quotients and direct sums, Cayley-Hamilton, canonical forms, real and complex inner product spaces, orthogonality, singular value decomposition, and introduction to abstract algebra including groups, rings, and modules.")
            ),
        Year::new(2023)
            .semester(
                Semester::new(Season::Fall)
                    .course("Honors Calculus I", "MATH 117",
                        "Functions, continuity, and the derivative; applications of the derivative; extended limits and L'Hospital's rule.")
                    .course("Honors Linear Algebra I", "MATH 127",
                        "Linear equations, Euclidean spaces and matrices, complex numbers and fields, vector spaces, introductions to groups and rings, permutation groups, determinants, and eigenvalues and eigenvectors.")
                    .course("Introduction to Foundations of Computation II", "CMPUT 175",
                        "Continuation of CMPUT 174 with greater depth and complexity, exploring objects, functional programming, abstract data types, and searching and sorting algorithms studied in terms of time and space efficiency.")
            ),
    ]
}
