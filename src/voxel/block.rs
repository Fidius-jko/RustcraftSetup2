use bevy::utils::HashMap;

use crate::prelude::*;

pub struct BlockType {
    pub sides: BlockSides,
}

#[derive(Clone)]
pub struct BlockSides {
    pub left: BlockSideInfo,
    pub right: BlockSideInfo,
    pub top: BlockSideInfo,
    pub bottom: BlockSideInfo,
    pub forward: BlockSideInfo,
    pub back: BlockSideInfo,
}

#[derive(Clone)]
pub struct BlockSideInfo(pub Mesh);

#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub struct BlockId(pub u32);

#[derive(Default)]
pub struct BlockStorage {
    storage: HashMap<BlockId, BlockType>,
}
impl BlockStorage {
    pub fn add(&mut self, id: BlockId, t: BlockType) {
        self.storage.insert(id, t);
    }
    pub fn get(&self, id: BlockId) -> Option<&BlockType> {
        self.storage.get(&id)
    }
}

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
