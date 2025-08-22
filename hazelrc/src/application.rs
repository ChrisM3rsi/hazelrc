pub trait Application {
    fn run(&self) {
        println!("Application is running...");
        loop {}
    }
}

