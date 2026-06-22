use crate::project::Project;

pub fn all_projects() -> Vec<Project> {
    vec![
        //Project::planned("Bikescape", "Embedded device straps to a bike frame and uses environmental image classification to generate real-time visualizations of your bike route with safety-coded heatmaps and other nifty stuff not yet dreamt of in heaven or earth", ""):qa,
        //Project::planned("This Little Light", "A Signal server running in secret on a distributed cluster of
        //Wi-Fi lightbulbs.") 
        //Project::planned("50 Years", "Embedded device displays the text of Ulysses at the rate of 1 sentence or 10 words per day. The device's display is livestreamed to the browser.", ""),
        //Project::planned("Spacetime", "Browser visualization of the evolution of astronomical
        //models of the solar system, beginning with the Ancient Greeks in 350 BCE up to general relativity in 1920.
        Project::started("Bare Metal Game of Life", "Two variants of John Conway's Game of Life, one in RISC-V, one in Rust, handwritten for a 64x32 Waveshare LED matrix with a Raspberry Pi Pico controller.", 10.0, ""),
        Project::started("Evil Cube", "An adversarial Rubik's cube: a Rubik's cube that fights back. The cube scrambles itself whenever a 'distance-to-solved' metric falls below a certain threshold. It estimates the number of remaining solve-steps using corner pattern database lookups.", 75.0, "github.com/tevans-3/evil-cube"),
        Project::finished("Synesthesia", "Color-picker sonification maps user-picked hexcodes to 24 tone-equal-temperament chords with in-browser playback", "visualnoise.ca"),
    ]
}

