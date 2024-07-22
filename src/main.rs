mod hello_world;
mod vehicles;
use vehicles::Vehicle;

fn main() {
    // Hello World.
    hello_world::say_hello();

    // Vehicle Traits test.
    let mut tractor : vehicles::Tractor = vehicles::Vehicle::new("Trevor");
    tractor.drive();
    tractor.ignition();
    tractor.drive();
    println!("{}", tractor.speedometer());
}
