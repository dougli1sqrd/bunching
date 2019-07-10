#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate reqwest;
extern crate geo;
extern crate serde;

mod route;
mod bunch;

use geo::prelude::*;

fn main() {
    println!("Hello, world!");

    // rocket::ignite().mount("/", routes![hello, hello_person]).launch();

    let client = reqwest::Client::new();

    let response_result = client.get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=sf-muni&r=24").send();
    let route: Result<route::muni::MuniRoute, String> = match response_result {
        Ok(r) => route::muni::muni_route_from_response(r),
        Err(err) => Err(format!("{}", err))
    };
    
    // let distance = match route {
    //     Ok(r) => bunch::route_path(&route::Route::from(r)).haversine_length(),
    //     Err(_) => 0.0
    // };

    // println!("Distance for 67: {} meters", distance);

    let vehicles_result = client.get("http://webservices.nextbus.com/service/publicJSONFeed?command=vehicleLocations&a=sf-muni&r=24&t=0").send();
    let muni_vehicles_result = match vehicles_result {
        Ok(r) => route::muni::muni_vehicles_from_response(r),
        Err(err) => Err(format!("{}", err))
    };

    let vehicles = match muni_vehicles_result {
        Ok(vs) => vs.vehicles(),
        Err(_) => Vec::new()// Empty if there is an error I guess
    };


    let model_route = route::Route::from(route.unwrap());
    println!("Route distance: {}", bunch::route_path(&model_route).haversine_length());

    for vehicle in vehicles {
        let dist = bunch::vehicle_distance_from_route(&vehicle, &model_route);
        println!("Vehicle {} ==> {} m", vehicle.id, dist);
    }
}



fn response_as_string(response_result: reqwest::Result<reqwest::Response>) -> String {
    match response_result {
        Ok(mut res) => match res.text() {
            Ok(text) => text,
            Err(err) => format!("Error! {}", err)
        },
        Err(err) => format!("Error! {}", err)
    }
}

// #[get("/hello")]
// fn hello() -> &'static str {
//     "Hello, World!\n"
// }

// #[get("/hello/<name>")]
// fn hello_person(name: &RawStr) -> String {
//     format!("Hello, {}!\n", name.as_str())
// }
