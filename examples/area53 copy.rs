use std::{collections::VecDeque, hash::Hash};

use ardh::{camera_controller, ardh::{ArdhFlat, QT, TileId}, quadtree::ZNodeIndex};
use bevy::{prelude::*, utils::HashSet, render::{RenderPlugin, settings::{WgpuSettings, WgpuFeatures, RenderCreation}, render_resource::{AsBindGroup, ShaderRef}}, pbr::{wireframe::{WireframePlugin, WireframeConfig, WireframeColor, Wireframe}, ExtendedMaterial, MaterialExtension, OpaqueRendererMethod, ScreenSpaceAmbientOcclusionBundle, CascadeShadowConfigBuilder}, reflect::{TypeUuid, TypePath}, core_pipeline::experimental::taa::TemporalAntiAliasPlugin};
//use bevy_infinite_grid::{GridShadowCamera, InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use camera_controller::{CameraController, CameraControllerPlugin};

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
    .add_plugins((
        DefaultPlugins
        // .set(RenderPlugin {
        //     render_creation: RenderCreation::Automatic(WgpuSettings {
        //         // WARN this is a native only feature. It will not work with webgl or webgpu
        //         features: WgpuFeatures::POLYGON_MODE_LINE,
        //         ..default()
        //     }),
        // }),
        // // You need to add this plugin to enable wireframe rendering
        // WireframePlugin,
    ))
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
        .add_systems(Update, max_depth_sys)
        //.insert_resource(FixedTime::new_from_secs(0.001))
        .insert_resource(MaxDepth(2))
        .run();
}

#[derive(Resource)]
struct TileMesh(Handle<Mesh>);


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
        brightness: 0.025,
    });

    let mesh = meshes.add(Mesh::from(shape::Plane { size: 150.0*4.0, subdivisions: 128*2 }).with_generated_tangents().unwrap());

    commands.insert_resource(TileMesh(mesh));

   // wireframe_config.global = false;
    let mut tree = ardh::quadtree::QuadTree::new();
    let root = ardh::ardh::Node {
        parent_copy: None,
        id: 0,
        depth: 0,
        tx: Transform::default(),
        index: ZNodeIndex::None,
        size: 150.0
    };
    tree.set_root(Some(root.clone()));
    // mark the source vertex as discovered
    let mut discovered = HashSet::new();
    let mut stack = VecDeque::new();

    discovered.insert(root.clone());
    stack.push_back(root);


    commands.spawn(ArdhFlat {
        local_tx: Transform::default(),
        size: 150.0,
        stree: ardh::ardh::SearchTree { tree , 
                        running: true, 
                        stack,
                        leafs: HashSet::new(),
                        leafs_prev: HashSet::new(),
                         discovered },
    }).insert(TransformBundle::default())
    .insert(VisibilityBundle::default())
    .insert(BatchAssets::default());

    // commands.spawn(InfiniteGridBundle {
    //     grid: InfiniteGrid {
    //         // shadow_color: None,
    //         ..default()
    //     },
    //     ..default()
    // });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 200.0),
            ..default()
        },
        CameraController::default(),
        FogSettings {
            color: Color::rgba(0.25, 0.25, 0.35, 1.0),
            falloff: FogFalloff::Linear {
                start: 5.0,
                end: 650.0,
            },
            ..default()
        },
       // GridShadowCamera,
       //paceAmbientOcclusionBundle::default()
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 35000.0,
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
        transform: Transform::from_translation(Vec3::X * 0. + Vec3::Y * 90.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

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
        ]),
        TextChanges,
    ));
}

fn sdiver(mut commands: Commands, 
    //mut tile_images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    //mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
   // mut materials: ResMut<Assets<CustomMaterial>>,
    tile_mesh: Res<TileMesh>,
    mut qtext: Query<&mut Text, With<TextChanges>>,
    mut meshes: ResMut<Assets<Mesh>>, md: Res<MaxDepth>, mut qry: Query<(&mut ArdhFlat, &mut BatchAssets, Entity)>, tileq: Query<(Entity, &TileId)>, qcam: Query<&Transform, With<Camera>>) {
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


    }
    for (mut q, batch_assets, pnute) in qry.iter_mut() {
        //println!("running dfs iter");
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut xqloop = true;
        while xqloop {
        let qloop = q.stree.dfs(|node| {

           if (node.depth as i32) == 0 { return true }
            
           // return (node.depth as i32) < md.0 ;
            if (node.depth as i32) >= md.0 { return false}
            
            let boxy_dist: f32 = [Vec3::new(1.0, 0.0, 0.0), 
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0), 
            Vec3::new(0.0, 0.0, -1.0)]
                    .map(|t| t*node.size*0.0 + node.tx.translation )
                    .map(|t| cam_tx.translation.distance(t ) - 75.0 ).iter().sum::<f32>() / 4.0 ;
              //  .map(|t| cam_tx.translation.distance(t )  ).iter().sum::<f32>() / 4.0 ;
            //let boxy_dist =  cam_tx.translation.distance(node.tx.translation );
            //let cliper = 4.0 * (2000.0 / boxy_dist ).log10();
            //let cliper =  ( 70000.0/boxy_dist.powf(1.7) ).log2() ;
            //let cliper =  ( 30000.0/boxy_dist.powf(1.7) ).log2() ;
            let cliper =  0.8*( 50000.0/boxy_dist.powf(2.0) ).log2() ;
            // Adjust the distance calculation for the curvature of the planet
            // let distance = 1.0/boxy_dist;
            // let radius = 1.0/100.0f32; // Radius of the planet
            // let curvatureDistance = ( -(distance * distance) + radius * radius).sqrt();// - radius;
            // let BaseDistance = 1.1;

            // Use a logarithmic function to calculate the LOD level based on the adjusted distance
            //let cliper = (  BaseDistance / curvatureDistance).log2();//.max(0.0);
            
            // Clamp to the maximum allowed LOD level
            
            println!("dist {} {}", node.depth, cliper);
            if node.depth < cliper as usize  {
                return true;
            }
             return false;
        });
//        println!("STACK {}", q.stree.stack.len());
        if  q.stree.stack.len() == 0 && q.stree.leafs.len() > 0 {
//        println!("leavs\n {:?}", q.stree.leafs.iter().map(|x| x.id).collect::<Vec<_>>());
//println!("leavs\n {:?}", q.stree.leafs.iter().map(|x| x.depth).collect::<Vec<_>>());
//q.stree.leafs.clear();
        //let mut add_list = HashSet::new();
        
        // for new_node in q.stree.leafs.iter() {
        //     if !q.stree.leafs_prev.contains(new_node) {
        //         add_list.insert(new_node.clone());
        //     }
        // }

        let add_list = { q.stree.leafs
            .difference( &q.stree.leafs_prev ).map(|x| x.clone()).collect::<HashSet::<_>>() };



        let del_list = { q.stree.leafs_prev
            .difference( &q.stree.leafs ).map(|x| x.clone()).collect::<HashSet::<_>>() };
        q.stree.leafs_prev = q.stree.leafs.clone();
        //println!("SUM {} ADD {} REM {}",q.stree.leafs.len(),  add_list.len(), del_list.len());
        q.stree.leafs.clear();

        for add_node in add_list {
            let base_color = Color::hsl(rng.gen::<f32>()*360.0, 0.2+(rng.gen::<f32>()) * 0.5, 0.2+rng.gen::<f32>() * 0.5);
            let test_hgt = asset_server.load(format!("tiles\\hgt{}.exr", add_node.id));
            let test_nor = asset_server.load(format!("tiles\\nor{}.png", add_node.id));
            println!("LOADNG tile #{}", add_node.id);
            let mat = materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color:base_color,// Color::DARK_GREEN,
                    specular_transmission: 0.2,
                    metallic: 0.025,
                    perceptual_roughness: 0.8,
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
                extension: MyExtension { quantize_steps: 3, height_texture: Some(test_hgt.clone()) },
            });

            let tile = commands.spawn(MaterialMeshBundle {
                material: mat.clone(),
                mesh: tile_mesh.0.clone(),
                transform: add_node.tx * Transform::from_scale(Vec3::splat(1.01)),
                
                ..default()
            }).insert(TileId {address: add_node.id}).insert( (Wireframe,
                // This lets you configure the wireframe color of this entity.
                // If not set, this will use the color in `WireframeConfig`
                WireframeColor {
                    color: base_color,
                })).id();
            let mut pent = commands.entity(pnute);
          
            pent.add_child(tile);
        }
        let del_ids =  del_list.iter().map(|n|n.id).collect::<HashSet<_>>();
        for (del_ent, del_node) in tileq.iter() {
            if del_ids.contains( &del_node.address ) {
                commands.entity(del_ent).despawn();
            } 
        }

    }
    xqloop = q.stree.stack.len() > 0;
    if qloop { break }
    } }
}

#[derive(Debug, Resource)]
struct MaxDepth(i32);

fn max_depth_sys(mut md: ResMut<MaxDepth>, 
                key_input: Res<Input<KeyCode>>) {
        if key_input.just_pressed(KeyCode::NumpadAdd) {
            md.0 += 1;
            println!("MXD: {:?}", md);
        }
        else if key_input.just_pressed(KeyCode::NumpadSubtract) {
            md.0 -= 1;
            println!("MXD: {:?}", md);
        }
    
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
    quantize_steps: u32,
    #[texture(101)]
    #[sampler(102)]
    height_texture: Option<Handle<Image>>,
}

impl MaterialExtension for MyExtension {
    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

   
    // fn deferred_vertex_shader() -> ShaderRef {
    //     "shaders/extended_material.wgsl".into()
    // }

    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
}


#[derive(Component, Default)]
struct BatchAssets {
    pub images: Vec<Handle<Image>>
}
