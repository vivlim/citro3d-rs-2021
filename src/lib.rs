extern crate pica_gl_sys;
extern crate ctru_sys;
extern crate ctru;

use core::default::Default;

fn main() {
    println!("Hello, world!");
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

    pub fn frame(&mut self){
        use pica_gl_sys::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT};
        ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::VBlank0, true);
        unsafe {

            pica_gl_sys::glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            pica_gl_sys::glLoadIdentity();
            pica_gl_sys::glTranslatef(0.0, 0.0, -2.5);
            pica_gl_sys::glRotatef(self.angle , 0.0, 0.0, 1.0);

            self.render();
            
            pica_gl_sys::pglSwapBuffers();
            self.angle += 0.5;
            println!("rendering a frame with angle {}", self.angle)
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