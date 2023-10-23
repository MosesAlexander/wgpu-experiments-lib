// Vertex shader

struct VertexOutput {
    // analogous to GLSL's gl_Position variable
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};

@vertex
fn vs_main (
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    // var is mut and must always specify the type
    // let is const but can infer its type
    var out: VertexOutput;

    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;

    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3,0.5,0.1,1.0);
}

@fragment
fn fs2_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.vert_pos.x,in.vert_pos.y,0.5,1.0);
}