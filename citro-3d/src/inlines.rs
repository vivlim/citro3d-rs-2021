use citro_3d_sys::*;
use ctru_sys::*;
use std::convert::TryInto;
// inlines from citro3d that weren't bindgen'd

pub unsafe fn C3D_TexEnvInit(env: *mut C3D_TexEnv)
{
    (*env).srcRgb     = GPU_TEVSOURCES(GPU_PREVIOUS, 0, 0);
    (*env).srcAlpha   = (*env).srcRgb;
    (*env).__bindgen_anon_1.opAll      = 0;
    (*env).funcRgb    = GPU_REPLACE.try_into().unwrap();
    (*env).funcAlpha  = (*env).funcRgb;
    (*env).color      = 0xFFFFFFFF;
    (*env).scaleRgb   = GPU_TEVSCALE_1.try_into().unwrap();
    (*env).scaleAlpha = GPU_TEVSCALE_1.try_into().unwrap();
}

pub unsafe fn C3D_TexEnvSrc(env: *mut C3D_TexEnv, mode: C3D_TexEnvMode,
    s1: GPU_TEVSRC,
    s2: Option<GPU_TEVSRC>,
    s3: Option<GPU_TEVSRC>)
{
    let s2 = s2.unwrap_or(GPU_PRIMARY_COLOR);
    let s3 = s3.unwrap_or(GPU_PRIMARY_COLOR);

    let param = GPU_TEVSOURCES(s1, s2, s3);
    if mode & C3D_RGB == C3D_RGB{
        (*env).srcRgb = param.try_into().unwrap();
    }
    if mode & C3D_Alpha == C3D_Alpha{
        (*env).srcAlpha = param.try_into().unwrap();
    }
}

pub fn GPU_TEVSOURCES(a: GPU_TEVSRC, b: GPU_TEVSRC, c: GPU_TEVSRC) -> u16 {
    (a | (b << 4) | (c << 8)).try_into().unwrap()
}

pub unsafe fn C3D_TexEnvFunc(env: *mut C3D_TexEnv, mode: C3D_TexEnvMode, param: GPU_COMBINEFUNC)
{
    if mode & C3D_RGB == C3D_RGB
    {
        (*env).funcRgb = param.try_into().unwrap();
    }
    if mode & C3D_Alpha == C3D_Alpha
    {
        (*env).funcAlpha = param.try_into().unwrap();
    }
}