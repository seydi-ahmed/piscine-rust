mod areas_volumes;
use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    match objects {
        GeometricalShapes::Square => {
            let area_per_object = a * a;
            let total_area = x * y;
            area_per_object * times <= total_area
        }
        GeometricalShapes::Circle => {
            // Utilisation de la moitié de la largeur ou de la hauteur du rectangle comme rayon
            let radius = a.min(b);
            let area_per_object = std::f64::consts::PI * (radius * radius) as f64;
            let total_area = x * y;
            area_per_object * times as f64 <= total_area as f64
        }
        GeometricalShapes::Rectangle => {
            let area_per_object = a * b;
            let total_area = x * y;
            area_per_object * times <= total_area
        }
        GeometricalShapes::Triangle => {
            let area_per_object = areas_volumes::triangle_area(a, b);
            let total_area = x * y;
            area_per_object * times as f64 <= total_area as f64
        }
    }
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    match objects {
        GeometricalVolumes::Cube => {
            let volume_per_object = a * a * a;
            let total_volume = x * y * z;
            volume_per_object * times <= total_volume
        }
        GeometricalVolumes::Sphere => {
            // Utilisation de la moitié de la largeur ou de la hauteur ou de la profondeur du cube comme rayon
            let radius = a.min(b).min(c);
            let volume_per_object = areas_volumes::sphere_volume(radius);
            let total_volume = x * y * z;
            volume_per_object * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Cone => {
            let volume_per_object = areas_volumes::cone_volume(a, b);
            let total_volume = x * y * z;
            volume_per_object * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Pyramid => {
            let base_area = areas_volumes::triangle_area(a, b);
            let volume_per_object = areas_volumes::triangular_pyramid_volume(base_area, c);
            let total_volume = x * y * z;
            volume_per_object * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Parallelepiped => {
            let volume_per_object = areas_volumes::parallelepiped_volume(a, b, c);
            let total_volume = x * y * z;
            volume_per_object * times <= total_volume
        }
    }
}
