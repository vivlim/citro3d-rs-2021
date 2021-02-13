extern crate citro_3d_sys;
extern crate ctru_sys;
extern crate ctru;
extern crate cgmath;

#[macro_use]
extern crate build_const;

#[macro_use]
extern crate lazy_static;


use std::{default::Default, ffi::c_void, mem::size_of};
use std::ffi::CString;
use citro_3d_sys::*;
use ctru_sys::{DVLB_s, GPU_RB_DEPTH24_STENCIL8, gfxSet3D, shaderProgram_s};
use std::convert::TryInto;
use cgmath::{Deg, Rad, Matrix4, Vector4, Matrix, SquareMatrix};
#[macro_use]
pub mod macros;

mod inlines;
use crate::inlines::*;

static shbin_data: &[u8] = include_bytes_align_as!(u32, concat!(env!("OUT_DIR"), "/vshader.shbin"));
//static shbin2d_data: &[u8] = include_bytes_align_as!(u32, concat!(env!("OUT_DIR"), "/render2d.v.shbin.o"));


include!("vertex.rs");
build_const!("model_data");

const material: C3D_Material = C3D_Material {
    ambient: [ 0.1, 0.1, 0.1 ],
    diffuse: [ 0.4, 0.4, 0.4 ],
    specular0: [ 0.5, 0.5, 0.5 ],
    specular1: [ 0.0, 0.0, 0.0 ],
    emission: [ 0.0, 0.0, 0.0 ],
};


pub struct CitroLibContext {
    renderTarget: *mut C3D_RenderTarget,
    clrWhite: u32,
    clrGreen: u32,
    clrBlack: u32,
    clrClear: u32,
    textBuf: *mut C2D_TextBuf_s,
    text: C2D_Text,
    textString: CString,
    scene: Scene,
}

pub struct Scene {
    //shader_dvlb: *mut DVLB_s,
    program: ctru_sys::shaderProgram_s,
    uLoc_projection: i8,
    uLoc_modelView: i8,
    vbo_attrInfo: C3D_AttrInfo,
    vbo_bufInfo: C3D_BufInfo,
    projection: Matrix4<f32>,
}

fn GX_TRANSFER_FLIP_VERT(x: u32) -> u32 {
    ((x)<<0)
}  
fn GX_TRANSFER_OUT_TILED(x: u32) -> u32 {
    ((x)<<1)
}  
fn GX_TRANSFER_RAW_COPY(x: u32) -> u32 {
    ((x)<<3)
}   
fn GX_TRANSFER_IN_FORMAT(x: u32) -> u32 {
    ((x)<<8)
}  
fn GX_TRANSFER_OUT_FORMAT(x: u32) -> u32 {
    ((x)<<12)
} 
fn GX_TRANSFER_SCALING(x: u32) -> u32 {
    ((x)<<24)
}

lazy_static!{
    static ref uLoc_projection_name: CString = CString::new("projection").unwrap();
    static ref uLoc_modelView_name: CString = CString::new("modelView").unwrap();
}

fn debug_print(message: &str){
    unsafe {ctru_sys::svcOutputDebugString(CString::new(message.clone()).unwrap().as_ptr() as *const u8, message.chars().count() as i32);}
}

pub fn init() -> CitroLibContext {
    unsafe {
        C3D_Init(C3D_DEFAULT_CMDBUF_SIZE);

        //C2D_Init(C2D_DEFAULT_MAX_OBJECTS);
        //C2D_Prepare();
        //let renderTarget = C2D_CreateScreenTarget(GFX_TOP, GFX_LEFT);
        let mut depthFmt = C3D_DEPTHTYPE::default();
        depthFmt.__e = GPU_RB_DEPTH24_STENCIL8;
        let renderTarget = C3D_RenderTargetCreate(240, 400, GPU_RB_RGBA8, depthFmt);
        C3D_RenderTargetSetOutput(renderTarget, GFX_TOP, GFX_LEFT,
            GX_TRANSFER_FLIP_VERT(0) | GX_TRANSFER_OUT_TILED(0) | GX_TRANSFER_RAW_COPY(0) |
            GX_TRANSFER_IN_FORMAT(GX_TRANSFER_FMT_RGBA8) | GX_TRANSFER_OUT_FORMAT(GX_TRANSFER_FMT_RGB8) |
            GX_TRANSFER_SCALING(GX_TRANSFER_SCALE_NO));
        
        // init scene
        // transmute to a *mut u32 since that's what the bindgen'd bindings want...
        debug_print(format!("{:X?}", shbin_data).as_str());
        let shader_dvlb = DVLB_ParseFile(std::mem::transmute::<*const u8, *mut u32>(shbin_data.as_ptr()), shbin_data.len() as u32);

        if shader_dvlb.is_null(){
            panic!("shader DVLB parse failed (it was {:?}, from {:x?}", shader_dvlb, shbin_data);
        }
        let mut program: ctru_sys::shaderProgram_s = ctru_sys::shaderProgram_s::default();
        let init_ret = ctru_sys::shaderProgramInit(&mut program as *mut _);

        let shader_dvle = (*(shader_dvlb as *mut ctru_sys::DVLB_s)).DVLE;
        if shader_dvle.is_null(){
            panic!("shader dvle is null");
        }
        let setvsh_ret = ctru_sys::shaderProgramSetVsh(&mut program, shader_dvle);
        debug_print(format!("program {:?} rets: {} {}", shader_dvlb, init_ret, setvsh_ret).as_ref());


        // Get the location of the uniforms
        let uLoc_projection = ctru_sys::shaderInstanceGetUniformLocation(program.vertexShader, uLoc_projection_name.as_ptr() as *const u8);
        let uLoc_modelView = ctru_sys::shaderInstanceGetUniformLocation(program.vertexShader, uLoc_modelView_name.as_ptr() as *const u8);

        debug_print(format!("uloc_proj/mv: {}, {}", uLoc_modelView, uLoc_projection).as_ref());

        // Configure attributes for use with the vertex shader
        let mut vbo_attrInfo: C3D_AttrInfo = Default::default();
        let mut vbo_bufInfo: C3D_BufInfo = Default::default();

        /*
        AttrInfo_Init(&mut vbo_attrInfo);
        AttrInfo_AddLoader(&mut vbo_attrInfo, 0, GPU_FLOAT, 3); // v0=position
        AttrInfo_AddLoader(&mut vbo_attrInfo, 1, GPU_FLOAT, 2); // v1=texcoord
        AttrInfo_AddLoader(&mut vbo_attrInfo, 2, GPU_FLOAT, 3); // v2=normal

        BufInfo_Init(&mut vbo_bufInfo);
        BufInfo_Add(&mut vbo_bufInfo, cube.as_ptr() as *const c_void, size_of::<C3D_Vertex>().try_into().unwrap(), 3, 0x210);
        debug_print(format!("cube sample: {:?}, {}", cube[0], cube.len()).as_ref());


        debug_print("init: c3d render target set output");

        */
        let clrWhite = C2D_Color32(0xFF, 0xFF, 0xFF, 0xFF);
        let clrGreen = C2D_Color32(0x00, 0xFF, 0x00, 0xFF);
        let clrBlack = C2D_Color32(0x00, 0x00, 0x00, 0xFF);
        let clrClear = C2D_Color32(0xFF, 0xD8, 0xB0, 0x68);

        let mut textBuf = C2D_TextBufNew(128);
        let mut text = C2D_Text::default();
        let textString = CString::new("hello world").unwrap();
        C2D_TextParse(&mut text as *mut _, textBuf, textString.as_ptr() as *const u8);
        C2D_TextOptimize(&mut text as *mut _);

        debug_print("done init");

        CitroLibContext {
            renderTarget,
            clrWhite,
            clrGreen,
            clrBlack,
            clrClear,
            textBuf,
            text,
            textString,
            scene: Scene {
                program,
                uLoc_projection,
                uLoc_modelView,
                vbo_attrInfo,
                vbo_bufInfo,
                projection: Matrix4::identity()
            }
        }
    }

}
pub fn exit() {
    unsafe {
        C2D_Fini();
        C3D_Fini();
    }
}

fn VecToC3D(v: Vector4<f32>) -> C3D_FVec {
    C3D_FVec {c: [v.w, v.z, v.y, v.x]}
}

pub fn MatrixToC3D(m: cgmath::Matrix4<f32>) -> C3D_Mtx {
    C3D_Mtx {
        r: [VecToC3D(m.row(0)), VecToC3D(m.row(1)), VecToC3D(m.row(2)), VecToC3D(m.row(3))]
    }
}

pub unsafe fn FVUnifMtx4x4(shader: GPU_SHADER_TYPE, id: i8, mut m: C3D_Mtx){
    let shader = shader as usize;
    let id = id as usize;
    // Set dirty bits
    for i in 0..4 {
        C3D_FVUnifDirty[shader][id+1] = true;
    }

    // ???
    /// [   2.132186] Debug.Emulated <Debug> core/hle/kernel/svc.cpp:OutputDebugString:832: thread 'main' panicked at 'index out of bounds: the len is 96 but the index is 4294967295', /home/vivlim/git/citro-3d-rs/citro-3d/src/lib.rs:174:32
    let mut destination = &mut C3D_FVUnif[shader][id];
    let mut destination = std::mem::transmute::<&mut C3D_FVec, &mut [C3D_FVec; 4]>(destination);
    let mut source = &mut m.r; // row access

    for i in 0..source.len() {
        // Vector access
        destination[i] = source[i]
    }

}

impl Scene {
    fn bind(&mut self){
        debug_print("enter bind");
        unsafe {
            // ... ... there has to be some better way :(
            let program_c3d = std::mem::transmute::<*mut ctru_sys::shaderProgram_s, *mut citro_3d_sys::shaderProgram_s>(&mut self.program as *mut ctru_sys::shaderProgram_s);

            C3D_BindProgram(program_c3d);
            C3D_SetAttrInfo(&mut self.vbo_attrInfo as *mut _);
            C3D_SetBufInfo(&mut self.vbo_bufInfo as *mut _);
            //C3D_LightEnvBind(&lightEnv);
            C3D_DepthTest(true, GPU_GREATER, GPU_WRITE_ALL);
            C3D_CullFace(GPU_CULL_BACK_CCW);

            // Configure the first fragment shading substage to blend the fragment primary color
            // with the fragment secondary color.
            // See https://www.opengl.org/sdk/docs/man2/xhtml/glTexEnv.xml for more insight
            let mut env = C3D_GetTexEnv(0);
            C3D_TexEnvInit(env);
            C3D_TexEnvSrc(env, C3D_Both, ctru_sys::GPU_FRAGMENT_PRIMARY_COLOR, Some(ctru_sys::GPU_FRAGMENT_SECONDARY_COLOR), None);
            C3D_TexEnvFunc(env, C3D_Both, ctru_sys::GPU_ADD);

            // Clear out the other texenvs
            C3D_TexEnvInit(C3D_GetTexEnv(1));
            C3D_TexEnvInit(C3D_GetTexEnv(2));
            C3D_TexEnvInit(C3D_GetTexEnv(3));
            C3D_TexEnvInit(C3D_GetTexEnv(4));
            C3D_TexEnvInit(C3D_GetTexEnv(5));
        }
        debug_print("exit bind");
    }

    fn render(&mut self, iod: f32){
        debug_print("enter render");
        unsafe {
            self.bind();

            // Compute the projection matrix
            //Mtx_PerspStereoTilt(&mut self.projection, 40.0*std::f32::consts::TAU / 360.0, C3D_AspectRatioTop, 0.01, 1000.0, iod, 2.0, false);

            let objPos = C3D_FVec { __bindgen_anon_1: C3D_FVec__bindgen_ty_1 {
                w: 0.0, z: 0.0, y: -3.0, x: 1.0
            }};
            let lightPos = C3D_FVec { __bindgen_anon_1: C3D_FVec__bindgen_ty_1 {
                w: 0.0, z: 0.0, y: -0.5, x: 1.0
            }};

            // Calculate the modelView matrix
            let mut modelView = MatrixToC3D(Matrix4::identity());
            Mtx_Translate(&mut modelView, objPos.__bindgen_anon_1.x, objPos.__bindgen_anon_1.y, objPos.__bindgen_anon_1.z, true);
            Mtx_RotateY(&mut modelView, std::f32::consts::TAU * 0.5, true);
            Mtx_Scale(&mut modelView, 2.0, 2.0, 2.0);

            //C3D_LightPosition(&light, &lightPos);

            debug_print("before update uniforms");
            // Update the uniforms
            FVUnifMtx4x4(GPU_VERTEX_SHADER, self.uLoc_projection, MatrixToC3D(self.projection.clone()));
            FVUnifMtx4x4(GPU_VERTEX_SHADER, self.uLoc_modelView,  modelView);

            debug_print("before draw vbo");
            // Draw the VBO
            C3D_DrawArrays(GPU_TRIANGLES, 0, cube.len().try_into().unwrap());

            /*
            // Draw the 2d scene
            C2D_Prepare();
            C2D_DrawText(&txt_helloWorld, 0, 8.0, 8.0, 1.0, 1.0, 1.0);
            C2D_Flush();
            */
        }
    }
}

pub fn on_main_loop(ctx: &mut CitroLibContext){
    unsafe {
        //let iod: f32 = ctru_sys::osGet3DSliderState()/3;
        let iod = 0.0;

        println!("\x1b[1;1HSimple citro2d shapes example");
        println!("\x1b[2;1HCPU:     {:.2}\x1b[K", C3D_GetProcessingTime()*6.0);
        println!("\x1b[3;1HGPU:     {:.2}\x1b[K", C3D_GetDrawingTime()*6.0);
        println!("\x1b[4;1HCmdBuf:  {:.2}\x1b[K", C3D_GetCmdBufUsage()*100.0);
        println!("{:?}", shbin_data.len()); // unmapped read. how do i get it to point at the actual shader...


        C3D_FrameBegin(C3D_FRAME_SYNCDRAW.try_into().unwrap());
        {
            // C3D_RenderTargetClear is an inline we don't get generated, just call C3D_FrameBufClear directly.
            C3D_FrameBufClear(&mut (*ctx.renderTarget).frameBuf, C3D_CLEAR_ALL, ctx.clrClear, 0);

            C3D_FrameDrawOn(ctx.renderTarget);
            ctx.scene.render(-iod);
            //C2D_Flush();
            //C2D_SceneSize(ctru_sys::GSP_SCREEN_WIDTH, ctru_sys::GSP_SCREEN_HEIGHT_TOP, true); // seems to have no effect
            //C2D_SceneTarget(ctx.renderTarget);
            //C2D_SceneBegin(ctx.renderTarget); // calls C2D_SceneTarget which calls C2D_SceneSize

            C3D_DrawArrays(GPU_TRIANGLES, 0, cube.len().try_into().unwrap());

            //C2D_DrawRectangle(0.0, 0.0, 0.0, 500.0, 500.0, ctx.clrGreen, ctx.clrGreen, ctx.clrGreen, ctx.clrGreen);

            //C2D_DrawText(&ctx.text as *const _, 0, 8.0, 8.0, 1.0, 1.0, 1.0);
        }
        C3D_FrameEnd(0);
    }
}