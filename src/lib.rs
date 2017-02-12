use std::ops;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

#[inline]
pub fn vec3(x: f32,
            y: f32,
            z: f32) -> Vec3 {
  Vec3{ x: x, y: y, z: z }
}

impl ops::Add for Vec3 {
  type Output = Vec3;

  #[inline]
  fn add(self, other: Vec3) -> Vec3 {
    Vec3{ x: self.x + other.x,
          y: self.y + other.y,
          z: self.z + other.z }
  }
}

impl Vec3 {
  #[inline]
  pub fn cross(self, other: Vec3) -> Vec3 {
    Vec3{ x: self.y * other.z - self.z * other.y,
          y: self.z * other.x - self.x * other.z,
          z: self.x * other.y - self.y * other.x }
  }
}

#[cfg(test)]
mod tests {
  mod vec3 {
    use vec3;

    #[test]
    fn add() {
      let v1 = vec3(1.0, 2.0, 3.0);
      let v2 = vec3(4.0, 5.0, 6.0);

      let sum = v1 + v2;

      assert_eq!(sum, vec3(5.0, 7.0, 9.0));
    }

    #[test]
    fn cross() {
      let v1 = vec3(1.0, 2.0, 3.0);
      let v2 = vec3(4.0, 5.0, 6.0);

      let sum = v1.cross(v2);

      assert_eq!(sum, vec3(-3.0, 6.0, -3.0 ));
    }
  }
}
