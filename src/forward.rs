use std::fmt;

#[derive(Clone)]
pub struct D(pub f64, pub f64);

impl D {
  pub fn new(real: f64, eps: f64) -> D {
    D(real, eps)
  }

  pub fn plus(&self, other: &Self) -> D {
    D(self.0 + other.0, self.1 + other.1)
  }

  pub fn mult(&self, other: &Self) -> D {
    D(self.0 * other.0, (other.1 * self.0) + (self.1 * other.0))
  }

  pub fn div(&self, other: &Self) -> D {
    let z = self.0 / other.0;
    D(z, (self.1 - (z * other.1)) / other.0)
  }

  pub fn sin(&self) -> D {
    D(self.0.sin(), self.1 * self.0.cos())
  }

  pub fn cos(&self) -> D {
    D(self.0.cos(), (0.0 - self.1) * self.0.sin())
  }

  pub fn from_double(real: f64) -> D {
    D(real, 0.0)
  }
}

impl fmt::Display for D {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}
