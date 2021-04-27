use std::fmt;

struct Structure(i32);

// To use '{}', must implement 'fmt::Display' manually
impl fmt::Display for Structure {

    // Trait requires 'fmt' with this exact signature
    // f is a buffer, which the formatted string must be written to
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {


        write!(f, "{}", self.0)
    }
}


#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y:{}", self.x, self.y)
    }
}