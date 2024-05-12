#![allow(dead_code)]
/// Config loading
/// TODO. This is DUMMY!
use crate::prelude::*;
//use serde::{Deserialize, Serialize};

#[derive(/*Serialize, Deserialize, */ Resource)]
pub struct GameSettings {
    lang: LanguageSettings,
}

#[derive(/*Serialize, Deserialize, */ Resource)]
pub struct LanguageSettings {
    lang: String,
}

pub fn load() -> GameSettings {
    // TODO load
    GameSettings {
        lang: LanguageSettings {
            lang: "ru_RU".to_string(),
        },
    }
}
