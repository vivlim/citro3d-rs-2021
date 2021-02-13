#[repr(C)]
#[derive(Debug)]
pub struct C3D_Vertex {
    pub position: [f32; 3],
    pub texcoord: [f32; 2],
    pub normal: [f32; 3],
}
