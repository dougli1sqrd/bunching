use geo;
use geo::{Point};

pub mod muni;

#[derive(PartialEq, Debug, Clone)]
pub struct Route {
    full_name: String,
    line: String,
    stops: Vec<Stop>
}

impl Route {
    pub fn stops(&self) -> &Vec<Stop> {
        &self.stops
    }

    /// Return a copy of the short name of the Route
    pub fn line(&self) -> String {
        self.line.clone()
    }

    /// Returns a copy of the full name of the Route
    pub fn name(&self) -> String {
        self.full_name.clone()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Stop {
    name: String,
    id: String,
    tag: String,
    pub location: Point<f64>
}

#[derive(PartialEq, Debug)]
pub struct Vehicle {
    pub id: String,
    pub location: Point<f64>
}
