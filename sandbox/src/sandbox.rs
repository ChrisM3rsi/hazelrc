use hazelrc::application::Application;

pub struct Sandbox;

impl Application for Sandbox {
}

pub fn create_application() -> Box<dyn Application> {
    Box::new(Sandbox)
}