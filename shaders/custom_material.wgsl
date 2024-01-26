#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_bindings   mesh

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<vec3<f32>>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    return material.color * textureSample(base_color_texture, base_color_sampler, mesh.uv);
}



@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    return out;
}