use crate::voxel::blocks::BlockId;

#[derive(Clone, Copy, Debug)]
pub enum Block {
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
