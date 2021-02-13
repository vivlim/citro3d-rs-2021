extern crate failure;
extern crate obj;
#[macro_use]
extern crate build_const;
extern crate citro_3d_sys;

use std::{env, path::PathBuf, process::Stdio};
use std::process::Command;
use std::path::Path;
use std::io::{Write, Read};
use failure::Error;

use build_const::ConstWriter;
use obj::{load_obj, Obj, Vertex, TexturedVertex};

include!("src/vertex.rs");

fn main() {

    load_models().unwrap();

    compile_shader(
        "vshader.v.pica",
    ).unwrap();
}

fn compile_shader(in_filename: &str) -> Result<(), Error>{
    let out_dir = env::var("OUT_DIR")?;
    println!("shader to compile: {}", in_filename);
    println!("outdir: {}", out_dir);

    // intermediate
    let mut shbin_path = Path::new(&out_dir).join(in_filename.split(".").next().unwrap());
    shbin_path.set_extension("shbin");
    let shbin_path = shbin_path.into_os_string();
    println!("shbin path: {:?}", &shbin_path);

    // final output file
    let mut out_file = Path::new(&out_dir).join(in_filename.split(".").next().unwrap());
    out_file.set_extension("shbin.o");
    let out_file = out_file.into_os_string();
    println!("final output path: {:?}", &out_file);


    let dkp_path = env::var("DEVKITPRO")?;
    let picasso_path = Path::new(&dkp_path).join("tools/bin/picasso");
    let bin2s_path = Path::new(&dkp_path).join("tools/bin/bin2s");
    let as_path = Path::new(&dkp_path).join("devkitARM/arm-none-eabi/bin/as");

    let mut picasso_cmd = Command::new(picasso_path);
    picasso_cmd.args(&["-o", shbin_path.to_str().unwrap(), in_filename]);
    println!("picasso cmd: {:?}", &picasso_cmd);
    let mut picasso_proc = picasso_cmd.spawn()?;

    picasso_proc.wait()?;

    /*
    let mut bin2s_cmd = Command::new(bin2s_path);
    bin2s_cmd.args(&[&shbin_path]).stdout(Stdio::piped());
    println!("bin2s cmd: {:?}", &bin2s_cmd);
    let mut bin2s_proc = bin2s_cmd.spawn()?;

    let mut as_cmd = Command::new(as_path);
    as_cmd.args(&["-o", &out_file.to_str().unwrap()]).stdin(bin2s_proc.stdout.unwrap());
    println!("as cmd: {:?}", &as_cmd);
    let mut as_proc = as_cmd.spawn()?;

    &as_proc.wait()?;
    */

    Ok(())
}

fn load_models() -> Result<(), Error>{
    use std::fs::File;
    use std::io::BufReader;

    let mut consts = ConstWriter::for_build(
        Path::new("model_data").to_str().unwrap()
    )?;

    let input = BufReader::new(File::open("cube.obj")?);
    let obj: Obj<TexturedVertex, u32> = load_obj(input)?;
    let c3d_vertices: Vec<C3D_Vertex> = obj.vertices
        .into_iter()
        .map(|v| C3D_Vertex {
            position: v.position,
            texcoord: [v.texture[0], v.texture[1]], // I don't know what it means for this to be 3d in obj but 2d in c3d?
            normal: v.normal,
        }).collect();

    let mut consts = consts.finish_dependencies();
    consts.add_array("cube", "C3D_Vertex", c3d_vertices.as_slice());

    Ok(())
}