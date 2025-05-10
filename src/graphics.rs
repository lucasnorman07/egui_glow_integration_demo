use super::Viewport;
use cgmath::{Deg, Matrix4, Point3, Vector3};
use glow::HasContext;
use std::fs;

pub struct GraphicsExample {
    vao: glow::VertexArray,
    ebo: glow::Buffer,
    shader_program: glow::NativeProgram,
    indices: Vec<u32>,
    texture: glow::NativeTexture,
}

impl GraphicsExample {
    pub fn new(gl: &glow::Context) -> Self {
        let texture = create_texture(gl, "assets/bunny_texture.png");

        let shader_program =
            create_shader_program(gl, "shaders/shader.vert", "shaders/shader.frag");

        let verticies: Vec<f32> = vec![
            //   Position           Tex Coords  Color
            // Front face
            -1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            // Back face
            -1.0, -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0,
            // Left face
            -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            // Right face
            1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            // Bottom face
            -1.0, -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0,
            // Top face
            -1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
        ];

        let indices: Vec<u32> = vec![
            // Front face
            0, 1, 2, 2, 3, 0, // Back face
            4, 5, 6, 6, 7, 4, // Left face
            8, 9, 10, 10, 11, 8, // Right face
            12, 13, 14, 14, 15, 12, // Bottom face
            16, 17, 18, 18, 19, 16, // Top face
            20, 21, 22, 22, 23, 20,
        ];

        let vao = create_vertex_array(gl, &verticies);
        let ebo = create_index_buffer(gl, &indices);

        // Create the vertex attribute pointers (layout of vertex attributes)
        let stride = (8 * std::mem::size_of::<f32>()) as i32;
        unsafe {
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(
                1,
                2,
                glow::FLOAT,
                false,
                stride,
                (3 * std::mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(1);

            gl.vertex_attrib_pointer_f32(
                2,
                3,
                glow::FLOAT,
                false,
                stride,
                (5 * std::mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(2);
        }

        GraphicsExample {
            vao,
            ebo,
            shader_program,
            indices,
            texture,
        }
    }

    pub fn clear(&self, gl: &glow::Context) {
        unsafe {
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn render(
        &mut self,
        gl: &glow::Context,
        viewport: &Viewport,
        translate: (f32, f32, f32),
        rotate: (f32, f32, f32),
        scale: (f32, f32, f32),
    ) {
        unsafe {
            gl.clear(glow::DEPTH_BUFFER_BIT);
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LESS);
            // Makes sure that everything is renderered in the central panel of the ui
            gl.viewport(viewport.x, viewport.y, viewport.width, viewport.height);
        }

        // Bind textures, select the shader program, add uniforms and render
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));

            gl.use_program(Some(self.shader_program));

            gl.active_texture(glow::TEXTURE0);
            let texture_uniform = gl
                .get_uniform_location(self.shader_program, "image")
                .expect("Could not find the uniform called 'image'");
            gl.uniform_1_i32(Some(&texture_uniform), 0);

            let mvp_matrix = create_mvp_matrix(viewport, translate, rotate, scale);
            let mvp_matrix: &[f32; 16] = mvp_matrix.as_ref();
            let camera_matrix_uniform = gl
                .get_uniform_location(self.shader_program, "camMatrix")
                .expect("Could not find the uniform called 'camMatrix'");
            gl.uniform_matrix_4_f32_slice(Some(&camera_matrix_uniform), false, mvp_matrix);

            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.draw_elements(
                glow::TRIANGLES,
                self.indices.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );
        }
    }
}

// Creates a model view projection matrix
fn create_mvp_matrix(
    viewport: &Viewport,
    translate: (f32, f32, f32),
    rotate: (f32, f32, f32),
    scale: (f32, f32, f32),
) -> cgmath::Matrix4<f32> {
    let projection_matrix = cgmath::perspective(
        Deg(45.0),
        viewport.width as f32 / viewport.height as f32,
        0.1,
        100.0,
    );
    // Should be used if camera movement was implemented
    let _view_matrix = Matrix4::look_at_rh(
        Point3::new(0.0, 0.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let translation_matrix = Matrix4::from_translation(Vector3 {
        x: translate.0,
        y: translate.1,
        z: translate.2,
    });
    let rotation_matrix = Matrix4::from_angle_x(Deg(rotate.0))
        * Matrix4::from_angle_y(Deg(rotate.1))
        * Matrix4::from_angle_z(Deg(rotate.2));
    let scale_matrix = Matrix4::new(
        scale.0, 0.0, 0.0, 0.0, 0.0, scale.1, 0.0, 0.0, 0.0, 0.0, scale.2, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let model_matrix = translation_matrix * rotation_matrix * scale_matrix;

    projection_matrix * model_matrix
}

fn create_shader_program(
    gl: &glow::Context,
    vertex_shader_path: &str,
    fragment_shader_path: &str,
) -> glow::NativeProgram {
    unsafe {
        let shader_source = fs::read_to_string(vertex_shader_path).unwrap();
        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vertex_shader, &shader_source);
        gl.compile_shader(vertex_shader);

        if !gl.get_shader_compile_status(vertex_shader) {
            panic!(
                "Error compiling vertex shader: {}",
                gl.get_shader_info_log(vertex_shader)
            );
        }

        let shader_source = fs::read_to_string(fragment_shader_path).unwrap();
        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fragment_shader, &shader_source);
        gl.compile_shader(fragment_shader);

        if !gl.get_shader_compile_status(fragment_shader) {
            panic!(
                "Error compiling fragment shader: {}",
                gl.get_shader_info_log(fragment_shader)
            );
        }

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(shader_program, vertex_shader);
        gl.attach_shader(shader_program, fragment_shader);
        gl.link_program(shader_program);

        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        if !gl.get_program_link_status(shader_program) {
            panic!(
                "Shader link error: {}",
                gl.get_program_info_log(shader_program)
            );
        }

        shader_program
    }
}

fn create_texture(gl: &glow::Context, image_path: &str) -> glow::NativeTexture {
    let img = image::open(image_path).unwrap().flipv().to_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();

    unsafe {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));

        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as i32,
        );

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            glow::PixelUnpackData::Slice(Some(&data)),
        );

        gl.generate_mipmap(glow::TEXTURE_2D);

        texture
    }
}

fn create_vertex_array(gl: &glow::Context, verticies: &[f32]) -> glow::VertexArray {
    unsafe {
        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(verticies),
            glow::STATIC_DRAW,
        );

        vao
    }
}

fn create_index_buffer(gl: &glow::Context, indices: &[u32]) -> glow::Buffer {
    unsafe {
        let ebo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(indices),
            glow::STATIC_DRAW,
        );

        ebo
    }
}
