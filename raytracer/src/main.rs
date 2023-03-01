use std::ops;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
    fn new_val(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    fn x(&self) -> f64 {
        self[0]
    }
    fn y(&self) -> f64 {
        self[1]
    }
    fn z(&self) -> f64 {
        self[2]
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[0] + rhs.e[0],
                self.e[0] + rhs.e[0],
            ],
        };
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs, self.e[0] * rhs, self.e[0] * rhs],
        };
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new_val(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new_val(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_val(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new_val(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new_val(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Vec3 {
    fn dot(lhs: &Self, rhs: &Self) -> f64 {
        lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
    }

    fn cross(lhs: &Self, rhs: &Self) -> Self {
        Self::new_val(
            lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
            lhs.e[2] * rhs.e[0] - lhs.e[0] * rhs.e[2],
            lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0],
        )
    }

    fn unit_vector(v: &Self) -> Self {
        *v / v.length()
    }
}

type point3 = Vec3;

struct Ray {
    orig: point3,
    dir: Vec3,
}

impl Ray {
    fn new() -> Self {
        Ray {
            orig: Vec3::new(),
            dir: Vec3::new(),
        }
    }

    fn new_val(orig: &Vec3, direction: &point3) -> Self {
        Ray {
            orig: orig.clone(),
            dir: direction.clone(),
        }
    }

    fn origin(&self) -> Vec3 {
        return self.orig.clone();
    }

    fn direction(&self) -> Vec3 {
        return self.dir.clone();
    }

    fn at(&self, t: f64) -> Vec3 {
        self.orig.clone() + (t * self.dir.clone())
    }
}

type color = Vec3;

fn write_color(pixel_color: &color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    );
}

fn ray_color(r: &Ray) -> color {
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    ((1.0 - t) * color::new_val(1.0, 1.0, 1.0)) + (t * color::new_val(0.5, 0.7, 1.0))
}
fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3::new_val(0.0, 0.0, 0.0);
    let horizontal = Vec3::new_val(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new_val(0.0, viewport_height, 0.0);
    let lower_left_corner = ((origin - (horizontal / 2 as f64)) - (vertical / 2 as f64))
        - (Vec3::new_val(0.0, 0.0, focal_length));

    // Render
    println!("P3\n {} {} \n255", image_width, image_height);

    for j in (0..image_height as usize - 1).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new_val(
                &origin,
                &(lower_left_corner + ((u * horizontal) + ((v * vertical) - origin))),
            );

            let pixel_color = ray_color(&r);
            write_color(&pixel_color)
        }
    }
}
