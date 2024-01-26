use std::{collections::VecDeque, hash::Hash};

use ardh::{camera_controller, ardh::{ArdhFlat, QT, TileId}, quadtree::ZNodeIndex};
use bevy::{prelude::*, utils::HashSet, render::{RenderPlugin, settings::{WgpuSettings, WgpuFeatures, RenderCreation}, render_resource::{AsBindGroup, ShaderRef, TextureDescriptor, AddressMode, SamplerDescriptor}, primitives::Aabb, texture::{ImageSampler, ImageSamplerDescriptor, ImageAddressMode, ImageLoaderSettings}}, pbr::{wireframe::{WireframePlugin, WireframeConfig, WireframeColor, Wireframe}, ExtendedMaterial, MaterialExtension, OpaqueRendererMethod, ScreenSpaceAmbientOcclusionBundle, CascadeShadowConfigBuilder}, reflect::{TypeUuid, TypePath}, core_pipeline::experimental::taa::TemporalAntiAliasPlugin, ecs::query::WorldQuery};
//use bevy_infinite_grid::{GridShadowCamera, InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use camera_controller::{CameraController, CameraControllerPlugin};

const PLANET_SIZE: f32 = 999.9;
fn main() {
    App::new()
    .insert_resource(Msaa::Sample4)
    //.add_plugins((
      //  DefaultPlugins//, TemporalAntiAliasPlugin
        // DefaultPlugins.set(RenderPlugin {
        //     wgpu_settings: WgpuSettings {
        //         features: WgpuFeatures::POLYGON_MODE_LINE,
        //         ..default()
        //     },
        // }),
        //WireframePlugin,
    //))
    
    .add_plugins(
        DefaultPlugins
        //   .set(ImagePlugin {
        //     default_sampler: ImageSamplerDescriptor {
        //       address_mode_u: ImageAddressMode::ClampToEdge,
        //       address_mode_v: ImageAddressMode::ClampToEdge,
        //       address_mode_w: ImageAddressMode::ClampToEdge,
        //       ..Default::default()
        //     },
        //   })
      )
    .insert_resource(WireframeConfig {
        // The global wireframe config enables drawing of wireframes on every mesh,
        // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
        // regardless of the global configuration.
        global: false,
        // Controls the default color of all wireframes. Used as the default color for global wireframes.
        // Can be changed per mesh using the `WireframeColor` component.
        default_color: Color::WHITE,
    })
    .add_plugins(MaterialPlugin::<
        ExtendedMaterial<StandardMaterial, MyExtension>,
    >::default())
    //.add_plugins((MaterialPlugin::<CustomMaterial>::default()))
        .add_plugins((CameraControllerPlugin))//, InfiniteGridPlugin))
        .add_systems(Startup, setup_system)
        .add_systems(FixedUpdate, sdiver)
        .add_systems(FixedUpdate, sdiver2)
        .add_systems(Update, max_depth_sys)
        //.insert_resource(FixedTime::new_from_secs(0.001))
        .insert_resource(MaxDepth(2, 0))
        .run();
}

#[derive(Resource)]
struct TileMesh(Handle<Mesh>, Aabb);


/// Compute the Axis-Aligned Bounding Box of the mesh vertices in model space
pub fn compute_aabb( tx: &Transform) -> bevy::render::primitives::Aabb {
   let pmin = tx.transform_point(Vec3::ZERO) + Vec3::splat(-PLANET_SIZE*4.0);
   let pmax = tx.transform_point(Vec3::ZERO) + Vec3::splat( PLANET_SIZE*4.0);

    bevy::render::primitives::Aabb::from_min_max(pmin, pmax)
//    bevy::render::primitives::Aabb::enclosing(values.iter().map(|p| Vec3::from_slice(p)*4.0 + Vec3::new(0.0,200.0,0.0)))
}



    

fn setup_system(
    //mut wireframe_config: ResMut<WireframeConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    //mut materials: ResMut<Assets<CustomMaterial>>,
) {


    commands.insert_resource(ClearColor(Color::rgb(0.4627450980392157, 0.6352941176470588, 0.9098039215686274)));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.03,
    });

    let pmesh = (Mesh::from(shape::Plane { size: PLANET_SIZE*4.0, subdivisions: 128*1 }))
        //.unwrap()
        .with_generated_tangents()
        .unwrap();
    
    let pushto = 2000.0;
    let paabb = compute_aabb( &Transform::default().with_translation(-Vec3::Y * pushto));
    let mesh = meshes.add(pmesh);

    commands.insert_resource(TileMesh(mesh,paabb)); 

    
   // wireframe_config.global = false;
   let sides_txes = [

//    Transform::from_rotation(Quat::from_axis_angle(Vec3::Z,  90f32.to_radians()) )
//    .mul_transform(
//         Transform::default().with_translation(Vec3::X * pushto).with_scale(Vec3::new(1.0, 1.0, 1.0))
//     ), // +y
   
Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  90f32.to_radians()) ) *
Transform::from_rotation(Quat::from_axis_angle(-Vec3::Z,  90f32.to_radians()) ) *
Transform::default().with_translation(Vec3::Y * pushto),


Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  90f32.to_radians()) ) *
Transform::from_rotation(Quat::from_axis_angle(-Vec3::Z,  -90f32.to_radians()) ) *
Transform::default().with_translation(Vec3::Y * pushto),
    
    Transform::default().with_translation(Vec3::Y * pushto)//.with_scale(Vec3::new(1.0, -1.0, 1.0))
        * (  Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  0f32.to_radians()) )), // +y
   

// -Y
Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  180f32.to_radians()) ) *
Transform::default().with_translation(Vec3::Y * pushto),

// +Z
//
Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  90f32.to_radians()) ) *
Transform::default().with_translation(Vec3::Y * pushto),

// -Z
(  Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  -90f32.to_radians()) )) *
Transform::default().with_translation(Vec3::Y * pushto) * 
Transform::from_rotation(Quat::from_axis_angle(Vec3::Y,  180f32.to_radians()) )
 ];

let sides_txes_lod = [

//    Transform::from_rotation(Quat::from_axis_angle(Vec3::Z,  90f32.to_radians()) )
//    .mul_transform(
//         Transform::default().with_translation(Vec3::X * pushto).with_scale(Vec3::new(1.0, 1.0, 1.0))
//     ), // +y
   
Transform::from_rotation(Quat::from_axis_angle(-Vec3::Z,  90f32.to_radians()) ) *
Transform::default().with_translation(Vec3::Y * pushto),


Transform::from_rotation(Quat::from_axis_angle(-Vec3::Z,  -90f32.to_radians()) ) *
Transform::default().with_translation(Vec3::Y * pushto),
    
    Transform::default().with_translation(Vec3::Y * pushto),
   

// -Y
Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  180f32.to_radians()) ) *
Transform::default().with_translation(Vec3::Y * pushto),

// +Z
//
Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  90f32.to_radians()) ) *
Transform::default().with_translation(Vec3::Y * pushto),

// -Z
(  Transform::from_rotation(Quat::from_axis_angle(Vec3::X,  -90f32.to_radians()) )) *
Transform::default().with_translation(Vec3::Y * pushto) * 
Transform::from_rotation(Quat::from_axis_angle(Vec3::Y,  180f32.to_radians()) )
 ];

   for (face_idx,(&local_tx, lod_tx)) in sides_txes.iter().zip(sides_txes_lod).enumerate() {

    // if face_idx == 0 {
    //     face_idx = 5;
    // }
    
    let mut tree = ardh::quadtree::QuadTree::new();
    let root = ardh::ardh::Node {
        face: face_idx,
        parent_copy: None,
        id: 0,
        depth: 0,
        tx: Transform::default(),//.with_translation(Vec3::Y * pushto),
        index: ZNodeIndex::None,
        uv_offset:  Vec2::new(0.0, 0.0), 
        uv_scale: 1.0,
        size: PLANET_SIZE
    };
    tree.set_root(Some(root.clone()));
    // mark the source vertex as discovered
    let mut discovered = HashSet::new();
    let mut stack = VecDeque::new();

    discovered.insert(root.clone());
    stack.push_back(root);

   
 
    commands.spawn(ArdhFlat {
        face: face_idx,
        local_tx: local_tx,// Transform::default().with_translation(Vec3::Y * pushto),//Transform::default(),
        lod_tx: lod_tx,
        size: PLANET_SIZE,
        stree: ardh::ardh::SearchTree { tree , 
                        running: true, 
                        stack,
                        leafs: HashSet::new(),
                        leafs_prev: HashSet::new(),
                         discovered },
    })
    //insert(TransformBundle::default())
    .insert(Transform::default())//.with_translation(Vec3::Y * pushto))
    .insert(GlobalTransform::default())
    .insert(VisibilityBundle::default())
    .insert(BatchAssets::default());
   }
    

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, pushto+500.0, 0.0),
            ..default()
        },
        CameraController::default(),
        FogSettings {
            color: Color::rgba(0.25, 0.25, 0.35, 1.0),
            falloff:FogFalloff::from_visibility_color(3500.0, Color::WHITE), //FogFalloff::Atmospheric { extinction: Vec3::splat(0.0075), inscattering: Vec3::splat(0.01) },
            ..default()
        },
       // GridShadowCamera,
       //paceAmbientOcclusionBundle::default()
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 85000.0,
            shadows_enabled: true,
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        transform: Transform::from_translation(Vec3::Y * 100. + Vec3::X * 50. + Vec3::Z * 20.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         illuminance: 35000.0,
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     // The default cascade config is designed to handle large scenes.
    //     // As this example has a much smaller world, we can tighten the shadow
    //     // bounds for better visual quality.
        
    //     transform: Transform::from_translation(Vec3::X * 0. + Vec3::Y * 90.)
    //         .looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });

    let mat = standard_materials.add(StandardMaterial::default());

   

    // cube
    commands.spawn(PbrBundle {
        material: mat.clone(),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform {
            translation: Vec3::new(3., 4., 0.),
            rotation: Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
            scale: Vec3::splat(1.5),
        },
        ..default()
    });

    commands.spawn(PbrBundle {
        material: mat.clone(),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "This text changes in the bottom right",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Count",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
        ]),
        TextChanges,
    ));
}

fn sdiver(mut commands: Commands, 
    //mut tile_images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    //mut standard_materials: ResMut<Assets<StandardMaterial>>,
    //mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
   // mut materials: ResMut<Assets<CustomMaterial>>,
    //tile_mesh: Res<TileMesh>,
    mut qtext: Query<&mut Text, With<TextChanges>>,
    //mut meshes: ResMut<Assets<Mesh>>, 
    md: Res<MaxDepth>, 
    mut qry: Query<(&mut ArdhFlat, &mut BatchAssets, Entity)>, 
    //tileq: Query<(Entity, &TileId)>, 
    qcam: Query<&Transform, With<Camera>>
) {

    // let sampler_desc = ImageSamplerDescriptor {
    //     address_mode_u: ImageAddressMode::Repeat,
    //     address_mode_v: ImageAddressMode::Repeat,
    //     ..Default::default()
    // };

    // let settings = move |s: &mut ImageLoaderSettings| {
    //     s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    // };
    let cam_tx = qcam.single();

    //let test_tex = asset_server.load("testmap.png");
    //let test_hgt = asset_server.load("hgt0.png");
    // let mat = standard_materials.add(StandardMaterial {
    //     base_color: Color::WHITE,
    //     base_color_texture: Some(test_tex),
    //     ..Default::default()
    // });

    

    for mut text in &mut qtext {
        
       

        text.sections[0].value = format!(
            "Max Depth: {}", md.0,
        );

        text.sections[1].value = format!(
            " Tiles  {}", md.1,
        );


    }
    for (mut q, mut qbatch_assets, pnute) in qry.iter_mut() {

        if qbatch_assets.queue.len()> 0 {
            //return;
            println!("running dfs iter {} = {}", q.face, qbatch_assets.queue.len());
            return;;
        }
        //println!("running dfs iter");
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut xqloop = true;
        
        let local_tx = q.local_tx.clone();
        let lod_tx = q.lod_tx;
        while xqloop {
        let qloop = q.stree.dfs(|node| {

           if (node.depth as i32) == 0 { return true }
            
           // return (node.depth as i32) < md.0 ;
            if (node.depth as i32) >= md.0 { return false}
            let mut nodt = (lod_tx.mul_transform(node.tx)).translation.normalize() * 2000.0;
            let camt = (cam_tx).translation;
            nodt *= Vec3::new(1.0, 1.0, 1.0);
            let boxy_dist =  camt.distance(nodt );
            let cliper =  0.0 + 1.0*( 10000000.0/boxy_dist.powf(2.0) ).log2() ;
            
            //println!("dist {} {}", node.depth, cliper);
            if node.depth < cliper as usize  {
                return true;
            }
             return false;
        });
//        println!("STACK {}", q.stree.stack.len());
        if  q.stree.stack.len() == 0 && q.stree.leafs.len() > 0 {

        let add_list = { q.stree.leafs
            .difference( &q.stree.leafs_prev ).map(|x| x.clone()).collect::<HashSet::<_>>() };



        let del_list = { q.stree.leafs_prev
            .difference( &q.stree.leafs ).map(|x| x.clone()).collect::<HashSet::<_>>() };
        q.stree.leafs_prev = q.stree.leafs.clone();
        //println!("SUM {} ADD {} REM {}",q.stree.leafs.len(),  add_list.len(), del_list.len());
        q.stree.leafs.clear();
        
        let mut batch_assets: BatchAsset = BatchAsset::default();

        for add_node in add_list {

            

            let base_color = Color::hsl(rng.gen::<f32>()*360.0, 0.2+(rng.gen::<f32>()) * 0.5, 0.2+rng.gen::<f32>() * 0.5);
            let test_hgt = asset_server.load(format!("tiles/hgt_{}_{}.exr", add_node.face, add_node.id));
            let test_nor = asset_server.load(format!("tiles/nor_{}_{}.png", add_node.face, add_node.id));
            let test_col = asset_server.load(format!("tiles/col_{}_{}.png", add_node.face, add_node.id));
            batch_assets.tiles.push(add_node);
            batch_assets.hgt_images.push(test_hgt);
            batch_assets.nor_images.push(test_nor);
            batch_assets.col_images.push(test_col);

        
        }

       
        let del_ids =  del_list.iter().map(|n|TileId {address: n.id, face: n.face }).collect::<HashSet<_>>();
        for del_id in del_ids.iter() {
           // if del_ids.contains( &del_node.address ) {
                //commands.entity(del_ent).despawn();
                batch_assets.tiles_removal.push(del_id.clone());
           // }
        }
        qbatch_assets.queue.push_back(batch_assets);
    }
    xqloop = q.stree.stack.len() > 0;
    if qloop { break; }
    } }
    
}

#[derive(Debug, Resource)]
struct MaxDepth(i32, usize);

fn max_depth_sys(
    mut md: ResMut<MaxDepth>, 
                key_input: Res<Input<KeyCode>>) {
        if key_input.just_pressed(KeyCode::NumpadAdd) {
            md.0 += 1;
            println!("MXD: {:?}", md);
        }
        else if key_input.just_pressed(KeyCode::NumpadSubtract) {
            md.0 -= 1;
            println!("MXD: {:?}", md);
        }
        //md.1 = world.entities().len() as usize;
}


#[derive(Component)]
struct TextChanges;


// This is the struct that will be passed to your shader
// #[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
// #[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
// pub struct CustomMaterial {
//     #[uniform(0)]
//     color: Color,
//     #[texture(1)]
//     #[sampler(2)]
//     color_texture: Option<Handle<Image>>,
// }

// impl Material for CustomMaterial {
//     fn vertex_shader() -> ShaderRef {
//         "shaders/custom_material.wgsl".into()
//     }

//     fn fragment_shader() -> ShaderRef {
//         "shaders/custom_material.wgsl".into()
//     }

// }
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
struct MyExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    custom_uv: Vec4,
    #[texture(101)]
    #[sampler(102)] 
    height_texture: Option<Handle<Image>>,
 //   #[texture(103)]
 //   #[sampler(104)]
 //   base_color_texture: Option<Handle<Image>>,
}

impl MaterialExtension for MyExtension {
    // fn prepass_vertex_shader() -> ShaderRef {
    //     "shaders/extended_material.wgsl".into()
    // }

    fn vertex_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

   
    // fn deferred_vertex_shader() -> ShaderRef {
    //     "shaders/extended_material.wgsl".into()
    // }

    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    // fn deferred_fragment_shader() -> ShaderRef {
    //     "shaders/extended_material.wgsl".into()
    // }
}


#[derive(Component, Default)]
struct BatchAssets {
    pub queue: VecDeque<BatchAsset>,
}

#[derive(Component, Default)]
struct BatchAsset {
    pub hgt_images: Vec<Handle<Image>>,
    pub nor_images: Vec<Handle<Image>>,
    pub col_images: Vec<Handle<Image>>,
    pub tiles: Vec<ardh::ardh::Node>,
    pub tiles_removal: Vec<TileId>,
}


fn sdiver2(mut commands: Commands, 
    asset_images: Res<Assets<Image>>,
    //asset_server: Res<AssetServer>,
    //mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
   // mut materials: ResMut<Assets<CustomMaterial>>,
    tile_mesh: Res<TileMesh>,
    //qtext: Query<&mut Text, With<TextChanges>>,
    //meshes: Res<Assets<Mesh>>,
    mut md: ResMut<MaxDepth>, 
      mut qry: Query<(&ArdhFlat, &mut BatchAssets, Entity)>, tileq: Query<(Entity, &TileId)>, qcam: Query<&Transform, With<Camera>>) {
        let mut tcount = 0usize;
        use rand::Rng;
        let mut rng = rand::thread_rng();
       
    
    
    'outerl: for (q, mut qbatch_assets, pnute) in qry.iter_mut()
    {
        //println!("FACE {}", q.face);
        if let Some(batch) = qbatch_assets.queue.front()
        //for batch in qbatch_assets.queue.iter_mut().take(1)
        {
            for ihght in batch.hgt_images.iter() {
                if asset_images.get(ihght).is_none() {
                    continue 'outerl
                }
            }
            for ihght in batch.nor_images.iter() {
                if asset_images.get(ihght).is_none() {
                    continue 'outerl
                }
            }
            for ihght in batch.col_images.iter() {
                if asset_images.get(ihght).is_none() {
                    continue 'outerl
                }
            }

            for ((test_hgt, test_nor), (add_node, test_col)) in  batch.hgt_images.iter().zip(batch.nor_images.iter()).zip(batch.tiles.iter().zip(batch.col_images.iter())) {
                // let pushto = 2000.0;
                let base_color = Color::hsl(rng.gen::<f32>()*360.0, 0.2+(rng.gen::<f32>()) * 0.5, 0.2+rng.gen::<f32>() * 0.5);
                let tile_mesh_aabb =  compute_aabb( &add_node.tx);
                let mat = materials.add(ExtendedMaterial {
                    base: StandardMaterial {
                        //base_color: Color::DARK_GREEN,//*0.25 + base_color*0.75,// Color::DARK_GREEN,
                        specular_transmission: 0.00001,
                        metallic: 0.00015,
                        perceptual_roughness: 0.99,
                        base_color_texture: Some(test_col.clone()),
                        //base_color_texture: Some(test_hgt.clone()),
                        //emissive_texture: Some(test_hgt.clone()),
                        // can be used in forward or deferred mode.
                        opaque_render_method: OpaqueRendererMethod::Auto,
                        // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                        // in forward mode, the output can also be modified after lighting is applied.
                        // see the fragment shader `extended_material.wgsl` for more info.
                        // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                        // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                         normal_map_texture: Some(test_nor.clone()),
                        ..Default::default()
                    },
                    extension: MyExtension { custom_uv: Vec4::new( add_node.uv_offset.x,  add_node.uv_offset.y,  add_node.uv_scale, 1.0), height_texture: Some(test_hgt.clone())},
                });
                tcount += 1;
                let tile = commands.spawn(MaterialMeshBundle {
                    material: mat.clone(),
                    mesh: tile_mesh.0.clone(),
                    transform:  q.local_tx.mul_transform(add_node.tx),//.mul_transform(Transform::from_scale(Vec3::splat(1.01))),
                    ..default()
                })
                .insert(tile_mesh_aabb)
                .insert(TileId {address: add_node.id, face: add_node.face }).id();
                // .insert( (Wireframe,
                //     // This lets you configure the wireframe color of this entity.
                //     // If not set, this will use the color in `WireframeConfig`
                //     WireframeColor {
                //         color: base_color,
                //     })).id();
                let mut pent = commands.entity(pnute);
                pent.add_child(tile);
            }
            let del_ids =  batch.tiles_removal.iter().map(|n|(*n).clone()).collect::<HashSet<_>>();
            for (del_ent, del_node) in tileq.iter() {
                if del_ids.contains( del_node ) {
                    commands.entity(del_ent).despawn();
                    md.1 -= 1;
                } 
            }
            qbatch_assets.queue.pop_front();
        } 
        //qbatch_assets.queue.pop_front();
    
       
    }

        
  
    for ((mut q, mut qbatch_assets, pnute)) in qry.iter_mut() {
        
    }
    

    md.1 += tcount;
}
