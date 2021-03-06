// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glfw;
extern crate glfw;

use gfx::format::{I8Scaled};

// Declare the vertex format suitable for drawing.
// Notice the use of FixedPoint.
gfx_vertex_struct!( Vertex {
    pos: [I8Scaled; 3] = "a_Pos",
    tex_coord: [I8Scaled; 2] = "a_TexCoord",
});

impl Vertex {
    fn new(p: [i8; 3], t: [i8; 2]) -> Vertex {
        Vertex {
            pos: I8Scaled::cast3(p),
            tex_coord: I8Scaled::cast2(t),
        }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
    color: gfx::TextureSampler<[f32; 4]> = "t_Color",
    out_color: gfx::RenderTarget<gfx::format::Rgba8> = "o_Color",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});


//----------------------------------------

pub fn main() {
    use cgmath::{Point3, Vector3};
    use cgmath::{Transform, AffineMatrix3};
    use glfw::Context;
    use gfx::traits::{Device, Factory, FactoryExt};

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    let (mut window, events) = glfw
        .create_window(640, 480, "Cube example", glfw::WindowMode::Windowed)
        .unwrap();
    window.set_key_polling(true);

    let (mut device, mut factory, main_color, main_depth) =
        gfx_window_glfw::init(&mut window);

    let vertex_data = [
        // top (0, 0, 1)
        Vertex::new([-1, -1,  1], [0, 0]),
        Vertex::new([ 1, -1,  1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([-1,  1,  1], [0, 1]),
        // bottom (0, 0, -1)
        Vertex::new([-1,  1, -1], [1, 0]),
        Vertex::new([ 1,  1, -1], [0, 0]),
        Vertex::new([ 1, -1, -1], [0, 1]),
        Vertex::new([-1, -1, -1], [1, 1]),
        // right (1, 0, 0)
        Vertex::new([ 1, -1, -1], [0, 0]),
        Vertex::new([ 1,  1, -1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([ 1, -1,  1], [0, 1]),
        // left (-1, 0, 0)
        Vertex::new([-1, -1,  1], [1, 0]),
        Vertex::new([-1,  1,  1], [0, 0]),
        Vertex::new([-1,  1, -1], [0, 1]),
        Vertex::new([-1, -1, -1], [1, 1]),
        // front (0, 1, 0)
        Vertex::new([ 1,  1, -1], [1, 0]),
        Vertex::new([-1,  1, -1], [0, 0]),
        Vertex::new([-1,  1,  1], [0, 1]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        // back (0, -1, 0)
        Vertex::new([ 1, -1,  1], [0, 0]),
        Vertex::new([-1, -1,  1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([ 1, -1, -1], [0, 1]),
    ];

    let index_data: &[u8] = &[
         0,  1,  2,  2,  3,  0, // top
         4,  5,  6,  6,  7,  4, // bottom
         8,  9, 10, 10, 11,  8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let (vbuf, slice) = factory.create_vertex_buffer_indexed(&vertex_data, index_data);

    let (_, texture_view) = factory.create_texture_const::<gfx::format::Rgba8>(
        gfx::tex::Kind::D2(1, 1, gfx::tex::AaMode::Single),
        &[[0x20, 0xA0, 0xC0, 0x00]],
        false
        ).unwrap();

    let sinfo = gfx::tex::SamplerInfo::new(
        gfx::tex::FilterMethod::Bilinear,
        gfx::tex::WrapMode::Clamp);

    let pso = factory.create_pipeline_simple(
        include_bytes!("cube_120.glslv"),
        include_bytes!("cube_120.glslf"),
        gfx::state::CullFace::Back,
        pipe::new()
        ).unwrap();

    let view: AffineMatrix3<f32> = Transform::look_at(
        Point3::new(1.5f32, -5.0, 3.0),
        Point3::new(0f32, 0.0, 0.0),
        Vector3::unit_z(),
    );
    let aspect = {
        let (w, h) = window.get_framebuffer_size();
        (w as f32) / (h as f32)
    };
    let proj = cgmath::perspective(cgmath::deg(45.0f32), aspect, 1.0, 10.0);

    let data = pipe::Data {
        vbuf: vbuf,
        transform: (proj * view.mat).into(),
        color: (texture_view, factory.create_sampler(sinfo)),
        out_color: main_color.clone(),
        out_depth: main_depth.clone(),
    };

    let mut encoder = factory.create_encoder();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    window.set_should_close(true),
                _ => {},
            }
        }

        encoder.reset();
        encoder.clear(&main_color, [0.3, 0.3, 0.3, 1.0]);
        encoder.clear_depth(&main_depth, 1.0);
        encoder.draw(&slice, &pso, &data);

        device.submit(encoder.as_buffer());
        window.swap_buffers();
        device.cleanup();
   }
}
