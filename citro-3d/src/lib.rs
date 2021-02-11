extern crate citro_3d_sys;
extern crate ctru_sys;
extern crate ctru;


use core::default::Default;
use std::ffi::CString;
use citro_3d_sys::*;
use ctru_sys::{DVLB_s, DVLB_ParseFile, GPU_RB_DEPTH24_STENCIL8, gfxSet3D};
use std::convert::TryInto;

#[macro_use]
pub mod macros;

static shbin_data: &[u8] = include_bytes_align_as!(u32, concat!(env!("OUT_DIR"), "/render2d.v.shbin.o"));

pub struct CitroLibContext {
    renderTarget: *mut C3D_RenderTarget,
    clrWhite: u32,
    clrGreen: u32,
    clrBlack: u32,
    clrClear: u32,
    textBuf: *mut C2D_TextBuf_s,
    text: C2D_Text,
    textString: CString,
    shader_dvlb: *mut DVLB_s,
    program: ctru_sys::shaderProgram_s,
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

pub fn init() -> CitroLibContext {
    unsafe {
        C3D_Init(C3D_DEFAULT_CMDBUF_SIZE);
        
        let shader_dvlb = DVLB_ParseFile(std::mem::transmute::<_, *mut u32>(&shbin_data), shbin_data.len() as u32);
        let mut program: ctru_sys::shaderProgram_s = ctru_sys::shaderProgram_s::default();
        ctru_sys::shaderProgramInit(&mut program);
        ctru_sys::shaderProgramSetVsh(&mut program, (*shader_dvlb).DVLE);

        C2D_Init(C2D_DEFAULT_MAX_OBJECTS);
        C2D_Prepare();
        //let renderTarget = C2D_CreateScreenTarget(GFX_TOP, GFX_LEFT);
        let mut depthFmt = C3D_DEPTHTYPE::default();
        depthFmt.__e = GPU_RB_DEPTH16;
        let renderTarget = C3D_RenderTargetCreate(240, 400, GPU_RB_RGBA8, depthFmt);
        C3D_RenderTargetSetOutput(renderTarget, GFX_TOP, GFX_LEFT,
            GX_TRANSFER_FLIP_VERT(0) | GX_TRANSFER_OUT_TILED(0) | GX_TRANSFER_RAW_COPY(0) |
            GX_TRANSFER_IN_FORMAT(GX_TRANSFER_FMT_RGBA8) | GX_TRANSFER_OUT_FORMAT(GX_TRANSFER_FMT_RGB8) |
            GX_TRANSFER_SCALING(GX_TRANSFER_SCALE_NO));

        let clrWhite = C2D_Color32(0xFF, 0xFF, 0xFF, 0xFF);
        let clrGreen = C2D_Color32(0x00, 0xFF, 0x00, 0xFF);
        let clrBlack = C2D_Color32(0x00, 0x00, 0x00, 0xFF);
        let clrClear = C2D_Color32(0xFF, 0xD8, 0xB0, 0x68);

        let mut textBuf = C2D_TextBufNew(128);
        let mut text = C2D_Text::default();
        let textString = CString::new("hello world").unwrap();
        C2D_TextParse(&mut text as *mut _, textBuf, textString.as_ptr() as *const u8);
        C2D_TextOptimize(&mut text as *mut _);


        CitroLibContext {
            renderTarget,
            clrWhite,
            clrGreen,
            clrBlack,
            clrClear,
            textBuf,
            text,
            textString,
            shader_dvlb,
            program,
        }


    }
}
pub fn exit() {
    unsafe {
        C2D_Fini();
        C3D_Fini();
    }
}

pub fn on_main_loop(ctx: &mut CitroLibContext){
    unsafe {
        println!("\x1b[1;1HSimple citro2d shapes example");
        println!("\x1b[2;1HCPU:     {:.2}\x1b[K", C3D_GetProcessingTime()*6.0);
        println!("\x1b[3;1HGPU:     {:.2}\x1b[K", C3D_GetDrawingTime()*6.0);
        println!("\x1b[4;1HCmdBuf:  {:.2}\x1b[K", C3D_GetCmdBufUsage()*100.0);
        println!("{:?}", shbin_data.len()); // unmapped read. how do i get it to point at the actual shader...


        C3D_FrameBegin(C3D_FRAME_SYNCDRAW.try_into().unwrap());
        C2D_TargetClear(ctx.renderTarget, ctx.clrClear);
        C2D_Flush();
        C3D_FrameDrawOn(ctx.renderTarget);
        // I think I need to get render2d.v.pica in, that contains a matrix
        C2D_SceneSize(ctru_sys::GSP_SCREEN_HEIGHT_TOP, ctru_sys::GSP_SCREEN_WIDTH, true); // seems to have no effect
        //C2D_SceneTarget(ctx.renderTarget);
        //C2D_SceneBegin(ctx.renderTarget); // calls C2D_SceneTarget which calls C2D_SceneSize

        C2D_DrawRectangle(0.0, 0.0, 0.0, 500.0, 500.0, ctx.clrGreen, ctx.clrGreen, ctx.clrGreen, ctx.clrGreen);

        C2D_DrawText(&ctx.text as *const _, 0, 8.0, 8.0, 1.0, 1.0, 1.0);
        C3D_FrameEnd(0);
    }
}