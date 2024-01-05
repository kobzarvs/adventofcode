use std::f64;
use std::fs::File;
use std::io::{self, BufRead};
use nalgebra::{Point3, Vector3};

use geo::{Contains, Coord, coord, Line, LineIntersection, Rect, Vector2DOps};
use geo::line_intersection::line_intersection;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Clone)]
struct Hailstone {
    id: i64,
    vx: i64,
    vy: i64,
    vz: i64,
    positions: Vec<Point3D>,
}

fn raw_line_intersection(p: Line<f64>, q: Line<f64>) -> Option<Coord<f64>> {
    let p_min_x = p.start.x.min(p.end.x);
    let p_min_y = p.start.y.min(p.end.y);
    let p_max_x = p.start.x.max(p.end.x);
    let p_max_y = p.start.y.max(p.end.y);

    let q_min_x = q.start.x.min(q.end.x);
    let q_min_y = q.start.y.min(q.end.y);
    let q_max_x = q.start.x.max(q.end.x);
    let q_max_y = q.start.y.max(q.end.y);

    let int_min_x = p_min_x.max(q_min_x);
    let int_max_x = p_max_x.min(q_max_x);
    let int_min_y = p_min_y.max(q_min_y);
    let int_max_y = p_max_y.min(q_max_y);

    let two = 1.0f64 + 1.0f64;
    let mid_x = (int_min_x + int_max_x) / two;
    let mid_y = (int_min_y + int_max_y) / two;

    // condition ordinate values by subtracting midpoint
    let p1x = p.start.x - mid_x;
    let p1y = p.start.y - mid_y;
    let p2x = p.end.x - mid_x;
    let p2y = p.end.y - mid_y;
    let q1x = q.start.x - mid_x;
    let q1y = q.start.y - mid_y;
    let q2x = q.end.x - mid_x;
    let q2y = q.end.y - mid_y;

    // unrolled computation using homogeneous coordinates eqn
    let px = p1y - p2y;
    let py = p2x - p1x;
    let pw = p1x * p2y - p2x * p1y;

    let qx = q1y - q2y;
    let qy = q2x - q1x;
    let qw = q1x * q2y - q2x * q1y;

    let xw = py * qw - qy * pw;
    let yw = qx * pw - px * qw;
    let w = px * qy - qx * py;

    let x_int = xw / w;
    let y_int = yw / w;

    // check for parallel lines
    if (x_int.is_nan() || x_int.is_infinite()) || (y_int.is_nan() || y_int.is_infinite()) {
        None
    } else {
        // de-condition intersection point
        Some(coord! {
            x: x_int + mid_x,
            y: y_int + mid_y,
        })
    }
}


fn main() -> io::Result<()> {
    // let min_dist = 200000000000000.0;
    // let max_dist = 400000000000000.0;
    // let divider = 100_000_000f64;
    // let min_dist = 200_000_000_000_000.0f64 / divider;
    // let max_dist = 400_000_000_000_000.0f64 / divider;

    let mut rays: Vec<Line<f64>> = Vec::new();
    let mut hailstones: Vec<Hailstone> = Vec::new();

    let test_area = Rect::new(
        Coord { x: 7.0, y: 7.0 },
        Coord { x: 27.0, y: 27.0 },
    );

    // let test_area = Rect::new(
    //     Coord { x: min_dist, y: min_dist },
    //     Coord { x: max_dist, y: max_dist },
    // );

    let mut id = 0;
    let file = File::open("test.txt")?;
    for line_result in io::BufReader::new(file).lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.split('@').collect();

        if parts.len() == 2 {
            let c: Vec<f64> = parts[0].split(',').filter_map(|s| s.trim().parse().ok()).collect();
            let dv: Vec<f64> = parts[1].split(',').filter_map(|s| s.trim().parse().ok()).collect();
            let c_int: Vec<i64> = parts[0].split(',').filter_map(|s| s.trim().parse().ok()).collect();
            let dv_int: Vec<i64> = parts[1].split(',').filter_map(|s| s.trim().parse().ok()).collect();
            // let mut ray = Line::new(
            //     coord! {x: c[0]/divider, y: c[1]/divider},
            //     coord! { x: c[0]/divider + dv[0]*1_000_000.0f64, y: c[1]/divider + dv[1]*1_000_000.0f64 },
            // );
            let mut ray = Line::new(
                coord! {x: c[0], y: c[1]},
                coord! { x: c[0] + dv[0]*1_000_000.0f64, y: c[1] + dv[1]*1_000_000.0f64 },
            );
            rays.push(ray);

            let hailstone = Hailstone {
                id,
                vx: i64::from(dv_int[0]),
                vy: i64::from(dv_int[1]),
                vz: i64::from(dv_int[2]),
                positions: vec![Point3D { x: c_int[0], y: c_int[1], z: c_int[2] }],
            };
            id += 1;
            hailstones.push(hailstone.clone());
            println!("{:?}", hailstone);
        } else {
            println!("Invalid input format");
        }
    }

    // 18651
    if let Some(cross) = part_1(rays, test_area) {
        if cross.
        is_empty()
        {
            println!("Нет пересекающихся векторов");
            return Ok(())
        }
    }
    //-----------------------------------------//

    if let Some(point) = raw_line_intersection(cross[0], cross[1]) {
        if let Some(normal) = point.try_normalize() {
            println!("Normal: {:?}", normal);
            let point_on_plane = Point3::origin();
            let a = normal.x;
            let b = normal.y;
            let c = normal.z;
            let d = -a * point_on_plane.x - b * point_on_plane.y - c * point_on_plane.z;

            // Начальная точка прямой
            let p0 = Point3::new(12.0, 31.0, 28.0);

            // Направляющий вектор прямой
            let v = Vector3::new(-1.0, -2.0, -1.0).normalize();

            // Находим параметр t
            let t = -(a * p0.x + b * p0.y + c * p0.z + d) / (a * v.x + b * v.y + c * v.z);

            // Находим точку пересечения
            let intersection_point = p0 + t * v;

            println!("Точка пересечения: {:?}", intersection_point);
        }
    }

    Ok(())
}

fn part_1(mut rays: Vec<Line>, test_area: Rect) -> Option<Coord> {
    let mut result = 0;
    let mut k = 1;
    let mut cross = None;
    rays.iter().enumerate().for_each(|(i, a)| {
        for j in i + 1..rays.len() {
            let x_point = raw_line_intersection(rays[i], rays[j]);
            let crossed = line_intersection(rays[i], rays[j]);
            if x_point.is_some() && crossed.is_some() {
                let valid = test_area.contains(&x_point.unwrap());
                result += if valid { 1 } else { 0 };
                println!("#{}: [{}] {:?} {:?}", k, if valid { '+' } else { '.' }, rays[i], rays[j]);
                k += 1;
                if valid && cross.is_empty() {
                    cross = x_point;
                }
            } else {
                println!("#{}: [ ] {:?}", k, rays[i]);
            }
        }
    });

    println!("Part I: {}", result);

    cross
}

fn find_projection_intersection_plane(origin1: Point3<f64>, direction1: Vector3<f64>,
                                      origin2: Point3<f64>, direction2: Vector3<f64>,
                                      origin3: Point3<f64>, direction3: Vector3<f64>) -> (Point3<f64>, Vector3<f64>) {
    // Выбираем плоскость XY для примера
    let normal = Vector3::new(0.0, 0.0, 1.0);

    // Находим направляющие векторы для проекций лучей на плоскость
    let projected_direction1 = direction1 - direction1.dot(&normal) * normal;
    let projected_direction2 = direction2 - direction2.dot(&normal) * normal;
    let projected_direction3 = direction3 - direction3.dot(&normal) * normal;

    // Теперь находим точку пересечения проекций на плоскость
    let intersection_point = origin1 + find_intersection(projected_direction1, origin1 - origin2, origin1 - origin3, origin1, normal);

    (intersection_point, normal)
}

fn find_intersection(
    v1: Vector3<f64>,
    v2: Vector3<f64>,
    v3: Vector3<f64>,
    p: Point3<f64>,
    n: Vector3<f64>,
) -> Vector3<f64> {
    let d = v1.dot(&v2.cross(&v3));
    let numer = n.dot(&(p.coords - v1));
    let t = numer / d;

    let result = p + t * v1;

    Vector3::new(result.x, result.y, result.z)
}

fn are_vectors_parallel(v1: Vector3<f64>, v2: Vector3<f64>) -> bool {
    // Проверяем, являются ли векторы коллинеарными
    v1.cross(&v2).norm() < 1e-6
}
