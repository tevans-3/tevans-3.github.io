use crate::blog::BlogPost;

pub fn all_posts() -> Vec<BlogPost> {
    vec![sample_post()]
}

fn sample_post() -> BlogPost {
    BlogPost::new(
        "Onionhead: a DIY Onion Router",
        "2026-04-01",
        "diy-onion-router",
    )
    .paragraph("This post describes how I built an onion router to run on my local machine. The system is named \"Onionhead\" in tribute to Todd Rundgren, who wrote a song by that name.")
    .title("Preliminaries")
    .subtitle("Why Care About Onion Routing?")
    .paragraph("Cryptography has historically focused on the protection of transmitted information, paying less attention to the shielding of information metadata. This metadata itself, however, can be a significant security and privacy concern. In countries where censorship is commonplace, simply accessing certain websites may mark you as a target for persecution. \"I have nothing to hide\", you might think, \"Why should I care who knows what websites I visit?\" That attitude, however, is only viable based on your position in a privileged present. Democracies can collapse. What was protected yesterday may be illegal tomorrow. And even in more mundane circumstances, it's often desirable to have control over what you disclose about yourself and how it gets disclosed. An onion router protects information metadata, like what website you visited at a certain time. It prevents passive observers from linking site access requests to the individuals making those requests. That makes it broadly useful, both for things like censorship resistance and also more nefarious things like illegal drug smuggling.")
    .paragraph("The social consequences of the technology are, thankfully, out of scope here; this post strictly focuses on 'how it works'.  To better understand that, we're going to build our own.")
    .colorbox_styled("#f3e5f5", "#7b1fa2", 2, "<strong>Definition 1.1.</strong> An <em>onion router</em> is a system of internetworked proxies which apply a protocol of successive encryption in order to protect information <em>metadata</em>.")
    .subtitle("Tools and Setup")
    .paragraph("Containerlab is a containerized network simulator, built on Docker. We're going to use it to configure our network topology.")
    .colorbox("#e8eaf6", "<strong>Note</strong> You need Docker to run containerlab. If you're reading this post tutorial-style, this post assumes that you already have Docker installed. You can just run this Docker command to install containerlab: \\texttt{docker pull ghcr.io/srl-labs/clab}")
    .title("Configuring a Network with Containerlab")
    .paragraph("A virtual lab is used for running simulations of complex networks on a single machine. Containerlab is a FOSS tool which lets users build their own virtual labs. In containerlab, you build a virtual lab by defining a network topology. A network is the whole set of interconnected components and connections, the complete system. A network topology is an abstraction and specification of a network. It's a set of nodes and a set of edges.")
    .colorbox_styled("#e8f5e9", "#2e7d32", 2, "")
    .latex_block(r"\begin{lstlisting}[style=bash] # topology documentation: http://containerlab.dev/lab-examples/two-srls/
        name: srl02 

        topology: 
            nodes:
                srl1: 
                    kind: nokia_srlinux
                    image: ghcr.io/nokia/nokia_srlinux
                    startup-config: srl1.cfg
                srl2: 
                    kind: nokia_srlinux 
                    image: ghcr.io/nokia/srlinux 
                    startup-config: srl2.cfg 

            links: 
                - endpoints: ['srl1:e1-1', 'srl2:e1-1' 
                \end{lstlisting}\\")
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
