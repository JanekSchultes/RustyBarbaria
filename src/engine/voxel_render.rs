use bitflags::bitflags;
use vulkano::buffer::BufferContents;

const RENDER_DISTANCE: usize = 64;

struct VoxelRender {

}

pub struct GraphicsMemory {
    mesh_data_memory: Vec<MeshData>,
    // Outside references are kept as indices become updated
    mesh_data_indices: Vec<Option<usize>>,
    cs_info_memory: Vec<u32>,
}

// First the bottom layer, then the top, left, right, front and back layers are transmitted
// -> efficient greedy meshing
// Chunks need to save data pointers to their faces(maybe two separate structures?)
// chunks in data layout are 16*16*16, so iteration needs to be performed over z as well
// 64 bit per face
// if distance > 5 chunks, only save faces not facing away from player (-25%)
// if more than 3 chunks to the left/right, consider right/left faces to not be visible (-12.5%)
// Give player position to CS to rearrange memory
#[derive(Copy, Clone)]
pub struct MeshData {
    flags: FaceFlags,
    pos_x: u8,
    pos_y: u8,
    pos_z: u8,
    texture_id: u16,
    length_x: u8,
    length_y: u8,
    chunk_pos_x: u16,
    chunk_pos_y: u16,
}

bitflags! {
    #[derive(Copy, Clone)]
    pub struct FaceFlags: u8 {
        const TOP = 0b0000_0001;
        const BOTTOM = 0b0000_0010;
        const LEFT = 0b0000_0100;
        const RIGHT = 0b0000_1000;
        const FRONT = 0b0001_0000;
        const BACK = 0b0010_0000;
        const CHUNK_NEXT = 0b0010_0000;
    }
}

fn render(mem: GraphicsMemory) {

}

impl GraphicsMemory {
    // chunk_index is index into mesh_data_chunk_indices
    pub fn add_mesh(&mut self, data: MeshData) -> usize {
        self.mesh_data_memory.push(data);
        let index = self.mesh_data_memory.len() - 1;
        match self.mesh_data_indices.iter().position(|&x| x == None) {
            None => {
                // Found no empty element in the index list, append one and return its index
                self.mesh_data_indices.push(Some(index));
                self.mesh_data_indices.len() - 1
            }
            Some(i) => {
                // Found an empty element in the index list, replace it and return its index
                self.mesh_data_indices[i] = Some(index);
                i
            }
        }
    }

    pub fn delete_mesh(&mut self, mesh_index: usize) {
        let memory_index = match self.mesh_data_indices[mesh_index] {
            Some(i) => {i}
            None => {println!("Tried to delete an already deleted mesh."); return;}
        };
        let last_index = self.mesh_data_memory.len() - 1;

        // Restructure the memory, move last element to deleted position
        self.mesh_data_memory.swap(memory_index, last_index);
        self.mesh_data_memory.remove(last_index);
        self.mesh_data_memory.shrink_to_fit();

        // Invalidate the mesh index and update the moved one
        self.mesh_data_indices[mesh_index] = None;
        let index_pos = self.mesh_data_indices.iter_mut()
            .position(|x| x.is_some() && x.unwrap() == last_index);
        if let Some(i) = index_pos {
            self.mesh_data_indices[i] = Some(memory_index);
        }
    }
}