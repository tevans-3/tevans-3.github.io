use crate::project::Project;

pub fn all_projects() -> Vec<Project> {
    vec![
        Project::planned("Bikescape", "Embedded device straps to a bike frame and uses environmental sound classification to real-time generate and BLE-stream visualizations of your bike route with safety-coded heatmaps and other nifty stuff not yet dreamt of in heaven or earth", ""), 
        Project::planned("Bare Metal Game of Life", "Two variants of John Conway's Game of Life, one in RISC-V, one in Rust, handwritten for a 64x32 Waveshare LED matrix with a Raspberry Pi Pico controller.", ""),
        Project::planned("Model Misalignment In Agentic ER Triage Applications", "Possible research project", ""),
        Project::started("Evil Cube", "Adversarial cube solver based on a minimax algorithm. A REINFORCE.js-based RL 'minimizer' agent tries to minimize remaining solve steps while a 'maximizer' fights back; the maximizer predictably scrambles the cube whenever solve-steps fall below a certain threshold. It estimates the number of remaining solve-steps using corner pattern database lookups.", 50.0, "github.com/tevans-3/evil-cube"),
        Project::finished("Synesthesia", "Color-picker sonification maps user-picked hexcodes to 24 tone-equal-temperament chords with in-browser playback", "visualnoise.ca"),
    ]
}
        
/*
        Project::finished("Journal", "The code for this project was entirely AI-generated. A TUI Rust terminal journalling app with analytics, calendar display, Latex exports, and support for local LLM post history analysis. No API calls: spawns a subprocess, calls a user-configured model through ollama, then pipes model output back into the journal TUI without any data ever leaving your machine.")*/
