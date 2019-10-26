use std::collections::HashMap;
use std::fmt;
use std::ops;

pub type Eps = HashMap<&'static str, f64>;

#[derive(Clone)]
pub struct D(pub f64, pub Eps);

impl ops::Add for D {
  type Output = D;

  fn add(self, other: Self) -> D {
    D(self.0 + other.0, add_eps(&self.1, &other.1))
  }
}

impl ops::Mul for D {
  type Output = D;

  fn mul(self, other: Self) -> D {
    D(
      self.0 * other.0,
      add_eps(&mul_eps(&other.1, self.0), &mul_eps(&self.1, other.0)),
    )
  }
}

impl D {
  pub fn new(real: f64, eps: Eps) -> D {
    D(real, eps)
  }

  pub fn sin(&self) -> D {
    D(self.0.sin(), mul_eps(&self.1, self.0.cos()))
  }

  pub fn cos(&self) -> D {
    D(self.0.cos(), mul_eps(&self.1, self.0.sin()))
  }
}

impl fmt::Display for D {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {:?})", self.0, self.1)
  }
}

fn add_eps(lhs: &Eps, rhs: &Eps) -> Eps {
  let mut add_map = HashMap::new();
  for key in lhs.keys().chain(rhs.keys()) {
    add_map.insert(
      *key,
      lhs.get(key).unwrap_or(&0.0) + rhs.get(key).unwrap_or(&0.0),
    );
  }
  add_map
}

fn mul_eps(lhs: &Eps, d: f64) -> Eps {
  lhs.iter().map(|(k, v)| (*k, v * d)).collect()
}
