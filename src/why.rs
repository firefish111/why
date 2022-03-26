extern crate num;

use num::PrimInt;
use std::ops::*;

#[derive(Debug)]
pub struct Why {
  a: i128,
  b: i128,
  c: i128,
  d: i128,
  e: i128,
  f: i128,
  g: i128,
  h: i128,
  i: i128,
  j: i128,
  k: i128,
  l: i128,
  m: i128,
  n: i128,
  o: i128,
  p: i128,
  q: i128,
  r: i128,
  s: i128,
  t: i128,
  u: i128,
  v: i128,
  w: i128,
  x: i128,
  y: i128,
  z: i128,
}

impl Why {
  pub fn new() -> Self {
    Why {
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      f: 0,
      g: 0,
      h: 0,
      i: 0,
      j: 0,
      k: 0,
      l: 0,
      m: 0,
      n: 0,
      o: 0,
      p: 0,
      q: 0,
      r: 0,
      s: 0,
      t: 0,
      u: 0,
      v: 0,
      w: 0,
      x: 0,
      y: 0,
      z: 0,
    }
  }
}

impl<I> Add<I> for Why 
  where I: PrimInt,
    i128: AddAssign<I>,
    i128: SubAssign<I>,
    i128: MulAssign<I>,
    i128: DivAssign<I>,
    i128: RemAssign<I>,
    i128: BitXorAssign<I>
  {
  type Output = Self;

  fn add(self, operand: I) -> Self::Output {
    let mut res = self;

    res.a += operand;
    res.b += operand + I::one();
    res.c += operand;
    res.d += operand;
    res.e -= operand + I::one();
    res.f += operand;
    res.g *= operand;
    res.h += operand + I::one();
    res.i += operand - I::one();
    res.j += operand;
    res.k -= operand;
    res.l *= operand - I::one();
    res.m /= operand;
    res.n /= operand + I::one();
    res.o += operand * I::from(4i128).unwrap();
    res.p -= operand - I::from(7i128).unwrap();
    res.q += operand;
    res.r += operand;
    res.s %= operand * I::from(15i128).unwrap();
    res.t -= operand;
    res.u ^= operand + I::one();
    res.v += operand - I::from(-9i128).unwrap();
    res.w -= operand * I::from(-17i128).unwrap();
    res.x += operand + I::one();
    res.y += I::from(res.e + res.m + res.r * res.g).unwrap();
    res.z *= operand;

    res
  }
}

impl Add<Why> for Why {
  type Output = Self;

  fn add(self, operand: Self::Output) -> Self::Output {
    let mut res = self;

    res.a += operand.z;
    res.b += operand.y;
    res.c += operand.x;
    res.d += operand.w * 2i128;
    res.e -= operand.v;
    res.f += operand.u;
    res.g += operand.t;
    res.h += operand.s + 1i128;
    res.i += operand.r - 7i128;
    res.j *= operand.q;
    res.k += operand.p;
    res.l += operand.o;
    res.m += operand.n + 19i128;
    res.n += operand.m;
    res.o %= (operand.l + 1i128) * 31i128;
    res.p += operand.k;
    res.q += operand.j + 1i128;
    res.r += operand.i;
    res.s *= operand.h;
    res.t += i128::pow(operand.g, 5);
    res.u += operand.f;
    res.v /= operand.e;
    res.w += operand.d;
    res.x += operand.c / 2i128;
    res.y += operand.b;
    res.z += operand.a;

    res
  }
}
