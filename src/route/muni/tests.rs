use super::*;

#[test]
fn de_serialize_muni_point() {

    let muni_point_from_str: MuniPoint = serde_json::from_str(r#"{"lon": "-122.41138", "lat": "37.73984"}"#).unwrap();
    println!("{:?}", muni_point_from_str);
    assert_eq!(muni_point_from_str, MuniPoint{ lon: -122.41138, lat: 37.73984});
}

#[test]
fn de_serialize_muni_stop() {
    let s = r#"
    {
        "lon": "-122.41833",
        "title": "24th St & Mission St",
        "stopId": "13476",
        "tag": "3476",
        "lat": "37.7523199"
    }"#;
    let muni_stop: MuniStop = serde_json::from_str(s).unwrap();
    assert_eq!(muni_stop, MuniStop {title: String::from("24th St & Mission St"), stopId: String::from("13476"), tag: String::from("3476"), lat: 37.7523199, lon: -122.41833});
}

#[test]
fn de_serialize_muni_path() {
    let s = r#"{"point": [
          {
            "lon": "-122.4133",
            "lat": "37.73888"
          },
          {
            "lon": "-122.41332",
            "lat": "37.73899"
          },
          {
            "lon": "-122.4132",
            "lat": "37.739"
          }
        ]}"#;
    let muni_path: MuniPath = serde_json::from_str(s).unwrap();
    let expected = MuniPath { point: vec![MuniPoint {lat: 37.73888, lon: -122.4133}, MuniPoint {lat: 37.73899, lon: -122.41332}, MuniPoint {lat: 37.739, lon: -122.4132}] };
    assert_eq!(muni_path, expected);
}

#[test]
fn de_serialize_muni_direction() {
    let s = r#"{
        "stop": [
          {
            "tag": "3476"
          },
          {
            "tag": "7532"
          }
        ],
        "title": "Outbound to Alemany + Ellsworth",
        "useForUI": "true",
        "tag": "67___O_F00",
        "name": "Outbound"
    }"#;
    let muni_direction: MuniDirection = serde_json::from_str(s).unwrap();
    let expected = MuniDirection {
        stop: vec![MuniTag{tag: String::from("3476")}, MuniTag{tag: String::from("7532")}], 
        title: String::from("Outbound to Alemany + Ellsworth"),
        name: String::from("Outbound"),
        useForUI: true,
        tag: String::from("67___O_F00")
    };
    assert_eq!(muni_direction, expected);
}

#[test]
fn de_serialize_muni_route() {
    let s = r#"{
        "latMax": "37.7525599",
        "lonMax": "-122.40934",
        "latMin": "37.73262",
        "lonMin": "-122.41893",
        "title": "67-Bernal Heights",
        "tag": "67",
        "color": "555555",
        "oppositeColor": "000000",
        "path": [
            {
                "point": [
                    {
                        "lat": "37.73491",
                        "lon": "-122.41128"
                    },
                    {
                        "lat": "37.73508",
                        "lon": "-122.41118"
                    }
                ]
            },
            {
                "point": [
                    {
                        "lat": "37.73471",
                        "lon": "-122.41491"
                    },
                    {
                        "lat": "37.73474",
                        "lon": "-122.4137"
                    }
                ]
            }
        ],
        "stop": [
            {
                "lon": "-122.41833",
                "title": "24th St & Mission St",
                "stopId": "13476",
                "tag": "3476",
                "lat": "37.7523199"
            }
        ],
        "direction": [
            {
                "stop": [
                    { "tag": "3476" },
                    { "tag": "7532" }
                ],
                "title": "Outbound to Alemany + Ellsworth",
                "useForUI": "true",
                "tag": "67___O_F00",
                "name": "Outbound"
            }
        ],
        "copyright": "All data copyright San Francisco Muni 2019."
    }"#;

    let route: MuniRoute = serde_json::from_str(s).unwrap();
    println!("{:?}", route);
    assert_eq!(route.title, String::from("67-Bernal Heights"));
    assert_eq!(route.direction.len(), 1);
    assert_eq!(route.direction[0].stop.len(), 2);
}