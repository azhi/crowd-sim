extern crate anymap;

use std::f64;

use self::anymap::AnyMap;
use ::simulation::person::Person;

pub struct Statistics {
    pub travel_time: NumStatisticItem,
}

pub struct NumStatisticItem {
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub sum_of_squares: f64,
    pub count: u32,
}

impl NumStatisticItem {
    pub fn new() -> NumStatisticItem {
        return NumStatisticItem{ min: f64::INFINITY, max: -f64::INFINITY,
                                 sum: 0_f64, sum_of_squares: 0_f64, count: 0 }
    }

    pub fn update_from_value(&mut self, value: f64) {
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }
        self.sum += value;
        self.sum_of_squares += value * value;
        self.count += 1;
    }

    pub fn current_avg(&self) -> f64 {
        return self.sum / (self.count as f64);
    }

    pub fn current_variance(&self) -> f64 {
        let avg = self.current_avg();
        return self.sum_of_squares / (self.count as f64) - avg * avg;
    }

    pub fn current_std_deviation(&self) -> f64 {
        return f64::sqrt(self.current_variance());
    }
}

impl Statistics {
    pub fn new(_configuration: &AnyMap) -> Statistics {
        Statistics{ travel_time: NumStatisticItem::new() }
    }

    pub fn update_from_reached_destination_people(&mut self, people: Vec<Person>, current_time: f64) {
        for _person in people.iter() {
            self.travel_time.update_from_value(current_time);
        }
    }
}
