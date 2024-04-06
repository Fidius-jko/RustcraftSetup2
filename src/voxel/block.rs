use bevy::utils::HashMap;

use crate::prelude::*;

use super::{square_mesh, SquareType3D};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockStorage>()
            .add_systems(OnEnter(GameState::Load), load_block_types);
    }
}

fn load_block_types(mut types: ResMut<BlockStorage>) {
    types.add(
        "test".to_string(),
        BlockType {
            sides: BlockSides {
                left: BlockSideInfo(square_mesh(1., 1., SquareType3D::Right(-1.))),
                right: BlockSideInfo(square_mesh(1., 1., SquareType3D::Right(1.))),
                top: BlockSideInfo(square_mesh(1., 1., SquareType3D::Top(1.))),
                bottom: BlockSideInfo(square_mesh(1., 1., SquareType3D::Top(-1.))),
                forward: BlockSideInfo(square_mesh(1., 1., SquareType3D::Back(-1.))),
                back: BlockSideInfo(square_mesh(1., 1., SquareType3D::Back(1.))),
            },
        },
    );
}

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

impl Default for BlockId {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Default, Resource)]
pub struct BlockStorage {
    storage: HashMap<BlockId, BlockType>,
    name_binds: HashMap<String, BlockId>,
    last_id: BlockId,
}
impl BlockStorage {
    pub fn add(&mut self, name: String, t: BlockType) {
        self.storage.insert(self.last_id.clone(), t);
        self.name_binds.insert(name, self.last_id);
        self.last_id.0 += 1;
    }
    pub fn get_id_by_name(&self, name: String) -> Option<&BlockId> {
        self.name_binds.get(&name)
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
