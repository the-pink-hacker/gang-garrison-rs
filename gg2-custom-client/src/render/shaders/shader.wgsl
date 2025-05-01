// Fragment shader
@group(0) @binding(0)
var texture_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var sampler_diffuse: sampler;

// Vertex shader
struct CameraUniform {
    view_projection: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_uv: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = camera.view_projection * vec4<f32>(model.position, 1.0);
    out.texture_uv = vec2<f32>(model.position.x, 1.0 - model.position.y);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_diffuse, sampler_diffuse, in.texture_uv);
}
