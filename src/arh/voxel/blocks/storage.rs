use bevy::utils::HashMap;

use crate::{
    prelude::*,
    resources::blocks::{BlockTypesAsset, UnMeshedBlockType},
    voxel::blocks::image_storage::BlockImageStorage,
};

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

#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug, Default)]
pub struct BlockId(pub u32);

use self::{resources::blocks::BlockFaces, voxel::render::mesh::meshing_block_type};

#[derive(Resource)]
pub struct BlockStorage {
    un_meshed_storage: HashMap<BlockId, UnMeshedBlockType>,
    storage: HashMap<BlockId, BlockType>,
    name_binds: HashMap<String, BlockId>,
    last_id: BlockId,
    pub(crate) imgs: BlockImageStorage,
}
impl BlockStorage {
    pub fn new(
        asset_server: Res<AssetServer>,
        mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let storage = HashMap::new();
        let name_binds = HashMap::new();

        let mut self_ = Self {
            storage,
            un_meshed_storage: HashMap::new(),
            name_binds,
            last_id: BlockId(1),
            imgs: BlockImageStorage::empty(asset_server, &mut layouts),
        };
        self_.add(
            "unknown".to_string(),
            UnMeshedBlockType::Block {
                faces: BlockFaces {
                    top: "unknown".to_string(),
                    bottom: "unknown".to_string(),
                    left: "unknown".to_string(),
                    right: "unknown".to_string(),
                    forward: "unknown".to_string(),
                    backward: "unknown".to_string(),
                },
            },
            &layouts.into(),
        );

        self_
    }
    pub fn add(
        &mut self,
        name: String,
        type_: UnMeshedBlockType,
        layouts: &Res<Assets<TextureAtlasLayout>>,
    ) {
        self.add_block_type(name.clone(), meshing_block_type(self, &type_, &layouts));
        self.un_meshed_storage
            .insert(self.get_id_by_name(name).unwrap().clone(), type_);
    }

    fn add_block_type(&mut self, name: String, t: BlockType) {
        self.storage.insert(self.last_id, t);
        self.name_binds.insert(name, self.last_id);
        self.last_id.0 += 1;
    }

    pub fn add_block_types(
        &mut self,
        asset: &BlockTypesAsset,
        mut images: ResMut<Assets<Image>>,
        mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let mut builder = TextureAtlasBuilder::default();
        let mut binds = HashMap::<String, usize>::new();
        let mut i = 0;
        for (name, img) in &asset.images {
            builder.add_texture(None, img);
            binds.insert(name.clone(), i);
            i += 1;
        }
        let (layout, texture) = builder.finish().unwrap();
        let new_img_storage = BlockImageStorage {
            texture_size: texture.size(),
            texture: images.add(texture),
            layout: layouts.add(layout),
            binds,
        };
        self.imgs.merge(new_img_storage, images, &mut layouts);
        let layout_unmut = Res::from(layouts);
        self.update_meshes(&layout_unmut);
        for (name, type_) in asset.types.iter() {
            self.add(name.clone(), type_.clone(), &layout_unmut);
        }
    }

    pub fn update_meshes(&mut self, layouts: &Res<Assets<TextureAtlasLayout>>) {
        for (id, type_) in self.un_meshed_storage.iter() {
            let type_ = meshing_block_type(self, type_, layouts);
            *self.storage.get_mut(id).unwrap() = type_;
        }
    }

    pub fn get_id_by_name(&self, name: String) -> Option<&BlockId> {
        self.name_binds.get(&name)
    }
    pub fn get(&self, id: BlockId) -> Option<&BlockType> {
        self.storage.get(&id)
    }
    pub fn get_or_default(&self, id: BlockId) -> &BlockType {
        match self.get(id) {
            Some(t) => t,
            None => self
                .get(self.get_id_by_name("unknown".to_string()).unwrap().clone())
                .unwrap(),
        }
    }
}
