// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coods: vec2<f32>,
}

struct VertexOutput {
    // analogous to GLSL's gl_Position variable
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coods: vec2<f32>,
};

@vertex
fn vs_main (
    model: VertexInput,
) -> VertexOutput {
    // var is mut and must always specify the type
    // let is const but can infer its type
    var out: VertexOutput;
    out.tex_coods = model.tex_coods;
    out.clip_position = vec4<f32>(model.position, 1.0);

    return out;
}

@group(0)@binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coods);
}
