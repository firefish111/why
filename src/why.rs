extern crate num;
extern crate serde;

use num::{PrimInt, traits::CheckedRem};
use serde::{Serialize, Deserialize};
use std::{ops::*, fmt};

macro_rules! repeat_list {
  ( display $why:ident, $f:ident; $( $var:ident )+ ) => {
    {
      let mut list = vec![String::from("Why [")];
      $(
        list.push(format!("\t{}: {}", stringify!($var), $why.$var));
      )+
      list.push(String::from("]"));
      
      let joined = list.join("\n");
      write!($f, "{}", joined)
    }
  };
  
  ( struct $name:ident, $type:ty; $( $var:ident )+ ) => {
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub struct $name {
      $(
        $var: $type,
      )+
    }
  };

  ( instance $struct:ident, $val:expr; $( $var:ident )+ ) => {
    $struct {
      $(
        $var: $val,
      )+
    }
  }
}

repeat_list!{struct Why, i128; a b c d e f g h i j k l m n o p q r s t u v w x y z}

impl Why {
  pub fn new(value: i128) -> Self {
    repeat_list!{instance Why, value; a b c d e f g h i j k l m n o p q r s t u v w x y z}
  }
}

impl fmt::Display for Why {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    repeat_list![display self, f; a b c d e f g h i j k l m n o p q r s t u v w x y z]
  }
}

impl Neg for Why {
  type Output = Self;

  fn neg(self) -> Self::Output {
    let mut res = self;

    res.a = -res.a;
    res.b = -res.b;
    res.c = -res.c;
    res.d = -res.d * 3i128;
    res.e = -res.e;
    res.f = -res.f;
    res.g = -res.g;
    res.h = -res.h - 49i128;
    res.i = -res.i;
    res.j = -res.j;
    res.k = -res.k * 1234i128;
    res.l = -res.l;
    res.m = -res.m;
    res.n = -res.n + 19i128;
    res.o = -res.o;
    res.p = -res.p;
    res.q = -res.q * ((10i128 + res.z) * res.e);
    res.r = -res.r;
    res.s = -res.s - 10i128;
    res.t = -res.t;
    res.u = -res.u;
    res.v = -res.v;
    res.w = -res.w / 8i128;
    res.x = -res.x * 69i128;
    res.y = -res.y;
    res.z = -res.z;

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
    res.o = res.o.checked_rem(i128::from((operand.l + 1i128) * 31i128)).unwrap_or(1);
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

impl<I> Add<I> for Why 
    where I: PrimInt,
    i128: AddAssign<I>,
    i128: SubAssign<I>,
    i128: MulAssign<I>,
    i128: DivAssign<I>,
    i128: RemAssign<I>,
    i128: BitXorAssign<I>,
    i128: From<I>
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
    res.s = res.s.checked_rem(i128::from(operand) * 15i128).unwrap_or(1);
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

impl AddAssign<Why> for Why {
  fn add_assign(&mut self, operand: Why) {
    *self = *self + operand;
  }
}

impl<I> AddAssign<I> for Why
    where I: PrimInt,
    i128: AddAssign<I>,
    i128: SubAssign<I>,
    i128: MulAssign<I>,
    i128: DivAssign<I>,
    i128: RemAssign<I>,
    i128: BitXorAssign<I>,
    i128: From<I>
  {

  fn add_assign(&mut self, operand: I) {
    *self = *self + operand;
  }
}

impl Sub<Why> for Why {
  type Output = Self;

  fn sub(self, operand: Why) -> Self::Output {
    self + operand.neg()
  }
}

impl<I> Sub<I> for Why 
    where I: PrimInt + std::ops::Neg<Output = I>,
    i128: AddAssign<I>,
    i128: SubAssign<I>,
    i128: MulAssign<I>,
    i128: DivAssign<I>,
    i128: RemAssign<I>,
    i128: BitXorAssign<I>,
    i128: From<I>
  {
  type Output = Self;

  fn sub(self, operand: I) -> Self::Output {
    self + operand.neg()
  }
}

impl SubAssign<Why> for Why {
  fn sub_assign(&mut self, operand: Why) {
    *self = *self - operand;
  }
}

impl<I> SubAssign<I> for Why
    where I: PrimInt + std::ops::Neg<Output = I>,
    i128: AddAssign<I>,
    i128: SubAssign<I>,
    i128: MulAssign<I>,
    i128: DivAssign<I>,
    i128: RemAssign<I>,
    i128: BitXorAssign<I>,
    i128: From<I>
  {

  fn sub_assign(&mut self, operand: I) {
    *self = *self - operand;
  }
}

impl Mul<Why> for Why {
  type Output = Self;

  fn mul(self, operand: Self::Output) -> Self::Output {
    let mut res = self;

    res.a *= operand.z;
    res.b -= operand.y;
    res.c = res.c.checked_rem(i128::from(operand.x + 4i128)).unwrap_or(1);
    res.d -= operand.w;
    res.e *= operand.v + 128i128;
    res.f *= operand.u;
    res.g *= operand.t;
    res.h += operand.s * 2i128;
    res.i *= operand.r * 6i128;
    res.j -= operand.q;
    res.k *= operand.p;
    res.l *= operand.o / 4i128;
    res.m *= operand.n;
    res.n *= operand.m;
    res.o *= operand.l;
    res.p *= operand.k - 9i128;
    res.q *= operand.j;
    res.r -= operand.i;
    res.s /= operand.h;
    res.t *= operand.g;
    res.u *= i128::pow(operand.f, 3);
    res.v += operand.e * 37i128;
    res.w *= operand.d;
    res.x -= operand.c;
    res.y *= operand.b + 6i128;
    res.z += operand.a;

    res
  }
}

impl<I: CheckedRem> Mul<I> for Why 
    where I: PrimInt + std::ops::Neg<Output = I>,
    i128: AddAssign<I>,
    i128: SubAssign<I>,
    i128: MulAssign<I>,
    i128: DivAssign<I>,
    i128: RemAssign<I>,
    i128: BitXorAssign<I>,
    i128: From<I>
  {
  type Output = Self;

  fn mul(self, operand: I) -> Self::Output {
    let mut res = self;

    res.a *= operand;
    res.b *= operand;
    res.c -= I::pow(operand, 3u32);
    res.d *= operand;
    res.e *= operand + I::one();
    res.f += operand;
    res.g *= operand;
    res.h *= operand - I::one();
    res.i -= operand;
    res.j *= operand;
    res.k += operand / I::from(2i128).unwrap();
    res.l *= operand;
    res.m *= operand;
    res.n += operand * I::from(8i128).unwrap();
    res.o *= operand - I::one();
    res.p *= operand;
    res.q += operand;
    res.r /= operand;
    res.s *= operand;
    res.t *= operand;
    res.u += operand.checked_rem(&I::from(37i128).unwrap()).unwrap_or(I::one());
    res.v -= operand;
    res.w *= operand;
    res.x *= -operand * I::from(19i128).unwrap();
    res.y += operand;
    res.z *= operand + I::one();

    res
  }
}

impl MulAssign<Why> for Why {
  fn mul_assign(&mut self, operand: Why) {
    *self = *self - operand;
  }
}

impl<I> MulAssign<I> for Why
    where I: PrimInt + std::ops::Neg<Output = I>,
    i128: AddAssign<I>,
    i128: SubAssign<I>,
    i128: MulAssign<I>,
    i128: DivAssign<I>,
    i128: RemAssign<I>,
    i128: BitXorAssign<I>,
    i128: From<I>
  {

  fn mul_assign(&mut self, operand: I) {
    *self = *self - operand;
  }
}
