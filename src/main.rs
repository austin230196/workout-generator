use std::{thread, time::Duration};

mod lib;


use lib::{Cacher, helpers};


fn main() {
    let mut workout_alogorithm = Cacher::new(|num| {
        println!("Generating workout from intensity: {}", num);
        thread::sleep(Duration::from_secs(3));
        num
    });

    let intensity = helpers::get_intensity().unwrap();
    let value = workout_alogorithm.get_value(intensity);

    if value < 5 {
        println!("Do {} pushups and also {} situps", value * 10, value * 5);
    }else {
        println!("Do {} pushups and also {} situps", value * 5, value * 4);
    }

}
