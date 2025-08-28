mod sandbox;

use hazelrc::entry_point;


fn main() {
    entry_point::run(sandbox::create_application);
}
