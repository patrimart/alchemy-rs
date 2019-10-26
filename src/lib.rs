
pub mod forward;

pub fn go() {
    let d = forward::D::new(1.0, 1.0);

    format!("{}", d);
}
