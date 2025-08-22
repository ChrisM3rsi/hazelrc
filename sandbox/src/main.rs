mod sandbox;

use hazelrc::{application::{self, Application}, entry_point};


fn main() {
    println!("Starting application...");
    entry_point::run(sandbox::create_application);
}
