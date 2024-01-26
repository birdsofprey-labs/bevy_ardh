use std::{collections::VecDeque, hash::Hash};

use ardh::{camera_controller, ardh::{ArdhFlat, QT, TileId}, quadtree::ZNodeIndex};
use bevy::{prelude::*, utils::HashSet, render::{RenderPlugin, settings::{WgpuSettings, WgpuFeatures}}, pbr::wireframe::{WireframePlugin, WireframeConfig}};
use bevy_infinite_grid::{GridShadowCamera, InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use camera_controller::{CameraController, CameraControllerPlugin};

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            },
        }),
        WireframePlugin,
    ))
        .add_plugins((CameraControllerPlugin, InfiniteGridPlugin))
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
    mut wireframe_config: ResMut<WireframeConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 100.0*4.0, subdivisions: 128 }));

    commands.insert_resource(TileMesh(mesh));

    wireframe_config.global = false;
    let mut tree = ardh::quadtree::QuadTree::new();
    let root = ardh::ardh::Node {
        parent_copy: None,
        id: 0,
        depth: 0,
        tx: Transform::default(),
        index: ZNodeIndex::None,
        size: 100.0
    };
    tree.set_root(Some(root.clone()));
    // mark the source vertex as discovered
    let mut discovered = HashSet::new();
    let mut stack = VecDeque::new();

    discovered.insert(root.clone());
    stack.push_back(root);


    commands.spawn(ArdhFlat {
        local_tx: Transform::default(),
        size: 100.0,
        stree: ardh::ardh::SearchTree { tree , 
                        running: true, 
                        stack,
                        leafs: HashSet::new(),
                        leafs_prev: HashSet::new(),
                         discovered },
    }).insert(TransformBundle::default())
    .insert(VisibilityBundle::default());


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
        GridShadowCamera,
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.)
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
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    tile_mesh: Res<TileMesh>,
    mut qtext: Query<&mut Text, With<TextChanges>>,
    mut meshes: ResMut<Assets<Mesh>>, md: Res<MaxDepth>, mut qry: Query<(&mut ArdhFlat, Entity)>, tileq: Query<(Entity, &TileId)>, qcam: Query<&Transform, With<Camera>>) {
    let cam_tx = qcam.single();

    let test_tex = asset_server.load("testmap.png");

    for mut text in &mut qtext {
        
       

        text.sections[0].value = format!(
            "Max Depth: {}", md.0,
        );


    }
    for (mut q, pnute) in qry.iter_mut() {
        //println!("running dfs iter");
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut xqloop = true;
        while xqloop {
        let qloop = q.stree.dfs(|node| {

           if (node.depth as i32) == 0 { return true }
            
           // return (node.depth as i32) < md.0 ;
            if (node.depth as i32) >= md.0 { return false}
            

            let boxy_dist =  cam_tx.translation.distance(node.tx.translation );
            //let cliper = 4.0 * (2000.0 / boxy_dist ).log10();
            //let cliper =  ( 70000.0/boxy_dist.powf(1.7) ).log2() ;
            //let cliper =  ( 30000.0/boxy_dist.powf(1.7) ).log2() ;
            let cliper =  0.8*( 40000.0/boxy_dist.powf(2.0) ).log2() ;
            //println!("dist {} {}", node.depth, cliper);
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
        println!("SUM {} ADD {} REM {}",q.stree.leafs.len(),  add_list.len(), del_list.len());
        q.stree.leafs.clear();

        for add_node in add_list {
            let base_color = Color::hsl(rng.gen::<f32>()*360.0, 0.2+(rng.gen::<f32>()) * 0.5, 0.2+rng.gen::<f32>() * 0.5);
            let mat = standard_materials.add(StandardMaterial { base_color, base_color_texture: Some(test_tex.clone()),  .. default() });


            let tile = commands.spawn(PbrBundle {
                material: mat.clone(),
                mesh: tile_mesh.0.clone(),
                transform: add_node.tx,
                
                ..default()
            }).insert(TileId {address: add_node.id}).id();
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
