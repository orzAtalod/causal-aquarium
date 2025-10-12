use std::ops::Deref;
use crate::ou_process;

/// A simple recorder to keep track of the history of a variable
/// 
/// T: Clone - the type of the variable to be recorded
/// 
/// history: Vec<T> - the history of the variable
/// 
/// the function `record` will clone a new copy into the vector, because it's affordable for most data entry to clone 
/// and clone could avoid lifetime and ownership issues 
pub struct Recorder<T: Clone> {
    pub history: Vec<T>,
}

/// deref to Vec<T>, which enables Recorder used as Vec<T> (by Rust auto-deref feature)
impl<T: Clone> Deref for Recorder<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.history
    }
}

impl<T: Clone> Default for Recorder<T> {
    fn default() -> Self {
        Recorder { history: Vec::new() }
    }
}

impl<T: Clone> Recorder<T> {
    pub fn new(initial: T) -> Self {
        Recorder {
            history: vec![initial],
        }
    }

    pub fn record(&mut self, value: &T) {
        self.history.push(value.clone());
    }
}