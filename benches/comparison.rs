#![feature(test)]

extern crate gammath;
extern crate cgmath;
extern crate nalgebra;

extern crate test;

mod vec3 {
  mod add {
    const N: usize = 1000;

    #[bench]
    fn gammath(b: &mut ::test::Bencher) {
      let v1 = ::gammath::vec3(1.0, 2.0, 3.0);
      let v2 = ::gammath::vec3(4.0, 5.0, 6.0);

      b.iter(|| (0..N).fold(v1, |acc, _| acc + v2 ) );
    }

    #[bench]
    fn cgmath(b: &mut ::test::Bencher) {
      let v1 = ::cgmath::vec3(1.0f32, 2.0, 3.0);
      let v2 = ::cgmath::vec3(4.0f32, 5.0, 6.0);

      b.iter(|| (0..N).fold(v1, |acc, _| acc + v2 ) );
    }

    #[bench]
    fn nalgebra(b: &mut ::test::Bencher) {
      let v1 = ::nalgebra::Vector3::new(1.0f32, 2.0, 3.0);
      let v2 = ::nalgebra::Vector3::new(4.0f32, 5.0, 6.0);

      b.iter(|| (0..N).fold(v1, |acc, _| acc + v2 ) );
    }
  }

  mod cross {
    const N: usize = 1000;

    #[bench]
    fn gammath(b: &mut ::test::Bencher) {
      let v1 = ::gammath::vec3(1.0, 2.0, 3.0);
      let v2 = ::gammath::vec3(4.0, 5.0, 6.0);

      b.iter(|| (0..N).fold(v1, |acc, _| acc.cross(v2) ) );
    }

    #[bench]
    fn cgmath(b: &mut ::test::Bencher) {
      let v1 = ::cgmath::vec3(1.0f32, 2.0, 3.0);
      let v2 = ::cgmath::vec3(4.0f32, 5.0, 6.0);

      b.iter(|| (0..N).fold(v1, |acc, _| acc.cross(v2) ) );
    }

    #[bench]
    fn nalgebra(b: &mut ::test::Bencher) {
      use nalgebra::Cross;

      let v1 = ::nalgebra::Vector3::new(1.0f32, 2.0, 3.0);
      let v2 = ::nalgebra::Vector3::new(4.0f32, 5.0, 6.0);

      b.iter(|| (0..N).fold(v1, |acc, _| acc.cross(&v2) ) );
    }
  }
}
