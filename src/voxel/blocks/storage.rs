use bevy::utils::HashMap;

use crate::{
    prelude::*,
    resources::blocks::{BlockTypesAsset, UnMeshedBlockType},
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

pub struct BlockImageStorage {
    pub texture: Handle<Image>,
    pub texture_size: UVec2,
    pub layout: Handle<TextureAtlasLayout>,
    pub binds: HashMap<String, usize>,
}

impl BlockImageStorage {
    fn get_texture_rect(&self, name: &str, layouts: &Res<Assets<TextureAtlasLayout>>) -> Rect {
        let layout = layouts.get(self.layout.clone()).unwrap();
        let ind = match self.binds.get(name) {
            Some(i) => i,
            None => {
                return layout
                    .textures
                    .get(0 /*Default texture*/)
                    .clone()
                    .expect("Not found default texture")
                    .clone();
            }
        };
        match layout.textures.get(ind.clone()).clone() {
            Some(rect) => rect.clone(),
            None => {
                return layout
                    .textures
                    .get(0 /*Default texture*/)
                    .clone()
                    .expect("Not found default texture")
                    .clone();
            }
        }
    }
    fn merge(
        &mut self,
        another: Self,
        mut images: ResMut<Assets<Image>>,
        layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let mut builder = TextureAtlasBuilder::default();

        builder.add_texture(
            Some(self.texture.clone().into()),
            images.get(self.texture.clone()).unwrap(),
        );
        builder.add_texture(
            Some(another.texture.clone().into()),
            images.get(another.texture.clone()).unwrap(),
        );

        let (mut layout, texture) = builder.finish().unwrap();
        let mut self_layout = layouts.get(self.layout.clone()).unwrap().clone();
        let mut another_layout = layouts.get(another.layout.clone()).unwrap().clone();
        for rect in self_layout.textures.iter_mut() {
            let rect2 = layout.textures.get(0).unwrap();
            rect.min.x += rect2.min.x;
            rect.min.y += rect2.min.y;
            rect.max.x += rect2.min.x;
            rect.max.y += rect2.min.y;
        }
        for rect in another_layout.textures.iter_mut() {
            let rect2 = layout.textures.get(1).unwrap();
            rect.min.x += rect2.min.x;
            rect.min.y += rect2.min.y;
            rect.max.x += rect2.min.x;
            rect.max.y += rect2.min.y;
        }
        for (name, ind) in another.binds.iter() {
            self.binds
                .insert(name.clone(), ind + self_layout.textures.len());
        }

        layout.textures = Vec::from(self_layout.textures);
        layout.textures.append(&mut another_layout.textures);

        self.texture_size = texture.size();
        self.texture = images.add(texture);
        *layouts.get_mut(self.layout.clone()).unwrap() = layout;
        //*images.get_mut(self.texture.clone()).unwrap() = texture;
    }
    fn empty(
        asset_server: Res<AssetServer>,
        layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let mut binds = HashMap::new();
        binds.insert("unknown".to_string(), 0);
        let mut layout = TextureAtlasLayout::new_empty(Vec2::new(
            resources::embedded::UNKNOWN_TEXTURE_SIZE.0 as f32,
            resources::embedded::UNKNOWN_TEXTURE_SIZE.1 as f32,
        ));
        layout.add_texture(Rect {
            min: Vec2::new(0., 0.),
            max: Vec2::new(16., 16.),
        });
        Self {
            texture: asset_server
                .load("embedded://".to_string() + resources::embedded::UNKNOWN_TEXTURE_PATH),
            texture_size: UVec2::from(resources::embedded::UNKNOWN_TEXTURE_SIZE),
            layout: layouts.add(layout),
            binds,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug, Default)]
pub struct BlockId(pub u32);

use utils::mesh::WithUvCoords;

use utils::mesh::square_mesh;

use utils::mesh::SquareType3D;

use self::resources::blocks::BlockFaces;

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
        self.add_block_type(name.clone(), self.meshing_block_type(&type_, &layouts));
        self.un_meshed_storage
            .insert(self.get_id_by_name(name).unwrap().clone(), type_);
    }

    fn meshing_block_type(
        &self,
        type_: &UnMeshedBlockType,
        layouts: &Res<Assets<TextureAtlasLayout>>,
    ) -> BlockType {
        match type_ {
            UnMeshedBlockType::Block { faces } => {
                let left_rect = self.imgs.get_texture_rect(&faces.left.clone(), layouts);
                let right_rect = self.imgs.get_texture_rect(&faces.right.clone(), layouts);
                let top_rect = self.imgs.get_texture_rect(&faces.top.clone(), layouts);
                let bottom_rect = self.imgs.get_texture_rect(&faces.bottom.clone(), layouts);
                let forward_rect = self.imgs.get_texture_rect(&faces.forward.clone(), layouts);
                let back_rect = self.imgs.get_texture_rect(&faces.backward.clone(), layouts);

                BlockType {
                    sides: BlockSides {
                        left: BlockSideInfo(
                            square_mesh(1., 1., SquareType3D::Right(-1.))
                                .with_uv_coords(self.imgs.texture_size, left_rect.clone()),
                        ),
                        right: BlockSideInfo(
                            square_mesh(1., 1., SquareType3D::Right(1.))
                                .with_uv_coords(self.imgs.texture_size, right_rect.clone()),
                        ),
                        top: BlockSideInfo(
                            square_mesh(1., 1., SquareType3D::Top(1.))
                                .with_uv_coords(self.imgs.texture_size, top_rect.clone()),
                        ),
                        bottom: BlockSideInfo(
                            square_mesh(1., 1., SquareType3D::Top(-1.))
                                .with_uv_coords(self.imgs.texture_size, bottom_rect.clone()),
                        ),
                        forward: BlockSideInfo(
                            square_mesh(1., 1., SquareType3D::Back(-1.))
                                .with_uv_coords(self.imgs.texture_size, forward_rect.clone()),
                        ),
                        back: BlockSideInfo(
                            square_mesh(1., 1., SquareType3D::Back(1.))
                                .with_uv_coords(self.imgs.texture_size, back_rect.clone()),
                        ),
                    },
                }
            }
        }
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
            let type_ = self.meshing_block_type(type_, layouts);
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
