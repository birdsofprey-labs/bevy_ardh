#import bevy_pbr::mesh_functions;
#import bevy_render::instance_index::get_instance_index
#import bevy_pbr::{
     skinning,
    morph::morph,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    forward_io::{Vertex},
    view_transformations::position_world_to_clip
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct MyExtendedMaterial {
    custom_uv: vec4<f32>,
}

@group(1) @binding(100)
var<uniform> my_extended_material: MyExtendedMaterial;
@group(1) @binding(101)
var height_texture: texture_2d<f32>;
@group(1) @binding(102)
var height_sampler: sampler;

@group(1) @binding(103)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(104)
var base_color_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // we can optionally modify the input before lighting and alpha_discard is applied
    pbr_input.material.base_color.b = pbr_input.material.base_color.r;

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // we can optionally modify the lit color before post-processing is applied
   // out.color = vec4<f32>(vec4<u32>(out.color * f32(my_extended_material.quantize_steps))) / f32(my_extended_material.quantize_steps);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
    //out.color = out.color * 2.0;
    
#endif
    
    // out.color = vec4<f32>(in.uv.x, in.uv.y, 0.0, 1.0);

    return out;
}

fn scaleLinear(value: f32, valueDomain: vec2<f32> ) -> f32 {
  return (value - valueDomain.x) / (valueDomain.y - valueDomain.x);
}

fn sl(value: f32, valueDomain: vec2<f32>, valueRange: vec2<f32>) -> f32 {
  return mix(valueRange.x, valueRange.y, scaleLinear(value, valueDomain));
}

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {

    
     let ds = 0.5 * 1.0 / 256.0;

    var out: VertexOutput;

   

#ifdef MORPH_TARGETS
    var vertex = morph_vertex(vertex_no_morph);
#else
    var vertex = vertex_no_morph;
#endif
   

#ifdef SKINNED
    var model = skinning::skin_model(vertex.joint_indices, vertex.joint_weights);
#else
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    var model = mesh_functions::get_model_matrix(vertex_no_morph.instance_index);
#endif

 let sp = vec3<f32>(vertex.position.x, vertex.position.y, vertex.position.z);
    let vertex_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(sp, 1.0)).xyz ;
 let sphere_norm = normalize(vertex_position.xyz);
   
var vertex_normal = vec3<f32>(vertex.normal.x, -vertex.normal.y, vertex.normal.z);
 var max_height = 100.0;
    var base_height = 2000.0;
    let vn = mesh_functions::mesh_normal_local_to_world(vertex_normal, vertex_no_morph.instance_index).xyz ;
    if dot(normalize(vertex_position), vn) > 0.0 {
       base_height = 1700.0;//1700.0;
       //hbase
       vertex_normal = -vertex_normal;
    }
//vertex_normal = normalize(vertex_normal);
#ifdef VERTEX_NORMALS
#ifdef SKINNED
    out.world_normal = skinning::skin_normals(model, vertex_normal);
#else
    out.world_normal = vertex_normal;
#endif
#endif

#ifdef VERTEX_POSITIONS
    //let sp = vec3<f32>(vertex.position.x, vertex.position.y, vertex.position.z);
    //let vertex_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(sp, 1.0)).xyz ;//+ vec3<f32>(0.0, 500.0*2.0, 0.0);
    let hbase = normalize(vertex_position);
    //let fuv = vertex.uv;
    //let height: f32  = textureGather(1, height_texture, height_sampler, vertex.uv).r;
    //let vuv = vec2<f32>(fuv.u, fuv.v);
    //let cuv = my_extended_material.custom_uv.xy;
    //let cscale = my_extended_material.custom_uv.z / 2.0;

    //let final_uv =  vertex.uv;//mix(vertex.uv, cuv, vertex.uv);

    //let fuv = convert_xyz_to_cube_uv2( hbase.x , hbase.y, hbase.z , vertex.uv );
    //let vuv = vec2<f32>(fuv.x, fuv.y);
     //vertex.uv = vuv;// vertex.uv ;//+ vec2<f32>(0.5, 0.5) ;

  
    //let height: f32  = textureGather(1, height_texture, height_sampler,  vec2<f32>(  scaleLinear(vertex.uv.x, vec2<f32>(ds, 1.0 - ds)),  scaleLinear(vertex.uv.y, vec2<f32>(ds, 1.0 - ds))  )).r;
    let height: f32  = textureGather(1, height_texture, height_sampler,  vertex.uv.xy  ).r;
     // vertex.uv = vec2<f32>(0.5*(-0.5 + vertex.uv.x * 2.0), 0.5*(-0.5 +  vertex.uv.y * 2.0));
   
   let base = hbase * (base_height + height * max_height);
   //let fixer = 1.0;
//    let base_fixed =  vec3<f32>(vec3<i32>(base / fixer)) * fixer;
//    if length(base_fixed-base) < 1.0 {
//         vertex.position = base_fixed;
//    } else {
//         vertex.position = base;
//    }
    // vec3<f32>(vec3<i32>(base / fixer)) * fixer;//(model * (vec4<f32>(vertex.position.xyz, 0.0))).xyz  + ;
    //let vertex_position = vertex.position;
     vertex.position = base;
    out.world_position = vec4<f32>(vertex.position, 1.0);//mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    //out.world_position = out.world_position + vec4<f32>(0.0,height*40.0 - 7.0,0.0, 0.0);
    out.position = position_world_to_clip(out.world_position.xyz);
#endif

#ifdef VERTEX_UVS
    //out.uv = vec2<f32>(  scaleLinear(vertex.uv.x, vec2<f32>(ds, 1.0 - ds)),  scaleLinear(vertex.uv.y, vec2<f32>(ds, 1.0 - ds))  );
    out.uv = vertex.uv.xy;
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        model,
        vertex.tangent,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        get_instance_index(vertex_no_morph.instance_index)
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    out.instance_index = get_instance_index(vertex_no_morph.instance_index);
#endif

    return out;
}

struct FaceUV { f:i32, u:f32, v:f32 }
fn convert_xyz_to_cube_uv(x: f32, y: f32, z: f32) -> FaceUV {
    let absX = abs(x);
    let absY = abs(y);
    let absZ = abs(z);

    let isXPositive = x > 0.0;
    let isYPositive = y > 0.0;
    let isZPositive = z > 0.0;

    var maxAxis: f32;
    var uc: f32;
    var vc: f32;
    var index: i32;

    // POSITIVE X
    if (isXPositive && absX >= absY && absX >= absZ) {
        // u (0 to 1) goes from +z to -z
        // v (0 to 1) goes from -y to +y
        maxAxis = absX;
        uc = -z;
        vc = y;
        index = 0;
    }
    // NEGATIVE X
    if (!isXPositive  && absX >= absY && absX >= absZ) {
        // u (0 to 1) goes from -z to +z
        // v (0 to 1) goes from -y to +y
        maxAxis = absX;
        uc = z;
        vc = y;
        index = 1;
    }
    // POSITIVE Y
    if (isYPositive && absY >= absX && absY >= absZ) {
        // u (0 to 1) goes from -x to +x
        // v (0 to 1) goes from +z to -z
        maxAxis = absY;
        uc = x;
        vc = -z;
        index = 2;
    }
    // NEGATIVE Y
    if (!isYPositive && absY >= absX && absY >= absZ) {
        // u (0 to 1) goes from -x to +x
        // v (0 to 1) goes from -z to +z
        maxAxis = absY;
        uc = x;
        vc = z;
        index = 3;
    }
    // POSITIVE Z
    if (isZPositive && absZ >= absX && absZ >= absY) {
        // u (0 to 1) goes from -x to +x
        // v (0 to 1) goes from -y to +y
        maxAxis = absZ;
        uc = x;
        vc = y;
        index = 4;
    }
    // NEGATIVE Z
    if (isZPositive && absZ >= absX && absZ >= absY) {
        // u (0 to 1) goes from +x to -x
        // v (0 to 1) goes from -y to +y
        maxAxis = absZ;
        uc = -x;
        vc = y;
        index = 5;
    }

    // Convert range from -1 to 1 to 0 to 1
    let u = 0.5 * (uc / maxAxis + 1.0);
    let v = 0.5 * (vc / maxAxis + 1.0);

    return FaceUV(index, u,  v);
}

fn convert_xyz_to_cube_uv2(x: f32, y: f32, z: f32, uv: vec2<f32>) -> vec2<f32> {
    let absX = abs(x);
    let absY = abs(y);
    let absZ = abs(z);

    var uv2 = uv;

    let isXPositive = x > 0.0;
    let isYPositive = y > 0.0;
    let isZPositive = z > 0.0;

    var maxAxis: f32;
    var uc: f32;
    var vc: f32;
    var index: i32;

    // POSITIVE X
    if (isXPositive && absX >= absY && absX >= absZ) {
        // u (0 to 1) goes from +z to -z
        // v (0 to 1) goes from -y to +y
        maxAxis = absX;
        let u =  uv.x;
        let v =  uv.y;
        index = 0;



        uv2 = vec2<f32>(  1.0 - u, 1.0 - v );
    }
    // NEGATIVE X
    if (!isXPositive  && absX >= absY && absX >= absZ) {
        // u (0 to 1) goes from -z to +z
        // v (0 to 1) goes from -y to +y
        maxAxis = absX;
        uc = z;
        vc = y;
        index = 1;

        uv2 = uv;
    }
    // POSITIVE Y
    if (isYPositive && absY >= absX && absY >= absZ) {
        let u =  uv.x;
        let v = uv.y;
        index = 2;



        uv2 = vec2<f32>( u, v );
    }
    // NEGATIVE Y
    if (!isYPositive && absY >= absX && absY >= absZ) {
        // u (0 to 1) goes from -x to +x
        // v (0 to 1) goes from -z to +z
        maxAxis = absY;
        uc = x;
        vc = z;
        index = 3;
    }
    // POSITIVE Z
    if (isZPositive && absZ >= absX && absZ >= absY) {
        // u (0 to 1) goes from -x to +x
        // v (0 to 1) goes from -y to +y
        maxAxis = absZ;
        uc = x;
        vc = y;
        index = 4;
    }
    // NEGATIVE Z
    if (isZPositive && absZ >= absX && absZ >= absY) {
        // u (0 to 1) goes from +x to -x
        // v (0 to 1) goes from -y to +y
        maxAxis = absZ;
        uc = -x;
        vc = y;
        index = 5;
    }

    // Convert range from -1 to 1 to 0 to 1
    let u = 0.5 * (uc / maxAxis + 1.0);
    let v = 0.5 * (vc / maxAxis + 1.0);

    return vec2<f32>( uv2.x, uv2.y );
}

