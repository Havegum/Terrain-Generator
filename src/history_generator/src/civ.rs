#[derive(PartialEq)]
pub struct Color(u8, u8, u8);

#[derive(PartialEq)]
pub struct Civilization {
    name: String,
    color: Color,
    resources: f64,
    // resources: f64,
}
