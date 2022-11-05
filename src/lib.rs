use pyo3::prelude::*;

/// Module for basic classes in adopt-a-doodle. This module is implemented in Rust.
#[pymodule]
fn rustydoodle(_: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Doodle>()?;
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
/// Here's an explanation of each field:
/// ```rust
/// use rustydoodle::*;
/// 
/// fn makeadoodle() -> Doodle {
///     Doodle {
///         color: (0.546875, 0.28125, 0.75, 1.0), // The color field requires a struct containing four 32-bit floating point values of values 0 through 1. The first three values are for Red, Blue, and Green content. The fourth value is the Alpha value. In this example, the color is purple.
///         eye_color: (0.242188, 0.742188, 0.515625, 1.0), // The same as color, but for the doodle's eyes. In this example, the color is sea green.
///         pattern: Pattern { // See the docs for the Pattern struct.
///             ears: Some(String::from("phase_4/maps/BeanCatEar6.jpg")),
///             body: String::from("phase_4/maps/BeanbodyLepord2.jpg"),
///             legs: String::from("phase_4/maps/BeanFoot6.jpg"),
///             tail: Some(String::from("phase_4/maps/BeanLongTailLepord.jpg"))
///         },
///         animation: Some(Animation { // See the docs for the Animation struct.
///             file: String::from("phase_5/models/char/TT_pets-speak.bam"),
///             anim_loop: true,
///             loop_from: None,
///             loop_to: None,
///             loop_restart: None,
///             pose: false,
///             pose_frame: None
///         }),
///         eyelashes: false, // A boolean that determines whether or not the doodle has eyelashes. In this case, the doodle will not have eyelashes.
///         hair: None, // An Option<String> value to determine if, and what hair style the doodle will have. See rustydoodle::hair_list() for the list of hair options.
///         ears: Some(String::from("catEars")), // An Option<String> value to determine if, and what ears the doodle will have. See rustydoodle::ear_list() for the list of ear options.
///         nose: None, // An Option<String> value to determine if, and what nose style the doodle will have. See rustydoodle::nose_list() for the list of nose options.
///         tail: Some(String::from("longTail")) // An Option<String> value to determine if, and what tail style the doodle will have. See rustydoodle::tail_list() for the list of tail options.
///     }
/// }
/// ```
/// However, since this is a Python module, you probably won't be implementing this struct like this in Rust! Here's how the same doodle would look in Python:
/// ```python
/// from rustydoodle import *
/// 
/// example_doodle = Doodle(color=(0.546875, 0.28125, 0.75, 1.0),
///     pattern=Pattern(ears="phase_4/maps/BeanCatEar6.jpg",
///         body="phase_4/maps/BeanbodyLepord2.jpg",
///         legs="phase_4/maps/BeanFoot6.jpg",
///         tail="phase_4/maps/BeanLongTailLepord.jpg"),
///     animation=Animation(file="phase_5/models/char/TT_pets-speak.bam",
///         anim_loop=True,
///         loop_from=None,
///         loop_to=None,
///         loop_restart=None,
///         pose=False,
///         pose_frame=None),
///     eyelashes=False,
///     hair=None,
///     ears="catEars",
///     nose=None,
///     tail="longTail")
/// ```
/// If using rustydoodle through adopt_a_doodle, the Doodle struct will be accessible through `adopt_a_doodle.Doodle`.
#[pyclass]
pub struct Doodle {
    #[pyo3(get,set)]
    color: (f32,f32,f32,f32), // A Color tuple for Panda3D. It is in the format of (Red,Green,Blue,Alpha)
    #[pyo3(get,set)]
    eye_color: (f32,f32,f32,f32),
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
    pub fn new(color:(f32,f32,f32,f32),eye_color:(f32,f32,f32,f32),pattern:Pattern,animation:Option<Animation>,eyelashes:bool,hair:Option<String>,ears:Option<String>,nose:Option<String>,tail:Option<String>) -> Self {
        Self {color,eye_color,pattern,animation,eyelashes,hair,ears,nose,tail}
    }
}

/// A pattern that a Doodle may have.
/// Here's an explanation for each field:
/// ```rust
/// use rustydoodle::*;
/// 
/// fn makeapattern() -> Pattern {
///     Pattern {
///     ears: Some(String::from("phase_4/maps/BeanCatEar6.jpg")), // An Option<String> value that representing a file path  leads to a doodle's ear texture. Since not every doodle will have ears, this is an optional value.
///     body: String::from("phase_4/maps/BeanbodyLepord2.jpg"), // A String value representing a file path that leads to a doodle's body texture.
///     legs: String::from("phase_4/maps/BeanFoot6.jpg"), // A String value representing a file path that leads to a doodle's foot texture.
///     tail: Some(String::from("phase_4/maps/BeanLongTailLepord.jpg")) // An Option<String> value that representing a file path  leads to a doodle's tail texture. Since not every doodle will have a tail, this is an optional value.
///     }
/// }
/// ```
/// However, since this is a Python module, you probably won't be implementing this struct like this in Rust! Here's how the same pattern would look in Python:
/// ```python
/// from rustydoodle import *
/// 
/// let examplepattern = Pattern(ears="phase_4/maps/BeanCatEar6.jpg",
///     body="phase_4/maps/BeanbodyLepord2.jpg",
///     legs="phase_4/maps/BeanFoot6.jpg",
///     tail="phase_4/maps/BeanLongTailLepord.jpg")
/// ```
/// If using rustydoodle through adopt_a_doodle, the Doodle struct will be accessible through `adopt_a_doodle.Pattern`.
#[pyclass]
#[derive(Clone)]
pub struct Pattern {
    #[pyo3(get,set)]
    ears: Option<String>,
    #[pyo3(get,set)]
    body: String,
    #[pyo3(get,set)]
    legs: String,
    #[pyo3(get,set)]
    tail: Option<String>
}

#[pymethods]
impl Pattern {

    /// Creates a new Pattern struct.
    #[new]
    pub fn new(ears:Option<String>,body:String,legs:String,tail:Option<String>) -> Self {
        Self {ears,body,legs,tail}
    }
}

/// An animation that the doodle will perform. 
#[pyclass]
#[derive(Clone)]
pub struct Animation {
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
    pub fn new(file:String,anim_loop:bool,loop_from:Option<u64>,loop_to:Option<u64>,loop_restart:Option<u64>,pose:bool,pose_frame:Option<u64>) -> Self {
        Self {file,anim_loop,loop_from,loop_to,loop_restart,pose,pose_frame}
    }
}

/// Cinnamon, a really cool doodle!
#[pyfunction]
fn cinnamon() -> Doodle {
    Doodle {
        color: (0.996094, 0.695312, 0.511719, 1.0),
        eye_color: (0.242188, 0.742188, 0.515625, 1.0),
        pattern: Pattern{
            ears: Some(String::from("phase_4/maps/BeanBunnyEar6.jpg")),
            body: String::from("phase_4/maps/BeanbodyDots6.jpg"),
            legs: String::from("phase_4/maps/BeanFoot6.jpg"),
            tail: Some(String::from("phase_4/maps/beanCatTail6.jpg"))
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
pub fn hair_list() -> Vec<String> {
    vec![String::from("feathers")]
}

/// Function returning list of ear options for doodles.
#[pyfunction]
pub fn ear_list() -> Vec<String> {
    vec![String::from("catEars"),String::from("horns"),String::from("rabbitEars"),String::from("dogEars"),String::from("antennae")]
}

/// Function returning list of nose options for doodles.
#[pyfunction]
pub fn nose_list() -> Vec<String> {
    vec![String::from("pigNose"),String::from("ovalNose"),String::from("dogNose"),String::from("clownNose")]
}

/// Function returning list of tail options for doodles.
#[pyfunction]
pub fn tail_list() -> Vec<String> {
    vec![String::from("longTail"),String::from("catTail"),String::from("bunnyTail"),String::from("birdTail")]
}

// sooo i was trying to figure out an issue i was having (pyd detection in python packages, i kept trying but it just wasn't working.) and during my efforts, my internet decided to shit itself. so instead of trying to figure it out im just going to leave the project in this format. it doesn't matter too much.
// it at least works and that's good enough for me.
// great news! i figured out how to make it work! all i had to do was read pyo3's readme!
// anyways now all i need to do here is document my code. which uh. i'll get to eventually.