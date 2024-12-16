use bevy::prelude::*;

// use std::sync::Arc;
// use std::sync::Mutex;

pub mod user_interface;

#[derive(Asset, Component, TypePath)]
pub struct EasyVecUiCamera;

#[derive(Component)]
pub struct EasyVecUiNode;

#[derive(Resource)]
pub struct EasyVecUiFonts {
    pub fonts: Vec<TextStyle>,
}

impl EasyVecUiFonts {
    pub fn new() -> Self {
        let fonts: Vec<TextStyle> = Vec::new();
        EasyVecUiFonts {
            fonts,
        }
    }
}

#[derive(Component)]
pub struct EasyVecUiStatusText;

#[derive(Component)]
pub struct EasyVecUiTitleText;

#[derive(Resource)]
pub struct EasyVecUiUpdateTimer(pub Timer);