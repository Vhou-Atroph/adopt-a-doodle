use pyo3::prelude::*;

/// A Doodle is a species that hails from Doodlevania, an area outside of Toontown owned by Deedle von Doodlesworth. Many are kept as pets by Toons, and you can adopt them at a pet shop.
#[pyclass]
struct Doodle {
    #[pyo3(get,set)]
    pattern: Pattern,
    #[pyo3(get,set)]
    animation: Option<Animation>,
    #[pyo3(get,set)]
    position: (u64,u64,u64) // Tuple of values to get the doodle's position in a 3D space.
}

/// A pattern that a Doodle may have.
#[pyclass]
#[derive(Clone)]
struct Pattern {
    #[pyo3(get,set)]
    body: String,
    #[pyo3(get,set)]
    legs: Option<String>, // Some doodle patterns affect more than just the body of the doodle- having an Option here will handle those.
    #[pyo3(get,set)]
    tail: Option<String>
}

/// An animation that the doodle will perform. 
#[pyclass]
#[derive(Clone)]
struct Animation {
    #[pyo3(get,set)]
    file: String,
    #[pyo3(get,set)]
    anim_loop: bool,
    #[pyo3(get,set)]
    loop_from: Option<u64>,
    #[pyo3(get,set)]
    loop_to: Option<u64>,
    #[pyo3(get,set)]
    loop_restart: Option<u64>,
    #[pyo3(get,set)]
    pose: bool, // takes priority over anim_loop - if this is set to true then the actor will be stuck in a pose.
    #[pyo3(get,set)]
    pose_frame: Option<u64>
}