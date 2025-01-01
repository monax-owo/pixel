#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(0.5)
}