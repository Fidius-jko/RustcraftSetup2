#[derive(Clone, Copy, Debug, Default)]
pub enum Block {
    #[default]
    Air,
    Solid(BlockId),
}
impl Block {
    pub fn is_solid(&self) -> bool {
        match self {
            Self::Solid(_) => true,
            _ => false,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug, Default)]
pub struct BlockId(pub u32);
