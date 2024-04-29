#import bevy_pbr::{
    pbr_fragment::pbr_input_from_vertex_output,
    pbr_functions,
    mesh_functions,
    pbr_types,
    forward_io::FragmentOutput,
}

struct MyVertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(3) texture_layer: u32,
    @location(4) uv: vec2<f32>,
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    @location(6) @interpolate(flat) instance_index: u32,
#endif
};

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) vert_data: vec4<u32>,
};

fn x_positive_bits(bits: u32) -> u32 {
    return (1u << bits) - 1u;
}

@vertex
fn vertex(vertex: Vertex) -> MyVertexOutput {
    var out: MyVertexOutput;

    let x = f32(vertex.vert_data[0] & x_positive_bits(6u));
    let y = f32(vertex.vert_data[0] >> 6u & x_positive_bits(6u));
    let z = f32(vertex.vert_data[0] >> 12 & x_positive_bits(6u));

    let normal_x = f32(vertex.vert_data[1] & x_positive_bits(6u)) / 16. - 1.;
    let normal_y = f32(vertex.vert_data[1] >> 6u & x_positive_bits(6u)) / 16. - 1.;
    let normal_z = f32(vertex.vert_data[1] >> 12u & x_positive_bits(6u)) / 16. - 1.;

    let u = f32(vertex.vert_data[2] & x_positive_bits(11u)) / 32.;
    let v = f32(vertex.vert_data[2] >> 11u & x_positive_bits(11u)) / 32.;

    let local_position = vec4(x,y,z, 1.0);
    let normal = vec3(normal_x, normal_y, normal_z);
    out.texture_layer = (vertex.vert_data[1] >> 18u);
    out.uv = vec2(u, v);

    let world_position = mesh_functions::get_model_matrix(vertex.instance_index) * local_position;
    out.position = mesh_functions::mesh_position_local_to_clip(
        mesh_functions::get_model_matrix(vertex.instance_index),
        local_position,
    );
    out.world_position = world_position;
    out.world_normal = mesh_functions::mesh_normal_local_to_world(normal, vertex.instance_index);

    out.instance_index = vertex.instance_index;
    return out;
}

//@group(2) @binding(100)
//var<uniform> block_material: BlockMaterial;

@group(2) @binding(101)
var base_color_array_texture: texture_2d_array<f32>;
@group(2) @binding(102)
var base_color_array_sampler: sampler;

@group(2) @binding(103)
var emissive_array_texture: texture_2d_array<f32>;
@group(2) @binding(104)
var emissive_array_sampler: sampler;

@group(2) @binding(105)
var metallic_roughness_array_texture: texture_2d_array<f32>;
@group(2) @binding(106)
var metallic_roughness_array_sampler: sampler;

@fragment
fn fragment(
    in: MyVertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_my_vertex_output(in, is_front);

    pbr_input.material.base_color = textureSample(base_color_array_texture, base_color_array_sampler, in.uv, in.texture_layer);
    let metallic_roughness = textureSample(metallic_roughness_array_texture, metallic_roughness_array_sampler, in.uv, in.texture_layer);
    // Sampling from GLTF standard channels for now
    pbr_input.material.metallic = metallic_roughness.b;
    pbr_input.material.perceptual_roughness = metallic_roughness.g;
    pbr_input.material.emissive = textureSample(emissive_array_texture, emissive_array_sampler, in.uv, in.texture_layer);

    var out: FragmentOutput;
    // apply lighting
    out.color = pbr_functions::apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = pbr_functions::main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}

fn pbr_input_from_my_vertex_output(in: MyVertexOutput, is_front: bool) -> pbr_types::PbrInput {
    let bevy_in = bevy_pbr::forward_io::VertexOutput(
        // This is `clip position` when the struct is used as a vertex stage output
        // and `frag coord` when used as a fragment stage input
        in.position,
        in.world_position,
        in.world_normal,
#ifdef VERTEX_UVS
        in.uv,
#endif
#ifdef VERTEX_TANGENTS
        in.world_tangent,
#endif
#ifdef VERTEX_COLORS
        in.color,
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
        in.instance_index,
#endif
    );

    return pbr_input_from_vertex_output(bevy_in, is_front, false);
}

