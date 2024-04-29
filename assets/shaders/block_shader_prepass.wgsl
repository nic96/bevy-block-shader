#import bevy_pbr::{
    prepass_io::FragmentOutput,
    pbr_deferred_functions::deferred_output,
    mesh_functions,
}
#ifdef DEFERRED_PREPASS
#import bevy_pbr::rgb9e5
#endif

struct MyVertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) texture_layer: u32,
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    @location(7) instance_index: u32,
#endif
};

fn x_positive_bits(bits: u32) -> u32{
    return (1u << bits) - 1u;
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) vert_data: vec4<u32>,
};

@vertex
fn vertex(vertex: Vertex) -> MyVertexOutput {
    var out: MyVertexOutput;

    let x = f32(vertex.vert_data[0] & x_positive_bits(6u));
    let y = f32(vertex.vert_data[0] >> 6u & x_positive_bits(6u));
    let z = f32(vertex.vert_data[0] >> 12 & x_positive_bits(6u));

    let normal_x = f32(vertex.vert_data[1] & x_positive_bits(6u)) / 16. - 1.;
    let normal_y = f32(vertex.vert_data[1] >> 6u & x_positive_bits(6u)) / 16. - 1.;
    let normal_z = f32(vertex.vert_data[1] >> 12u & x_positive_bits(6u)) / 16. - 1.;

    let local_position = vec4(x,y,z, 1.0);
    let normal = vec3(normal_x, normal_y, normal_z);
    out.texture_layer = (vertex.vert_data[1] >> 18u);

    out.world_normal = mesh_functions::mesh_normal_local_to_world(normal, vertex.instance_index);

    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    var model = mesh_functions::get_model_matrix(vertex.instance_index);

    let world_position = model * local_position;
    out.position = mesh_functions::mesh_position_local_to_clip(model, local_position);

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index = vertex.instance_index;
#endif
    return out;
}

#ifdef PREPASS_FRAGMENT
@fragment
fn fragment(in: MyVertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

#ifdef NORMAL_PREPASS
    out.normal = vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);
#endif

#ifdef DEPTH_CLAMP_ORTHO
    out.frag_depth = in.position.z;
#endif // DEPTH_CLAMP_ORTHO

#ifdef DEFERRED_PREPASS
    // There isn't any material info available for this default prepass shader so we are just writing
    // emissive magenta out to the deferred gbuffer to be rendered by the first deferred lighting pass layer.
    // This is here so if the default prepass fragment is used for deferred magenta will be rendered, and also
    // as an example to show that a user could write to the deferred gbuffer if they were to start from this shader.
    out.deferred = vec4(0u, rgb9e5::vec3_to_rgb9e5_(vec3(1.0, 0.0, 1.0)), 0u, 0u);
    out.deferred_lighting_pass_id = 1u;
#endif

    return out;
}
#endif // PREPASS_FRAGMENT
