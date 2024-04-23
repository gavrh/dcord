mod settings;

pub use settings::*;


pub struct Cache {}
impl Cache {
    pub fn new() -> Self {
        Self {}
    }
}