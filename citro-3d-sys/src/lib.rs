extern crate libc;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod bindings;

// static inlines that can't be bindgen'd currently
// https://github.com/rust-lang/rust-bindgen/issues/1090

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod inlines {
    use crate::bindings::*;
    pub fn C2D_Clamp(x: f32, min: f32, max: f32) -> f32 
    {
        if x <= min {
            min
        }
        else if x >= max
        {
            max
        }
        else {
            x
        }
    }

    pub fn C2D_FloatToU8(x: f32) -> u8
    {
        (255.0*C2D_Clamp(x, 0.0, 1.0)+0.5) as u8
    }

    pub fn C2D_Color32(r: u8, g: u8, b: u8, a: u8) -> u32
    {
        r as u32 | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24)
    }

    pub fn C2D_Color32f(r: f32, g: f32, b: f32, a: f32) -> u32
    {
        C2D_Color32(C2D_FloatToU8(r),C2D_FloatToU8(g),C2D_FloatToU8(b),C2D_FloatToU8(a))
    }

    pub unsafe fn C2D_SceneBegin(target: *mut C3D_RenderTarget){
        C2D_Flush();
        C3D_FrameDrawOn(target);
        C2D_SceneTarget(target);
    }
    
    pub unsafe fn C2D_SceneTarget(target: *mut C3D_RenderTarget){
        C2D_SceneSize(u32::from((*target).frameBuf.width), u32::from((*target).frameBuf.height), (*target).linked)
    }

}

pub use bindings::*;
pub use inlines::*;