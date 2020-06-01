use gleam::gl::Gl;

use crate::call;
use crate::Recording;
use crate::serialize::Deserialize;

/*
/// A `Gl` parameter type that we can deserialize without custom code.
///
/// There are three levels of 'difficulty', so to speak, in obtaining parameter
/// values we can pass to `Gl` trait methods:
///
/// - The easiest are types like `f32`. These are stored directly in the `Call`,
///   and can be passed directly to the `Gl` method. (We may add an exception
///   for methods with very long argument lists.)
///
/// - The next level of difficulty is types like `&[GLuint]`. These can't be
///   stored in the `Call`, but they can be borrowed directly from the
///   variable-length data section.
///
/// - The worst level of difficulty is types like &[&[u8]]. These need to be
///   deserialized to a `Vec<&[u8]>`, which must then be moved to a location
///   that is owned for the duration of the `Gl` method call, and borrowed to be
///   passed.
///
/// This trait handles the worst case in a way that should still generate good
/// code for the simpler cases. If the `Call` variant for a method `m` has a
/// field named `x`, we call the method with code like:
///
///    let x = x.from_call(&variable);
///    gl.m(..., x.to_argument(), ...)
///
/// This handles all three cases:
///
/// - For a simple type like `f32`, `from_call` is the identity function, and
///   `to_argument` simply dereferences its `&self` argument.
///
/// - For an argument type like `&[GLuint]`, `from_call` borrows data from the
///   variable-length section, and `to_argument` dereferences.
///
/// - For an argument type like `&[&[u8]]`, `from_call` builds a `Vec<&[u8]>`,
///   which is owned by `x` across the call, and `to_argument` borrows the outer
///   slice.
///
/// The trait's lifetime parameter `'v` is the lifetime of the variable-length
/// data, to allow `from_call` implementations to borrow from there.
trait Parameter<'v>: Sized {
    /// The owned form of the type, as returned by `from_call`.
    type Held;

    /// The type actually passed to the `Gl` method, as returned by `to_argument`.
    type Passed;

    fn from_call(self, variable: &'v [u8]) -> Holder<'v, Self>;
    fn to_argument(held: &Self::Held) -> Self::Passed;
}

/// Helper type for `Parameter` trait.
struct Holder<'v, T: Parameter<'v>>(T::Held);

impl<'v, T: Parameter<'v>> Holder<'v, T> {
    fn to_argument(&self) -> T::Passed {
        T::to_argument(&self.0)
    }
}

impl<'v> Parameter<'v> for &'v str {
    type Held = &'v str;
    type Passed = &'v str;
    fn from_call(self, mut variable: &'v [u8]) -> Holder<&'v str> {
        Holder(<&'v str>::deserialize(&mut variable).expect("deserializing `str` failed"))
    }
    fn to_argument(held: &&'v str) -> &'v str {
        *held
    }
}

impl<'v> Parameter<'v> for 

macro_rules! simply_deserialized_types {
    ( $( $type:ty ),* ) => {
        $(
            impl<'v> Parameter<'v> for $type {
                type Held = $type;
                type Passed = $type;
                fn from_call(self, mut variable: &'v [u8]) -> Holder<$type> {
                    Holder(<$type>::deserialize(&mut variable)
                           .expect(concat!("deserializing ",
                                           stringify!($type),
                                           " failed")))
                }
                fn to_argument(held: & $type) -> $type {
                    *held
                }
            }
        )*
    }
}

// These types are serialized in variable content as themselves.
simply_deserialized_types!(bool, u8, u32, i32, f32, f64, usize);
 */


#[allow(unused_variables)]
pub fn replay(gl: &dyn Gl, recording: &Recording) {
    for call in &recording.calls {
        let call = *call;
        use call::Call::*;
        match call {
            active_texture { texture } => { gl.active_texture(texture); }
            bind_buffer { target, buffer } => { gl.bind_buffer(target, buffer); }
            bind_texture { target, texture } => { gl.bind_texture(target, texture); }
            bind_vertex_array { vao } => { gl.bind_vertex_array(vao); }
            buffer_data_untyped {
                target,
                size_data,
                usage,
            } => {
                let mut variable = &recording.variable[..];
                let size_data = call::GlRawBuf::from(
                    <&[u8]>::deserialize(&mut variable)
                        .expect("failed to deserialize data for buffer_data_untyped"));
                gl.buffer_data_untyped(target, size_data.size, size_data.data, usage)
            }
            clear_color { r, g, b, a } => { gl.clear_color(r, g, b, a); }
            disable { cap } => { gl.disable(cap); }
            disable_vertex_attrib_array { index } => { gl.disable_vertex_attrib_array(index); }
            enable { cap } => { gl.enable(cap); }
            enable_vertex_attrib_array { index } => { gl.enable_vertex_attrib_array(index); }
            gen_buffers { n, returned } => unimplemented!("gen_buffers"),
            gen_framebuffers { n, returned } => unimplemented!("gen_framebuffers"),
            gen_queries { n, returned } => unimplemented!("gen_queries"),
            gen_renderbuffers { n, returned } => unimplemented!("gen_renderbuffers"),
            gen_textures { n, returned } => unimplemented!("gen_textures"),
            gen_vertex_arrays { n, returned } => unimplemented!("gen_vertex_arrays"),
            gen_vertex_arrays_apple { n, returned } => unimplemented!("gen_vertex_arrays_apple"),
            line_width { width } => { gl.line_width(width); }
            pixel_store_i { name, param } => { gl.pixel_store_i(name, param); }
            scissor {
                x,
                y,
                width,
                height,
            } => { gl.scissor(
                x,
                y,
                width,
                height,
            ); }
            tex_image_2d {
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                ty,
                opt_data,
            } => unimplemented!() /*gl.tex_image_2d(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                ty,
                opt_data,
)*/,
            tex_image_3d {
                target,
                level,
                internal_format,
                width,
                height,
                depth,
                border,
                format,
                ty,
                opt_data,
            } => unimplemented!("tex_image_3d") /*gl.tex_image_3d(
                target,
                level,
                internal_format,
                width,
                height,
                depth,
                border,
                format,
                ty,
                opt_data,
)*/,
            tex_parameter_f {
                target,
                pname,
                param,
            } => { gl.tex_parameter_f(
                target,
                pname,
                param,
            ); }
            tex_parameter_i {
                target,
                pname,
                param,
            } => { gl.tex_parameter_i(
                target,
                pname,
                param,
            ); }
            tex_sub_image_3d {
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                ty,
                data,
            } => unimplemented!("tex_sub_image_3d") /*gl.tex_sub_image_3d(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                ty,
                data,
)*/,
            use_program { program } => { gl.use_program(program); }
            vertex_attrib_divisor { index, divisor } => { gl.vertex_attrib_divisor(index, divisor); }
            vertex_attrib_i_pointer {
                index,
                size,
                type_,
                stride,
                offset,
            } => { gl.vertex_attrib_i_pointer(
                index,
                size,
                type_,
                stride,
                offset,
            ); }
            vertex_attrib_pointer {
                index,
                size,
                type_,
                normalized,
                stride,
                offset,
            } => { gl.vertex_attrib_pointer(
                index,
                size,
                type_,
                normalized,
                stride,
                offset,
            ); }
            viewport {
                x,
                y,
                width,
                height,
            } => { gl.viewport(
                x,
                y,
                width,
                height,
            ); }
            bind_vertex_array_apple { vao } => { gl.bind_vertex_array_apple(vao); }
            bind_renderbuffer {
                target,
                renderbuffer,
            } => { gl.bind_renderbuffer(
                target,
                renderbuffer,
            ); }
            bind_framebuffer {
                target,
                framebuffer,
            } => { gl.bind_framebuffer(
                target,
                framebuffer,
            ); }
            framebuffer_texture_2d {
                target,
                attachment,
                textarget,
                texture,
                level,
            } => { gl.framebuffer_texture_2d(
                target,
                attachment,
                textarget,
                texture,
                level,
            ); }
            framebuffer_texture_layer {
                target,
                attachment,
                texture,
                level,
                layer,
            } => { gl.framebuffer_texture_layer(
                target,
                attachment,
                texture,
                level,
                layer,
            ); }
            blit_framebuffer {
                src_x0,
                src_y0,
                src_x1,
                src_y1,
                dst_x0,
                dst_y0,
                dst_x1,
                dst_y1,
                mask,
                filter,
            } => { gl.blit_framebuffer(
                src_x0,
                src_y0,
                src_x1,
                src_y1,
                dst_x0,
                dst_y0,
                dst_x1,
                dst_y1,
                mask,
                filter,
            ); }
            hint {
                param_name,
                param_val,
            } => { gl.hint(
                param_name,
                param_val,
            ); }
            is_enabled { cap } => { gl.is_enabled(cap); }
            is_shader { shader } => { gl.is_shader(shader); }
            is_texture { texture } => { gl.is_texture(texture); }
            is_framebuffer { framebuffer } => { gl.is_framebuffer(framebuffer); }
            is_renderbuffer { renderbuffer } => { gl.is_renderbuffer(renderbuffer); }
            check_frame_buffer_status { target } => { gl.check_frame_buffer_status(target); }
            renderbuffer_storage {
                target,
                internalformat,
                width,
                height,
            } => { gl.renderbuffer_storage(
                target,
                internalformat,
                width,
                height,
            ); }
            framebuffer_renderbuffer {
                target,
                attachment,
                renderbuffertarget,
                renderbuffer,
            } => { gl.framebuffer_renderbuffer(
                target,
                attachment,
                renderbuffertarget,
                renderbuffer,
            ); }
            tex_sub_image_2d_pbo {
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                ty,
                offset,
            } => { gl.tex_sub_image_2d_pbo(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                ty,
                offset,
            ); }
            flush {} => { gl.flush(); }
            finish {} => { gl.finish(); }
            depth_mask { flag } => { gl.depth_mask(flag); }
            create_program { returned } => unimplemented!("create_program"),
            create_shader {
                shader_type,
                returned,
            } => unimplemented!("create_shader"),
            shader_source { shader, strings } => unimplemented!("shader_source"), /*{ gl.shader_source(shader, strings); }*/
            compile_shader { shader } => { gl.compile_shader(shader); }
            get_shader_iv {
                shader,
                pname,
                result,
            } => unimplemented!("get_shader_iv"), /*{ gl.get_shader_iv(
                shader,
                pname,
                result,
            ); }*/
            attach_shader { program, shader } => { gl.attach_shader(program, shader); }
            bind_attrib_location {
                program,
                index,
                name,
            } => unimplemented!("bind_attrib_location"), /*{ gl.bind_attrib_location(
                program,
                index,
                name,
            ); }*/
            link_program { program } => { gl.link_program(program); }
            delete_shader { shader } => { gl.delete_shader(shader); }
            detach_shader { program, shader } => { gl.detach_shader(program, shader); }
            clear { buffer_mask } => { gl.clear(buffer_mask); }
            clear_depth { depth } => { gl.clear_depth(depth); }
            clear_stencil { s } => { gl.clear_stencil(s); }
            get_attrib_location { program, name } => unimplemented!("get_attrib_location"), /*{ gl.get_attrib_location(program, name); }*/
            get_frag_data_location { program, name } => unimplemented!("get_frag_data_location"), /*{ gl.get_frag_data_location(program, name); }*/
            get_uniform_location { program, name } => unimplemented!("get_uniform_location"), /*{ gl.get_uniform_location(program, name); }*/
            get_program_iv {
                program,
                pname,
                result,
            } => unimplemented!("get_program_iv"), /*{ gl.get_program_iv(
                program,
                pname,
                result,
            ); }*/
            uniform_1i { location, v0 } => { gl.uniform_1i(location, v0); }
            uniform_1iv { location, values } => unimplemented!("uniform_1iv"), /*{ gl.uniform_1iv(location, values); }*/
            uniform_1f { location, v0 } => { gl.uniform_1f(location, v0); }
            uniform_1fv { location, values } => unimplemented!("uniform_1fv"), /*{ gl.uniform_1fv(location, values); }*/
            uniform_1ui { location, v0 } => { gl.uniform_1ui(location, v0); }
            uniform_2f { location, v0, v1 } => { gl.uniform_2f(location, v0, v1); }
            uniform_2fv { location, values } => unimplemented!("uniform_2fv"), /*{ gl.uniform_2fv(location, values); }*/
            uniform_2i { location, v0, v1 } => { gl.uniform_2i(location, v0, v1); }
            uniform_2iv { location, values } => unimplemented!("uniform_2iv"), /*{ gl.uniform_2iv(location, values); }*/
            uniform_2ui { location, v0, v1 } => { gl.uniform_2ui(location, v0, v1); }
            uniform_3f {
                location,
                v0,
                v1,
                v2,
            } => { gl.uniform_3f(
                location,
                v0,
                v1,
                v2,
            ); }
            uniform_3fv { location, values } => unimplemented!("uniform_3fv"), /*{ gl.uniform_3fv(location, values); }*/
            uniform_3i {
                location,
                v0,
                v1,
                v2,
            } => { gl.uniform_3i(
                location,
                v0,
                v1,
                v2,
            ); }
            uniform_3iv { location, values } => unimplemented!("uniform_3iv"), /*{ gl.uniform_3iv(location, values); }*/
            uniform_3ui {
                location,
                v0,
                v1,
                v2,
            } => { gl.uniform_3ui(
                location,
                v0,
                v1,
                v2,
            ); }
            uniform_4f {
                location,
                x,
                y,
                z,
                w,
            } => { gl.uniform_4f(
                location,
                x,
                y,
                z,
                w,
            ); }
            uniform_4i {
                location,
                x,
                y,
                z,
                w,
            } => { gl.uniform_4i(
                location,
                x,
                y,
                z,
                w,
            ); }
            uniform_4iv { location, values } => unimplemented!("uniform_4iv"), /*{ gl.uniform_4iv(location, values); }*/
            uniform_4ui {
                location,
                x,
                y,
                z,
                w,
            } => { gl.uniform_4ui(
                location,
                x,
                y,
                z,
                w,
            ); }
            uniform_4fv { location, values } => unimplemented!("uniform_4fv"), /*{ gl.uniform_4fv(location, values); }*/
            uniform_matrix_2fv {
                location,
                transpose,
                value,
            } => unimplemented!("uniform_matrix_2fv"), /*{ gl.uniform_matrix_2fv(
                location,
                transpose,
                value,
            ); }*/
            uniform_matrix_3fv {
                location,
                transpose,
                value,
            } => unimplemented!("uniform_matrix_3fv"), /*{ gl.uniform_matrix_3fv(
                location,
                transpose,
                value,
            ); }*/
            uniform_matrix_4fv {
                location,
                transpose,
                value,
            } => unimplemented!("uniform_matrix_4fv"), /*{ gl.uniform_matrix_4fv(
                location,
                transpose,
                value,
            ); }*/
            depth_range { near, far } => { gl.depth_range(near, far); }
            draw_elements_instanced {
                mode,
                count,
                element_type,
                indices_offset,
                primcount,
            } => { gl.draw_elements_instanced(
                mode,
                count,
                element_type,
                indices_offset,
                primcount,
            ); }
            blend_color { r, g, b, a } => { gl.blend_color(r, g, b, a); }
            blend_func { sfactor, dfactor } => { gl.blend_func(sfactor, dfactor); }
            blend_func_separate {
                src_rgb,
                dest_rgb,
                src_alpha,
                dest_alpha,
            } => { gl.blend_func_separate(
                src_rgb,
                dest_rgb,
                src_alpha,
                dest_alpha,
            ); }
            blend_equation { mode } => { gl.blend_equation(mode); }
            blend_equation_separate {
                mode_rgb,
                mode_alpha,
            } => { gl.blend_equation_separate(
                mode_rgb,
                mode_alpha,
            ); }
            color_mask { r, g, b, a } => { gl.color_mask(r, g, b, a); }
            cull_face { mode } => { gl.cull_face(mode); }
            front_face { mode } => { gl.front_face(mode); }
            depth_func { func } => { gl.depth_func(func); }
            invalidate_framebuffer {
                target,
                attachments,
            } => unimplemented!("invalidate_framebuffer"), /*{ gl.invalidate_framebuffer(
                target,
                attachments,
            ); }*/
            invalidate_sub_framebuffer {
                target,
                attachments,
                xoffset,
                yoffset,
                width,
                height,
            } => unimplemented!("invalidate_sub_framebuffer"), /*{ gl.invalidate_sub_framebuffer(
                target,
                attachments,
                xoffset,
                yoffset,
                width,
                height,
            ); }*/
            read_buffer { mode } => { gl.read_buffer(mode); }
            read_pixels_into_buffer {
                x,
                y,
                width,
                height,
                format,
                pixel_type,
                dst_buffer,
            } => unimplemented!("read_pixels_into_buffer"), /*{ gl.read_pixels_into_buffer(
                x,
                y,
                width,
                height,
                format,
                pixel_type,
                dst_buffer,
            ); }*/
            read_pixels {
                x,
                y,
                width,
                height,
                format,
                pixel_type,
            } => { gl.read_pixels(
                x,
                y,
                width,
                height,
                format,
                pixel_type,
            ); }
            read_pixels_into_pbo {
                x,
                y,
                width,
                height,
                format,
                pixel_type,
            } => { unsafe { gl.read_pixels_into_pbo(
                x,
                y,
                width,
                height,
                format,
                pixel_type,
            ); } }
            sample_coverage { value, invert } => { gl.sample_coverage(value, invert); }
            polygon_offset { factor, units } => { gl.polygon_offset(factor, units); }
            begin_query { target, id } => { gl.begin_query(target, id); }
            end_query { target } => { gl.end_query(target); }
            query_counter { id, target } => { gl.query_counter(id, target); }
            get_query_object_iv {
                id,
                pname,
                returned,
            } => unimplemented!("get_query_object_iv"),
            get_query_object_uiv {
                id,
                pname,
                returned,
            } => unimplemented!("get_query_object_uiv"),
            get_query_object_i64v {
                id,
                pname,
                returned,
            } => unimplemented!("get_query_object_i64v"),
            get_query_object_ui64v {
                id,
                pname,
                returned,
            } => unimplemented!("get_query_object_ui64v"),
            delete_queries { queries } => unimplemented!("delete_queries"), /*{ gl.delete_queries(queries); }*/
            delete_vertex_arrays { vertex_arrays } => unimplemented!("delete_vertex_arrays"), /*{ gl.delete_vertex_arrays(vertex_arrays); }*/
            delete_vertex_arrays_apple { vertex_arrays } => unimplemented!("delete_vertex_arrays_apple"), /*{ gl.delete_vertex_arrays_apple(vertex_arrays); }*/
            delete_buffers { buffers } => unimplemented!("delete_buffers"), /*{ gl.delete_buffers(buffers); }*/
            delete_renderbuffers { renderbuffers } => unimplemented!("delete_renderbuffers"), /*{ gl.delete_renderbuffers(renderbuffers); }*/
            delete_framebuffers { framebuffers } => unimplemented!("delete_framebuffers"), /*{ gl.delete_framebuffers(framebuffers); }*/
            delete_textures { textures } => unimplemented!("delete_textures"), /*{ gl.delete_textures(textures); }*/
            delete_program { program } => { gl.delete_program(program); }
            tex_sub_image_3d_pbo {
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                ty,
                offset,
            } => { gl.tex_sub_image_3d_pbo(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                ty,
                offset,
            ); }
            tex_storage_2d {
                target,
                levels,
                internal_format,
                width,
                height,
            } => { gl.tex_storage_2d(
                target,
                levels,
                internal_format,
                width,
                height,
            ); }
            tex_storage_3d {
                target,
                levels,
                internal_format,
                width,
                height,
                depth,
            } => { gl.tex_storage_3d(
                target,
                levels,
                internal_format,
                width,
                height,
                depth,
            ); }
            get_tex_image_into_buffer {
                target,
                level,
                format,
                ty,
                output,
            } => unimplemented!("get_tex_image_into_buffer"), /*{ gl.get_tex_image_into_buffer(
                target,
                level,
                format,
                ty,
                output,
            ); }*/
            copy_image_sub_data {
                src_name,
                src_target,
                src_level,
                src_x,
                src_y,
                src_z,
                dst_name,
                dst_target,
                dst_level,
                dst_x,
                dst_y,
                dst_z,
                src_width,
                src_height,
                src_depth,
            } => {
                unsafe {
                    gl.copy_image_sub_data(
                        src_name,
                        src_target,
                        src_level,
                        src_x,
                        src_y,
                        src_z,
                        dst_name,
                        dst_target,
                        dst_level,
                        dst_x,
                        dst_y,
                        dst_z,
                        src_width,
                        src_height,
                        src_depth,
                    );
                }
            }
            generate_mipmap { target } => { gl.generate_mipmap(target); }
        }
    }
}
