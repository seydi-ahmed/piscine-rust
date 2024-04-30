pub mod areas_volumes;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    match objects {
        areas_volumes::GeometricalShapes::Square => {
            let area_of_one = areas_volumes::square_area(a);
            let total_area = x * y;
            total_area / area_of_one >= times
        }
        areas_volumes::GeometricalShapes::Triangle => {
            let area_of_one = areas_volumes::triangle_area(a, b);
            let total_area = x * y;
            total_area as f64 / area_of_one >= times as f64
        }
        areas_volumes::GeometricalShapes::Circle => false,
        areas_volumes::GeometricalShapes::Rectangle => {
            let area_of_one = areas_volumes::rectangle_area(a, b);
            let total_area = x * y;
            total_area / area_of_one >= times
        }
    }
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    match objects {
        areas_volumes::GeometricalVolumes::Cube => {
            let volume_of_one = areas_volumes::cube_volume(a);
            let total_volume = x * y * z;
            total_volume / volume_of_one >= times
        }
        areas_volumes::GeometricalVolumes::Sphere => false,
        areas_volumes::GeometricalVolumes::Cone => false,
        areas_volumes::GeometricalVolumes::Pyramid => false,
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            let volume_of_one = areas_volumes::parallelepiped_volume(a, b, c);
            let total_volume = x * y * z;
            total_volume / volume_of_one >= times
        }
    }
}
