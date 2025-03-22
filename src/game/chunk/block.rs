use crate::engine::voxel_render::MeshData;

pub struct Block {
    pub visible_faces: Vec<VisibleBlockFace>,
    pub is_air: bool,
}

#[derive(Clone)]
pub struct VisibleBlockFace {
    pub index: usize
}