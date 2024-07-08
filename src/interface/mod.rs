//mod config; TODO!
//mod locale; TODO!
mod constants;
mod debug;
mod player;
mod render;
mod resources;

use render::voxel::{blocks::storage::BlockStorage, chunk::RenderOfChunk};

use crate::{prelude::*, voxel::chunks::chunk::Chunk};

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            resources::ResourcesPlugin,
            render::RenderPlugin,
            player::PlayerPlugin,
        ))
        .add_systems(OnEnter(GameState::Play), test_chunk);
    }
}

fn test_chunk(mut commands: Commands, storage: Res<BlockStorage>) {
    let render = RenderOfChunk {
        is_generated_mesh: false,
        left_chunk: None,
        right_chunk: None,
        forward_chunk: None,
        backward_chunk: None,
    };
    let mut chk = Chunk::new_air(IVec2::splat(0));
    for x in 0..CHUNK_W {
        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                if y < 17 {
                    chk.set(
                        x,
                        y,
                        z,
                        crate::voxel::blocks::Block::Solid(
                            storage.get_id_by_name("grass".to_string()).unwrap().clone(),
                        ),
                    );
                }
            }
        }
    }
    commands.spawn(chk).insert(render);
}
