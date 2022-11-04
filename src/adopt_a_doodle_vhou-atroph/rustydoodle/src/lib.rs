use pyo3::prelude::*;

/// Module for basic classes in adopt-a-doodle. This module is implemented in Rust.
#[pymodule]
fn rustydoodle(_: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Doodle>()?;
    m.add_class::<Color>()?;
    m.add_class::<Pattern>()?;
    m.add_class::<Animation>()?;
    m.add_function(wrap_pyfunction!(cinnamon, m)?)?;
    m.add_function(wrap_pyfunction!(hair_list, m)?)?;
    m.add_function(wrap_pyfunction!(ear_list, m)?)?;
    m.add_function(wrap_pyfunction!(nose_list, m)?)?;
    m.add_function(wrap_pyfunction!(tail_list, m)?)?;
    Ok(())
}


/// A Doodle is a species that hails from Doodlevania, an area outside of Toontown owned by Deedle von Doodlesworth. Many are kept as pets by Toons, and you can adopt them at a pet shop.
#[pyclass]
struct Doodle {
    #[pyo3(get,set)]
    color: Color,
    #[pyo3(get,set)]
    eye_color: Color,
    #[pyo3(get,set)]
    pattern: Pattern,
    #[pyo3(get,set)]
    animation: Option<Animation>,
    #[pyo3(get,set)]
    eyelashes: bool,
    #[pyo3(get,set)]
    hair: Option<String>, // 'feathers' or None
    #[pyo3(get,set)]
    ears: Option<String>, // 'catEars', 'horns', 'rabbitEars', 'dogEars', 'antennae', or None
    #[pyo3(get,set)]
    nose: Option<String>, // 'pigNose', 'ovalNose', 'dogNose', 'clownNose', or None
    #[pyo3(get,set)]
    tail: Option<String>, // 'longTail', 'catTail', 'bunnyTail', 'birdTail', or None
}

#[pymethods]
impl Doodle {

    /// Creates a new Doodle struct.
    #[new]
    fn new(color:Color,eye_color:Color,pattern:Pattern,animation:Option<Animation>,eyelashes:bool,hair:Option<String>,ears:Option<String>,nose:Option<String>,tail:Option<String>) -> Self {
        Self {color,eye_color,pattern,animation,eyelashes,hair,ears,nose,tail}
    }
}

/// A Color tuple for Panda3D. It is in the format of (Red,Green,Blue,Alpha)
#[pyclass]
#[derive(Clone)]
struct Color(f32,f32,f32,f32);

#[pymethods]
impl Color {

    /// Creates a new Color tuple struct.
    #[new]
    fn new(r:f32,g:f32,b:f32,a:f32) -> Self {
        Self(r,g,b,a)
    }
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

#[pymethods]
impl Pattern {

    /// Creates a new Pattern struct.
    #[new]
    fn new(body:String,legs:Option<String>,tail:Option<String>) -> Self {
        Self {body,legs,tail}
    }
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
    pose: bool, // Takes priority over anim_loop - if this is set to true then the actor will be stuck in a pose.
    #[pyo3(get,set)]
    pose_frame: Option<u64> // Necessary parameter if pose is true. Sets the frame the doodle will be posed at.
}

#[pymethods]
impl Animation {

    /// Creates a new Animation struct.
    #[new]
    fn new(file:String,anim_loop:bool,loop_from:Option<u64>,loop_to:Option<u64>,loop_restart:Option<u64>,pose:bool,pose_frame:Option<u64>) -> Self {
        Self {file,anim_loop,loop_from,loop_to,loop_restart,pose,pose_frame}
    }
}

/// Cinnamon, a really cool doodle!
#[pyfunction]
fn cinnamon() -> Doodle {
    Doodle {
        color: Color(0.996094, 0.695312, 0.511719, 1.0),
        eye_color: Color(0.242188, 0.742188, 0.515625, 1.0),
        pattern: Pattern{
            body: String::from("phase_4/maps/BeanbodyDots6.jpg"),
            legs: None,
            tail: None
        },
        animation: Some(Animation{
            file: String::from("phase_4/models/char/TT_pets-neutralHappy.bam"),
            anim_loop: true,
            loop_from: None,
            loop_to: None,
            loop_restart: None,
            pose: false,
            pose_frame: None
        }),
        eyelashes: true,
        hair: None,
        ears: Some(String::from("rabbitEars")),
        nose: None,
        tail: Some(String::from("catTail"))
    }
}

/// Function returning list of hair options for doodles.
#[pyfunction]
fn hair_list() -> Vec<String> {
    vec![String::from("feathers")]
}

/// Function returning list of ear options for doodles.
#[pyfunction]
fn ear_list() -> Vec<String> {
    vec![String::from("catEars"),String::from("horns"),String::from("rabbitEars"),String::from("dogEars"),String::from("antennae")]
}

/// Function returning list of nose options for doodles.
#[pyfunction]
fn nose_list() -> Vec<String> {
    vec![String::from("pigNose"),String::from("ovalNose"),String::from("dogNose"),String::from("clownNose")]
}

/// Function returning list of tail options for doodles.
#[pyfunction]
fn tail_list() -> Vec<String> {
    vec![String::from("longTail"),String::from("catTail"),String::from("bunnyTail"),String::from("birdTail")]
}