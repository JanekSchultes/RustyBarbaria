use std::ops::Deref;
use crate::engine::voxel_render::MeshData;
use crate::game::chunk::block::Block;
use crate::game::GameState;

pub mod block;

const CHUNK_SIZE: usize = 16;


// Chunk saves index into "global" mesh memory. this index becomes shifted with mesh insertion/deletion
// the local indices are saved in visible_meshes
struct Chunk {
    blocks: [Block; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
    global_chunk_index: usize,
}

impl Chunk {
    fn get_block_index(&self, rel_pos_x: usize, rel_pos_y: usize, rel_pos_z: usize) -> usize {
        rel_pos_z * CHUNK_SIZE * CHUNK_SIZE + rel_pos_y * CHUNK_SIZE + rel_pos_z
    }

    fn set_block(&mut self, mut state: GameState, index: usize, block: Block) {
        self.blocks[index] = block;
        for f in self.blocks[index].visible_faces.iter() {
            state.graphics_memory.add_mesh(f.clone().mesh);
        }
    }

    fn remove_block(&mut self, mut state: GameState, index: usize) {
        self.blocks[index].is_air = true;
        for f in self.blocks[index].visible_faces.iter() {

        }
    }
}