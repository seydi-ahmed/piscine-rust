pub mod areas_volumes;
pub use areas_volumes::*;

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
            let square_area = crate::areas_volumes::square_area(a);
            let total_area = x * y;
            square_area * times <= total_area
        }
        GeometricalShapes::Circle => {
            let circle_area = crate::areas_volumes::circle_area(a);
            let total_area = x * y;
            circle_area * times as f64 <= total_area as f64
        }
        GeometricalShapes::Rectangle => {
            let rectangle_area = crate::areas_volumes::rectangle_area(a, b);
            let total_area = x * y;
            rectangle_area * times <= total_area
        }
        GeometricalShapes::Triangle => {
            let triangle_area = crate::areas_volumes::triangle_area(a, b);
            let total_area = x * y;
            triangle_area * times as f64 <= total_area as f64
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
            let cube_volume = crate::areas_volumes::cube_volume(a);
            let total_volume = x * y * z;
            cube_volume * times <= total_volume
        }
        GeometricalVolumes::Sphere => {
            let sphere_volume = crate::areas_volumes::sphere_volume(a);
            let total_volume = x * y * z;
            sphere_volume * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Cone => {
            let cone_volume = crate::areas_volumes::cone_volume(a, b);
            let total_volume = x * y * z;
            cone_volume * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Pyramid => {
            let pyramid_volume = crate::areas_volumes::triangular_pyramid_volume(
                crate::areas_volumes::triangle_area(a, b),
                c,
            );
            let total_volume = x * y * z;
            pyramid_volume * times as f64 <= total_volume as f64
        }
        GeometricalVolumes::Parallelepiped => {
            let parallelepiped_volume =
                crate::areas_volumes::parallelepiped_volume(a, b, c);
            let total_volume = x * y * z;
            parallelepiped_volume * times <= total_volume
        }
    }
}
