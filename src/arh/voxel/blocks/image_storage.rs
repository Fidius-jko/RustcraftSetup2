use bevy::utils::HashMap;

use crate::prelude::*;

pub struct BlockImageStorage {
    pub texture: Handle<Image>,
    pub texture_size: UVec2,
    pub layout: Handle<TextureAtlasLayout>,
    pub binds: HashMap<String, usize>,
}

impl BlockImageStorage {
    pub fn get_texture_rect(&self, name: &str, layouts: &Res<Assets<TextureAtlasLayout>>) -> Rect {
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
    pub fn merge(
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
    pub fn empty(
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
