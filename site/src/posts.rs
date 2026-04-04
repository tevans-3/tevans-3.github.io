use crate::blog::BlogPost;

pub fn all_posts() -> Vec<BlogPost> {
    vec![sample_post()]
}

fn sample_post() -> BlogPost {
    BlogPost::new(
        "On the Topology of Continuous Functions",
        "2026-04-01",
        "topology-continuous-functions",
    )
    .paragraph("We begin with a review of the fundamental structures underlying continuous mappings between topological spaces.")
    .title("Preliminaries")
    .subtitle("Topological Spaces")
    .paragraph("Let \\(X\\) be a topological space equipped with a topology \\(\\tau\\). Recall that \\(\\tau\\) satisfies the usual axioms of closure under arbitrary unions and finite intersections.")
    .colorbox_styled("#f3e5f5", "#7b1fa2", 2, "<strong>Definition 1.1.</strong> A function \\(f: X \\to Y\\) is <em>continuous</em> if for every open set \\(V \\subseteq Y\\), the preimage \\(f^{-1}(V)\\) is open in \\(X\\).")
    .latex_block(r"f^{-1}(V) = \{x \in X \mid f(x) \in V\}")
    .subtitle("Compactness")
    .paragraph("Compactness is among the most powerful properties a topological space can possess.")
    .colorbox("#e8eaf6", "<strong>Theorem 1.2.</strong> A continuous image of a compact space is compact.")
    .title("The Intermediate Value Theorem")
    .paragraph("We now state and prove one of the cornerstones of analysis.")
    .colorbox_styled("#e8f5e9", "#2e7d32", 2, "<strong>Theorem 2.1</strong> (Intermediate Value Theorem). Let \\(f: [a,b] \\to \\mathbb{R}\\) be continuous. If \\(f(a) < d < f(b)\\), then there exists \\(c \\in (a,b)\\) such that \\(f(c) = d\\).")
    .latex_block(r"\forall\, d \in (f(a), f(b)),\; \exists\, c \in (a,b) : f(c) = d")
    .paragraph("<em>Proof.</em> Consider the set \\(S = \\{x \\in [a,b] \\mid f(x) < d\\}\\). Since \\(f(a) < d\\), the set \\(S\\) is nonempty. Let \\(c = \\sup S\\). By continuity of \\(f\\), we conclude \\(f(c) = d\\). \\(\\blacksquare\\)")
    .title("A Geometric Illustration")
    .paragraph("The following diagram illustrates a continuous map between two spaces.")
    .tikz(r"\begin{tikzpicture}
  \draw[thick, purple] (0,0) ellipse (1.5 and 1);
  \draw[thick, purple] (5,0) ellipse (1.5 and 1);
  \draw[->, thick, purple] (1.7,0.3) to[bend left=20] node[above] {$f$} (3.3,0.3);
  \node at (0, -0.1) {$X$};
  \node at (5, -0.1) {$Y$};
  \fill[purple!40] (0.3, 0.2) circle (0.08);
  \fill[purple!40] (4.7, -0.1) circle (0.08);
  \draw[->, thin, purple!60] (0.38, 0.2) to[bend left=10] (4.62, -0.1);
  \node[below, font=\tiny] at (0.3, 0.12) {$x$};
  \node[below, font=\tiny] at (4.7, -0.18) {$f(x)$};
\end{tikzpicture}")
    .link("Further reading on topology", "https://en.wikipedia.org/wiki/Topology")
}
