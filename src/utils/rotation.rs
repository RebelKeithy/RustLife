#[derive(Default,Copy,Clone)]
pub enum Rotation {
    #[default]
    None,
    CW,
    CCW,
    Half
}