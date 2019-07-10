use geo;
use geo::*;
use geo::prelude::*;

use crate::route::*;

pub fn route_path(route: &Route) -> geo::LineString<f64> {
    let mut points: Vec<Coordinate<f64>> = route.stops().iter().map(|s| {
        Coordinate::from(s.location)
    }).collect();
    points.push(Coordinate::from(route.stops()[0].location));
    geo::LineString::from(points)
}

pub fn vehicle_distance_from_route(vehicle: &Vehicle, route: &Route) -> f64 {
    let geo_route = route_path(route);
    vehicle.location.euclidean_distance(&geo_route)
}

