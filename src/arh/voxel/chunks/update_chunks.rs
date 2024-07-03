use bevy::ecs::query::QueryFilter;
use voxel::{
    blocks::{Block, BlockStorage},
    chunks::chunk::Chunk,
    consts::*,
};

use crate::prelude::*;

use super::{chunks::ChunksStorage, spawn_chunks};

pub struct UpdateChunksPlugin;

impl Plugin for UpdateChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load.chain().run_if(in_state(GameState::Play)))
            .add_systems(
                OnEnter(GameState::Play),
                create_load_pipeline.after(spawn_chunks),
            )
            .insert_resource(LoadChunksPipeLine {
                pipeline: Vec::new(),
                speed: 1,
            })
            .init_resource::<LoadedChunksInPipeline>();
    }
}

#[derive(Resource, Default)]
pub struct LoadedChunksInPipeline {
    chunks: Vec<Vec<bool>>,
}

fn create_load_pipeline(
    mut loaded: ResMut<LoadedChunksInPipeline>,
    mut pipeline: ResMut<LoadChunksPipeLine>,
    update_chunks: Res<UpdateChunks>,
) {
    if loaded.chunks.len() != update_chunks.size.x as usize {
        loaded.chunks.clear();
        for _ in 0..update_chunks.size.x {
            let mut y_vec = Vec::new();
            for _ in 0..update_chunks.size.y {
                y_vec.push(false);
            }
            loaded.chunks.push(y_vec);
        }
    }
    if update_chunks.size.x == 1 {
        let temp = loaded.chunks.get_mut(0).unwrap().get_mut(0).unwrap();
        if *temp != true {
            pipeline.pipeline.push(IVec2::new(0, 0));
        }
        *temp = true;
        return;
    }
    for dis in 0..update_chunks.size.x {
        for x2 in 0..update_chunks.size.x {
            for y2 in 0..update_chunks.size.y {
                let x = x2 as i32 - (update_chunks.size.x / 2 + 1) as i32;
                let y = y2 as i32 - (update_chunks.size.y / 2 + 1) as i32;
                if (x.abs() == dis as i32 && y.abs() <= dis as i32)
                    || (y.abs() == dis as i32 && x.abs() <= dis as i32)
                {
                    let temp = loaded
                        .chunks
                        .get_mut(x2 as usize)
                        .unwrap()
                        .get_mut(y2 as usize)
                        .unwrap();
                    if *temp != true {
                        pipeline.pipeline.push(IVec2::new(x2 as i32, y2 as i32));
                    }
                    *temp = true;
                } else {
                    let temp = loaded
                        .chunks
                        .get_mut(x2 as usize)
                        .unwrap()
                        .get_mut(y2 as usize)
                        .unwrap();
                    // If we doesn't deloading chunks
                    //if *temp != false {
                    //}
                    *temp = false;
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct LoadChunksPipeLine {
    pipeline: Vec<IVec2>,
    speed: usize,
}

fn load(
    mut pipeline: ResMut<LoadChunksPipeLine>,
    update_chunks: Res<UpdateChunks>,
    mut chunks: Query<&mut Chunk>,
    mut chunk_storage: NonSendMut<ChunksStorage>,
    block_storage: Res<BlockStorage>,
) {
    if pipeline.pipeline.len() == 1 {
        let i = 0;
        match pipeline.pipeline.get_mut(i) {
            Some(pos) => {
                let mut chunk = chunks
                    .get_mut(update_chunks.get_chunk_entity(pos.x, pos.y))
                    .unwrap();

                chunk.data = chunk_storage.get(pos.clone(), &block_storage);
                chunk.set_to_not_generated_mesh();
                let left_chunk = chunk.left_chunk.clone();
                let right_chunk = chunk.right_chunk.clone();
                let forward_chunk = chunk.forward_chunk.clone();
                let backward_chunk = chunk.backward_chunk.clone();
                drop(chunk);
                if let Some(en) = left_chunk {
                    chunks.get_mut(en).unwrap().set_to_not_generated_mesh();
                }
                if let Some(en) = right_chunk {
                    chunks.get_mut(en).unwrap().set_to_not_generated_mesh();
                }
                if let Some(en) = forward_chunk {
                    chunks.get_mut(en).unwrap().set_to_not_generated_mesh();
                }
                if let Some(en) = backward_chunk {
                    chunks.get_mut(en).unwrap().set_to_not_generated_mesh();
                }
            }
            None => {
                panic!("Something went wrong!(LoadChunkPipeLine)");
            }
        }
        pipeline.pipeline.clear();
        return;
    }
    let mut last_i = 0;
    for i in 0..pipeline.pipeline.len() {
        last_i = i;
        if i > pipeline.speed {
            let vec2 = pipeline.pipeline.split_off(i);
            pipeline.pipeline = vec2;
            break;
        }
        match pipeline.pipeline.get_mut(i) {
            Some(pos) => {
                let mut chunk = chunks
                    .get_mut(update_chunks.get_chunk_entity(pos.x, pos.y))
                    .unwrap();

                chunk.data = chunk_storage.get(pos.clone(), &block_storage);
                chunk.set_to_not_generated_mesh();
                let left_chunk = chunk.left_chunk.clone();
                let right_chunk = chunk.right_chunk.clone();
                let forward_chunk = chunk.forward_chunk.clone();
                let backward_chunk = chunk.backward_chunk.clone();
                drop(chunk);
                if let Some(en) = left_chunk {
                    chunks.get_mut(en).unwrap().set_to_not_generated_mesh();
                }
                if let Some(en) = right_chunk {
                    chunks.get_mut(en).unwrap().set_to_not_generated_mesh();
                }
                if let Some(en) = forward_chunk {
                    chunks.get_mut(en).unwrap().set_to_not_generated_mesh();
                }
                if let Some(en) = backward_chunk {
                    chunks.get_mut(en).unwrap().set_to_not_generated_mesh();
                }
            }
            None => {
                panic!("Something went wrong!(LoadChunkPipeLine)");
            }
        }
    }
    if last_i <= pipeline.speed {
        pipeline.pipeline.clear()
    }
}

#[derive(Resource)]
pub struct UpdateChunks {
    chunks: Vec<Entity>,
    size: UVec2,
}
impl UpdateChunks {
    pub fn get<F: QueryFilter>(
        &self,
        x: i32,
        y: i32,
        z: i32,
        chunks: &Query<&Chunk, F>,
    ) -> Option<Block> {
        let mut chunk_x = x / CHUNK_W as i32;
        let mut chunk_z = z / CHUNK_D as i32;
        if x < 0 {
            chunk_x -= 1
        }
        if z < 0 {
            chunk_z -= 1
        }
        if chunk_x < 0
            || chunk_z < 0
            || y < 0
            || chunk_x < self.size.x as i32
            || chunk_z < self.size.y as i32
            || y < CHUNK_H as i32
        {
            return None;
        }
        let chunk = chunks
            .get(
                self.chunks
                    .get((chunk_x + chunk_z * self.size.x as i32) as usize)
                    .unwrap()
                    .clone(),
            )
            .unwrap();
        Some(chunk.get_from_only_my(
            (x.abs() as usize + CHUNK_W / 2) % CHUNK_W,
            y.abs() as usize,
            (z.abs() as usize + CHUNK_D / 2) % CHUNK_D,
        ))
    }
    pub fn set<F: QueryFilter>(
        &self,
        x: i32,
        y: i32,
        z: i32,
        chunks: &mut Query<&mut Chunk, F>,
        block: Block,
    ) {
        let mut chunk_x = x / CHUNK_W as i32;
        let mut chunk_z = z / CHUNK_D as i32;
        /*
        if x < 0 {
            chunk_x -= 1
        }
        if z < 0 {
            chunk_z -= 1
        }*/
        if chunk_x < 0
            || chunk_z < 0
            || y < 0
            || chunk_x < self.size.x as i32
            || chunk_z < self.size.y as i32
            || y < CHUNK_H as i32
        {
            warn!("set: out of bounds");
            return;
        }
        let mut chunk = chunks
            .get_mut(
                self.chunks
                    .get((chunk_x + chunk_z * self.size.x as i32) as usize)
                    .unwrap()
                    .clone(),
            )
            .unwrap();
        let left_chunk = chunk.left_chunk.clone();
        let right_chunk = chunk.right_chunk.clone();
        let forward_chunk = chunk.forward_chunk.clone();
        let backward_chunk = chunk.backward_chunk.clone();
        chunk.set(
            (x.abs() as usize + CHUNK_W / 2) % CHUNK_W,
            y.abs() as usize,
            (z.abs() as usize + CHUNK_D / 2) % CHUNK_D,
            block,
        );
        if let Some(chk_ref) = left_chunk {
            let mut chunk = chunks.get_mut(chk_ref).unwrap();
            chunk.set_to_not_generated_mesh();
        }
        if let Some(chk_ref) = right_chunk {
            let mut chunk = chunks.get_mut(chk_ref).unwrap();
            chunk.set_to_not_generated_mesh();
        }
        if let Some(chk_ref) = forward_chunk {
            let mut chunk = chunks.get_mut(chk_ref).unwrap();
            chunk.set_to_not_generated_mesh();
        }
        if let Some(chk_ref) = backward_chunk {
            let mut chunk = chunks.get_mut(chk_ref).unwrap();
            chunk.set_to_not_generated_mesh();
        }
    }
    pub fn get_chunk<'a, F: QueryFilter>(
        &self,
        x: i32,
        z: i32,
        chunks: &'a Query<&Chunk, F>,
    ) -> Option<&'a Chunk> {
        match chunks.get(
            self.chunks
                .get((x + z * self.size.x as i32) as usize)
                .unwrap()
                .clone(),
        ) {
            Ok(chunk) => Some(chunk),
            Err(_) => None,
        }
    }
    pub fn get_chunk_entity(&self, x: i32, z: i32) -> Entity {
        self.chunks
            .get((x + z * self.size.x as i32) as usize)
            .unwrap()
            .clone()
    }

    pub fn new_air_with_size(size: UVec2, commands: &mut Commands) -> Self {
        let mut chunks = Vec::new();
        let mut real_chunks = Vec::new();
        commands
            .spawn(TransformBundle {
                local: Transform::from_translation(Vec3::new(
                    0., //-((size.x / 2 * CHUNK_W as u32) as f32 * VOXEL_SIZE)
                    //  - CHUNK_W as f32 / 2. * VOXEL_SIZE,
                    VOXEL_SIZE * 15.,
                    0., //-((size.y / 2 * CHUNK_D as u32) as f32 * VOXEL_SIZE)
                        //- CHUNK_D as f32 / 2. * VOXEL_SIZE,
                )),
                ..Default::default()
            })
            .insert(VisibilityBundle::default())
            .with_children(|child_cmd| {
                for z1 in 0..size.y {
                    for x1 in 0..size.x {
                        let mut chunk = Chunk::new(|| {
                            let chunk = Chunk::air();
                            chunk
                        });
                        chunk.with_translation(Vec3::new(
                            (x1 * CHUNK_W as u32) as f32 * VOXEL_SIZE,
                            0.,
                            (z1 * CHUNK_D as u32) as f32 * VOXEL_SIZE,
                        ));
                        chunks.push(child_cmd.spawn(()).id());
                        real_chunks.push(chunk);
                    }
                }
            })
            .insert(Name::new("Chunks"));
        let mut i = 0;
        for mut real_chunk in real_chunks {
            let x = i % size.x;
            let z = i / size.x;
            real_chunk.backward_chunk = get_chunk_entity(x as i32, z as i32 - 1, size, &chunks);
            real_chunk.forward_chunk = get_chunk_entity(x as i32, z as i32 + 1, size, &chunks);
            real_chunk.left_chunk = get_chunk_entity(x as i32 + 1, z as i32, size, &chunks);
            real_chunk.right_chunk = get_chunk_entity(x as i32 - 1, z as i32, size, &chunks);
            commands
                .entity(chunks.get((x + z * size.x) as usize).unwrap().clone())
                .insert(real_chunk);
            i += 1;
        }
        Self { chunks, size }
    }
}

fn get_chunk_entity(x: i32, z: i32, size: UVec2, chunks: &Vec<Entity>) -> Option<Entity> {
    let is_x0 = x >= 0;
    let is_z0 = z >= 0;
    let is_xw = x < size.x as i32;
    let is_zd = z < size.y as i32;

    if is_x0 && is_z0 && is_xw && is_zd {
        return match chunks.get((x + z * size.x as i32) as usize) {
            Some(en) => Some(en.clone()),
            None => None,
        };
    }
    return None;
}
