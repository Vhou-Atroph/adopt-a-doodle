use pyo3::prelude::*;

#[pyclass]
struct Doodle {
    #[pyo3(get,set)]
    pattern: Pattern,
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