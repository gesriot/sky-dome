use std::f64::consts::PI;

#[derive(Debug)]
struct GeoCoordinate {
    latitude: f64,
    longitude: f64,
}

impl GeoCoordinate {
    fn new(latitude: f64, longitude: f64) -> Self {
        GeoCoordinate {
            latitude,
            longitude,
        }
    }
}

// 1 degree of latitude is approximately 111.32 km
const KM_PER_DEGREE_LATITUDE: f64 = 111.32;

fn main() {
    let coords1 = GeoCoordinate::new(37.7621, -122.4111);     // Camera
    let coords2 = GeoCoordinate::new(37.795921, -122.466652); // Object

    let angle_radians = calculate_angle(&coords2, &coords1);

    let angle_degrees = radians_to_degrees(angle_radians);
    println!("Angle: {}", angle_degrees);
}

fn calculate_angle(a_object: &GeoCoordinate, a_camera: &GeoCoordinate) -> f64 {
    let coords_north = move_north(a_camera, 1.0);

    let bearing_camera_to_north = calculate_bearing(a_camera, &coords_north);
    let bearing_camera_to_object = calculate_bearing(a_camera, a_object);

    let angle_difference = (bearing_camera_to_object - bearing_camera_to_north).abs();
    let angle = angle_difference.min(360.0 - angle_difference);

    degrees_to_radians(angle)
}

fn move_north(coords: &GeoCoordinate, distance_km: f64) -> GeoCoordinate {
    let lat_new = coords.latitude + (distance_km / KM_PER_DEGREE_LATITUDE);
    GeoCoordinate::new(lat_new, coords.longitude)
}

fn calculate_bearing(start: &GeoCoordinate, end: &GeoCoordinate) -> f64 {
    let lat1 = degrees_to_radians(start.latitude);
    let lon1 = degrees_to_radians(start.longitude);
    let lat2 = degrees_to_radians(end.latitude);
    let lon2 = degrees_to_radians(end.longitude);

    let dlon = lon2 - lon1;

    let x = dlon.sin() * lat2.cos();
    let y = lat1.cos() * lat2.sin() - (lat1.sin() * lat2.cos() * dlon.cos());

    let initial_bearing = x.atan2(y);
    let compass_bearing = (radians_to_degrees(initial_bearing) + 360.0) % 360.0;

    compass_bearing
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}
