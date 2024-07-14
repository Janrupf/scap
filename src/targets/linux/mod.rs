use super::{Display, Target};

// On Linux, the target is selected when a Recorder is instanciated because this
// requires user interaction
pub fn get_all_targets() -> Vec<Target> {
    Vec::new()
}

pub fn get_main_display() -> Display {
    // Return fake data, there is no such concept as a main display
    Display {
        title: "Fake display".to_string(),
        id: 0,
    }
}

pub fn get_target_dimensions(_target: &Target) -> (u64, u64) {
    // Unknown on Linux
    (0, 0)
}
