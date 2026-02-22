pub trait Stacks {
    fn stacks(&self) -> f32;
}

impl Stacks for u32 {
    fn stacks(&self) -> f32 {
        *self as f32 / 64.0
    }
}
