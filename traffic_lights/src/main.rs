/*
    Define an enum named TrafficLight with three variants: Red, Yellow, and Green.

    Implement a function named next that takes a TrafficLight as a parameter and returns the next state of the traffic light. The state transitions should follow this pattern:
        Red -> Green
        Green -> Yellow
        Yellow -> Red

    Write a main function that demonstrates the functionality of your traffic light simulator. It should start in the Red state and print the current state of the traffic light, then transition to the next state, print the new state, and continue this cycle for a few iterations
*/

use core::fmt;

enum TrafficLight {
    Red,
    Amber,
    Green,
}

impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TrafficLight::Red => write!(f, "Red"),
            TrafficLight::Green => write!(f, "Green"),
            TrafficLight::Amber => write!(f, "Amber"),
        }
    }
}

fn main() {
    let mut number = 0;
    let limit = 10;
    let mut light = TrafficLight::Red;

    while number < limit {
        let light_str = light.to_string();
        println!("The light is {:?}", light_str);
        light = next(&light);
        number += 1;
    }
}

fn next(light: &TrafficLight) -> TrafficLight {
    return match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Green => TrafficLight::Amber,
        TrafficLight::Amber => TrafficLight::Red,
    };
}
