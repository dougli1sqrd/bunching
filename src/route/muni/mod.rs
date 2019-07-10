use serde::{Deserialize, Deserializer};
use serde::de;
use reqwest::Response;
use std::str::FromStr;
use std::fmt::Display;

use crate::route::*;

#[cfg(test)]
mod tests;

pub fn muni_route_from_response(mut response: Response) -> Result<MuniRoute, String> {
    match response.json::<MuniTopRoute>() {
        Ok(route) => Ok(route.route),
        Err(err) => Err(format!("{}", err))
    }
}

pub fn muni_vehicles_from_response(mut response: Response) -> Result<MuniVehicleLocations, String> {
    match response.json::<MuniVehicleLocations>() {
        Ok(vehicles) => Ok(vehicles),
        Err(err) => Err(format!("{}", err))
    }
}

// pub fn muni_from_response<'de, T: Deserialize<'de>>(mut response: Response) -> Result<T, String> 
//     where T: serde::Deserialize<'de> {
//     match response.json::<T>() {
//         Ok(m) => Ok(m),
//         Err(err) => Err(format!("{}", err))
//     }
// }

// pub fn muni_route_from_text(json_string: String) -> Result<MuniRoute, String>{
//     match serde_json::from_str(&json_string) {
//         Ok(route) => Ok(route),
//         Err(message) => Err(format!("{}", message))
//     }
// }

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de> {

    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

/// This only exists because the top json object
/// has route: MuniRoute. MuniRoute is the real object
/// we care about.
#[derive(Deserialize, Debug, PartialEq)]
pub struct MuniTopRoute {
    route: MuniRoute,
    copyright: String
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MuniRoute {
    #[serde(deserialize_with = "from_str")]
    latMax: f64,
    #[serde(deserialize_with = "from_str")]
    lonMax: f64,
    #[serde(deserialize_with = "from_str")]
    latMin: f64,
    #[serde(deserialize_with = "from_str")]
    lonMin: f64,
    title: String,
    color: String,
    oppositeColor: String,
    tag: String,
    path: Vec<MuniPath>,
    stop: Vec<MuniStop>,
    direction: Vec<MuniDirection>
}

#[derive(Deserialize, Debug, PartialEq)]
struct MuniDirection {
    title: String,
    name: String,
    #[serde(deserialize_with = "from_str")]
    useForUI: bool,
    tag: String,
    stop: Vec<MuniTag>
}

#[derive(Deserialize, Debug, PartialEq)]
struct MuniTag {
    tag: String
}

///This is called "point" in the muni json api
#[derive(Deserialize, Debug, PartialEq)]
struct MuniPath {
    point: Vec<MuniPoint>
}

#[derive(Deserialize, Debug, PartialEq)]
struct MuniPoint {
    #[serde(deserialize_with = "from_str")]
    lat: f64,
    #[serde(deserialize_with = "from_str")]
    lon: f64
}

#[derive(Deserialize, Debug, PartialEq)]
struct MuniStop {
    title: String,
    stopId: String,
    tag: String,
    #[serde(deserialize_with = "from_str")]
    lon: f64,
    #[serde(deserialize_with = "from_str")]
    lat: f64
}

/// Converts a MuniStop to Stop
impl From<&MuniStop> for Stop {
    fn from(stop: &MuniStop) -> Self {
        Stop {
            name: stop.title.clone(),
            id: stop.stopId.clone(),
            tag: stop.tag.clone(),
            location: geo::Point::new(stop.lat, stop.lon)
        }
    }
}

/// Converts a MuniRoute into a Route
impl From<MuniRoute> for Route {
    fn from(route: MuniRoute) -> Self {
        Route {
            full_name: route.title,
            line: route.tag,
            stops: route.stop.iter().map(|s| { Stop::from(s) }).collect()
        }
    }
}

// --------- VehicleLocation ------------------------------------

#[derive(Deserialize, Debug, PartialEq)]
pub struct MuniVehicleLocations {
    vehicle: Vec<MuniVehicle>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct MuniVehicle {
    id: String,
    #[serde(deserialize_with = "from_str")]
    lat: f64,
    #[serde(deserialize_with = "from_str")]
    lon: f64,
    routeTag: String,
    #[serde(deserialize_with = "from_str")]
    predictable: bool,
    #[serde(deserialize_with = "from_str")]
    speedKmHr: f64,
    #[serde(deserialize_with = "from_str")]
    heading: f64,
    #[serde(deserialize_with = "from_str")]
    secsSinceReport: f64
}

impl MuniVehicleLocations {
    pub fn vehicles(&self) -> Vec<Vehicle> {
        self.vehicle.iter().map( | v | { Vehicle::from(v) }).collect()
    }
}

impl From<&MuniVehicle> for Vehicle {
    fn from(vehicle: &MuniVehicle) -> Self {
        Vehicle {
            id: vehicle.id.clone(),
            location: Point::from((vehicle.lat, vehicle.lon))
        }
    }
}


