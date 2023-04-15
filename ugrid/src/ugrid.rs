use bevy::{
    prelude::*,
    reflect::TypeUuid,
};

use grid::{ugrid::UGrid};


#[derive(Resource, Deref, DerefMut, TypeUuid)]
#[uuid = "e458f087-eee5-48ee-bc11-f59f8826d4ae"]
pub struct Grid(pub UGrid);

impl Default for Grid {
    fn default() -> Self {
        
        Self(UGrid::default())
    }
}