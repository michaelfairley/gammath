#![feature(test)]

extern crate gammath;
extern crate cgmath;
extern crate nalgebra;

extern crate rand;
extern crate test;

mod vec3 {
  mod add {
    const N: usize = 100;

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
    const N: usize = 100;

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
      let v1 = ::nalgebra::Vector3::new(1.0f32, 2.0, 3.0);
      let v2 = ::nalgebra::Vector3::new(4.0f32, 5.0, 6.0);

      b.iter(|| (0..N).fold(v1, |acc, _| acc.cross(&v2) ) );
    }
  }

  mod magnitude {
    const N: usize = 100;

    fn gen<F, T>(f: F) -> Vec<T>
      where F: Fn(f32, f32, f32) -> T {
      use ::rand::Rng;

      let mut rng = ::rand::XorShiftRng::new_unseeded();

      (0..N).map(|_|
        f(rng.next_f32(), rng.next_f32(), rng.next_f32())
      ).collect::<Vec<_>>()
    }

    #[bench]
    fn gammath(b: &mut ::test::Bencher) {
      let vs = gen(::gammath::vec3);

      b.iter(|| vs.iter().fold(0.0, |acc, item| acc + item.magnitude() ) );
    }


    #[bench]
    fn cgmath(b: &mut ::test::Bencher) {
      use cgmath::InnerSpace;

      let vs = gen(::cgmath::vec3);

      b.iter(|| vs.iter().fold(0.0, |acc, item| acc + item.magnitude() ) );
    }


    #[bench]
    fn nalgebra(b: &mut ::test::Bencher) {
      let vs = gen(::nalgebra::Vector3::new);

      b.iter(|| vs.iter().fold(0.0, |acc, item| acc + item.norm() ) );
    }
  }
}
