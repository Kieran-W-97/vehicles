enum Speed {
    Fast = 1,
    Slow = 0,
    WarpSpeed = 10,
    MikeSpeed = -10,
}

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

pub struct VehicleState {
    speed: Speed,
    color: Color,
    ignition_on: bool,
}

pub trait Vehicle {
    fn new(_name: &'static str) -> Self;

    fn mut_vehicle_state(&mut self) -> &mut VehicleState;

    fn vehicle_state(&self) -> &VehicleState;

    fn ignition(&mut self) {
        self.mut_vehicle_state().ignition_on = true;
    }

    fn drive(&self) {
        if !self.vehicle_state().ignition_on {
            println!("Ignition Off, Cannot Drive.");
        } else {
            println!("{}", self.vroom());
        }
    }

    fn vroom(&self) -> String {
        String::from("Vroom")
    }

    fn speedometer(&self) -> String {
        if self.vehicle_state().ignition_on {
            match self.vehicle_state().speed {
                Speed::Fast => String::from("200km/h"),
                Speed::WarpSpeed => String::from(r"h\mk002"),
                Speed::Slow => String::from("50km/h"),
                Speed::MikeSpeed => String::from("undetectable speed."),
            }
        } else {
            String::from("0 km/h")
        }
    }
}

pub struct Tractor {
    vehicle_state: VehicleState,
}

impl Vehicle for Tractor {
    fn new(_name: &'static str) -> Tractor {
        Tractor {
            vehicle_state: VehicleState {
                speed: Speed::MikeSpeed,
                color: Color::Green,
                ignition_on: false,
            },
        }
    }

    fn mut_vehicle_state(&mut self) -> &mut VehicleState {
        &mut self.vehicle_state
    }

    fn vehicle_state(&self) -> &VehicleState {
        &self.vehicle_state
    }
}
