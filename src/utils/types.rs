use std::io;

/// Methods are passed input as a line iterator on the input file
pub type Input = Box<dyn Iterator<Item = io::Result<String>>>;
