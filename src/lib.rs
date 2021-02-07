extern crate pica_gl_sys;
extern crate ctru_sys;
extern crate ctru;
//extern crate gfx_backend_gl;
extern crate glow;

use core::default::Default;

pub fn main() {
}
pub struct PicaGlContext {
    textures: Vec<Texture>,
    angle: f32, // just for demo

}

pub struct Texture {
    id: pica_gl_sys::GLuint,
    data: Box<[u32]>
}

impl PicaGlContext {
    pub fn new() -> PicaGlContext {
        use pica_gl_sys::{GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA, GL_LESS, GL_BLEND, GL_ALPHA_TEST, GL_DEPTH_TEST, GL_TEXTURE_2D, GL_CULL_FACE, GL_PROJECTION, GL_MODELVIEW};

        unsafe {
            pica_gl_sys::pglInit();
            pica_gl_sys::glClearColor(0.2, 0.2, 0.2, 1.0);
            pica_gl_sys::glViewport(0,0, 400, 240);

            pica_gl_sys::glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
            pica_gl_sys::glDepthFunc(GL_LESS);

            pica_gl_sys::glEnable(GL_BLEND);
            pica_gl_sys::glEnable(GL_ALPHA_TEST);
            pica_gl_sys::glEnable(GL_DEPTH_TEST);
            pica_gl_sys::glEnable(GL_TEXTURE_2D);

            pica_gl_sys::glDisable(GL_CULL_FACE);
            pica_gl_sys::glEnableClientState(pica_gl_sys::GL_VERTEX_ARRAY);

            pica_gl_sys::glMatrixMode(GL_PROJECTION);
            pica_gl_sys::glLoadIdentity();

            pica_gl_sys::gluPerspective(80.0, 400.0/240.0, 0.01, 100.0);

            pica_gl_sys::glMatrixMode(GL_MODELVIEW);
            pica_gl_sys::glLoadIdentity();
        }

        return PicaGlContext{
            textures: Default::default(),
            angle: 0.0
        }
    }

    pub fn render(&mut self){
        use pica_gl_sys::{GL_TEXTURE_2D, GL_TRIANGLES};
        unsafe {
            pica_gl_sys::glBindTexture(GL_TEXTURE_2D, self.textures.first().unwrap().id);

            pica_gl_sys::glBegin(GL_TRIANGLES);
            pica_gl_sys::glTexCoord2f(0.0, 0.0);
            pica_gl_sys::glVertex3f(-10.0, -10.0, 0.0 );
            pica_gl_sys::glTexCoord2f(0.0, 10.0);
            pica_gl_sys::glVertex3f(-10.0,  10.0, 0.0 );
            pica_gl_sys::glTexCoord2f(10.0, 10.0);
            pica_gl_sys::glVertex3f( 10.0,  10.0, 0.0 );
            pica_gl_sys::glEnd();
        }
    }

    pub fn frame(&mut self, renderFn: &mut impl FnMut(&mut bool)){
        use pica_gl_sys::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT};
        ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::VBlank0, true);
        unsafe {

            pica_gl_sys::glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            /*
            pica_gl_sys::glLoadIdentity();
            pica_gl_sys::glTranslatef(0.0, 0.0, -2.5);
            pica_gl_sys::glRotatef(self.angle , 0.0, 0.0, 1.0);
            */

            renderFn(&mut true);
            
            pica_gl_sys::pglSwapBuffers();
            //self.angle += 0.5;
            //println!("rendering a frame with angle {}", self.angle)
        }
    }

    pub fn checkerboard_texture(&mut self){
        use pica_gl_sys::{GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_MAG_FILTER, GL_NEAREST, GL_RGBA, GL_UNSIGNED_BYTE};

        let mut me = Texture {
            id: 1,
            data: Box::new([0; 32*32])
        };

        let mut i = 0;

        //Generate a basic checkerboard pattern
        for y in 0..32
        {
            for x in 0..32
            {
                if((x + y) % 2 == 0){
                    me.data[i] = 0xFF000000;
                }
                else{
                    me.data[i] = 0xFFFFFFFF;
                }
                i += 0;
            }
        }

        unsafe {
            pica_gl_sys::glGenTextures (1, &mut me.id);
            pica_gl_sys::glBindTexture(GL_TEXTURE_2D, me.id);
            let data_ptr = me.data.as_mut_ptr() as *mut pica_gl_sys::GLvoid;
            pica_gl_sys::glTexImage2D(GL_TEXTURE_2D, 0, 0, 32, 32, 0, GL_RGBA, GL_UNSIGNED_BYTE, data_ptr);
            pica_gl_sys::glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as f32);
            pica_gl_sys::glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as f32);
        }

        self.textures.push(me);
    }

}
pub mod test_glow {
    use glow::*;

    pub fn main() -> impl FnMut(&mut bool) {
        unsafe {
            let gl = glow::Context::from_loader_function(|s| {
                    std::ptr::null()
                });

                let vertex_array: [pica_gl_sys::GLfloat;9] = [-0.1, -0.25, 0.0,
                0.1, -0.25, 0.0,
                0.0, 0.559016994, 0.0];

                /* 
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vertex_array));

            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                r#"const vec2 verts[3] = vec2[3](
                    vec2(0.5f, 1.0f),
                    vec2(0.0f, 0.0f),
                    vec2(1.0f, 0.0f)
                );
                out vec2 vert;
                void main() {
                    vert = verts[gl_VertexID];
                    gl_Position = vec4(vert - 0.5, 0.0, 1.0);
                }"#,
                r#"precision mediump float;
                in vec2 vert;
                out vec4 color;
                void main() {
                    color = vec4(vert, 0.5, 1.0);
                }"#,
            );

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");

                //gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!(gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!(gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }
            */

            

            //gl.use_program(Some(program));
            gl.clear_color(0.1, 0.2, 0.3, 1.0);

            //render_loop.run(move |running: &mut bool| {
            return move |running: &mut bool| {
                // this would be an OK place to poll inputs
                println!("render loop");

                gl.clear(glow::COLOR_BUFFER_BIT);

                pica_gl_sys::glColor4f(0.6367, 0.76, 0.22, 0.0);
                pica_gl_sys::glVertexPointer(3, pica_gl_sys::GL_FLOAT, 0, vertex_array.as_ptr() as *const std::ffi::c_void);
                gl.draw_arrays(glow::TRIANGLES, 0, 3);

                if !*running {
                    //gl.delete_program(program);
                    //gl.delete_vertex_array(vertex_array);
                }
            };
        }
    }

}