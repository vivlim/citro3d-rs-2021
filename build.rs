extern crate failure;

use std::{env, path::PathBuf, process::Stdio};
use std::process::Command;
use std::path::Path;
use std::io::{Write, Read};
use failure::Error;

fn main() {

    compile_shader(
        "render2d.v.pica",
    ).unwrap();

}

fn compile_shader(in_filename: &str) -> Result<(), Error>{
    let out_dir = env::var("OUT_DIR")?;
    println!("shader to compile: {}", in_filename);
    println!("outdir: {}", out_dir);

    // intermediate
    let mut shbin_path = Path::new(&out_dir).join(in_filename);
    shbin_path.set_extension("shbin");
    let shbin_path = shbin_path.into_os_string();
    println!("shbin path: {:?}", &shbin_path);

    // final output file
    let mut out_file = Path::new(&out_dir).join(in_filename);
    out_file.set_extension("shbin.o");
    let out_file = out_file.into_os_string();
    println!("final output path: {:?}", &out_file);


    let dkp_path = env::var("DEVKITPRO")?;
    let picasso_path = Path::new(&dkp_path).join("tools/bin/picasso");
    let bin2s_path = Path::new(&dkp_path).join("tools/bin/bin2s");

    let mut picasso_cmd = Command::new(picasso_path);
    picasso_cmd.args(&["-o", shbin_path.to_str().unwrap(), in_filename]);
    println!("picasso cmd: {:?}", &picasso_cmd);
    let mut picasso_proc = picasso_cmd.spawn()?;

    picasso_proc.wait()?;

    let mut bin2s_cmd = Command::new(bin2s_path);
    bin2s_cmd.args(&[&shbin_path]).stdout(Stdio::piped());
    println!("bin2s cmd: {:?}", &bin2s_cmd);
    let mut bin2s_proc = bin2s_cmd.spawn()?;

    let mut as_cmd = Command::new("as");
    as_cmd.args(&["-o", &out_file.to_str().unwrap()]).stdin(bin2s_proc.stdout.unwrap());
    println!("as cmd: {:?}", &as_cmd);
    let mut as_proc = as_cmd.spawn()?;

    &as_proc.wait()?;
    Ok(())
}