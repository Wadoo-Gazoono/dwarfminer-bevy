
use std::default;

use bevy::{math::{ivec2, vec2}, prelude::*, render::{primitives, render_resource::{AsBindGroup, ShaderRef}}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}};
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};

const TILESIZE : f32 = 32.;
const TILE_PER_CHUNK : i8 = 32;
const TILE_AREA_PER_CHUNK : usize = 1024;
const X_CHUNKS : i16 = 20;
const Y_CHUNKS : i16 = 20;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins.set (
        WindowPlugin {
            primary_window: Some(Window {
                title: "Dwarf Miner".into(),
                ..default()
            }),
            ..default()
        }
    ).set(ImagePlugin::default_nearest())
    .set(AssetPlugin {
        watch_for_changes_override: Some(true),
        ..Default::default()
    }),
    ))
    .add_plugins(Material2dPlugin::<ChunkMaterial>::default())
    ////FRAMERATE UI START////
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
    .add_plugins(PerfUiPlugin)
    ////FRAMERATE UI END////
    .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
    .add_systems(PostStartup, (
        setup_camera,
        create_chunk_system   
    ))
    .add_systems(Update, (
        bevy::window::close_on_esc,
        render_chunk_system,
        move_camera_system
    ))
    .run();
}


fn setup_camera(mut commands: Commands){
    commands.spawn((Camera2dBundle {
        transform : Transform::from_scale(Vec3::splat(12.)),
        ..Default::default()
    }));
    commands.spawn(PerfUiCompleteBundle::default());
}



#[derive(Component)]
struct Chunk {
    active:bool,
    position: IVec2,
    tile_data:[Block;TILE_AREA_PER_CHUNK]
}

impl Chunk {
    fn empty(x:i16, y:i16) -> Chunk{
        let tile_data: [Block; TILE_AREA_PER_CHUNK] = [Block {
            block_id : 0,
            wall_id : 0
        }; TILE_AREA_PER_CHUNK];  
        return Chunk{
            active: false,
            position: ivec2(x.into(), y.into()),
            tile_data : tile_data
        }
    }

    fn get_2d_index(index :i16) -> (i16, i16){
        let x = index / TILE_PER_CHUNK as i16;
        let y = index % TILE_PER_CHUNK as i16;
        return (x,y);
    }

    fn pixel_to_chunk_position(&self) -> Vec2 {
        return Vec2::floor(self.position.as_vec2() / (TILESIZE * TILE_PER_CHUNK as f32));
    }

    fn pixel_to_tile_position(&self) -> Vec2 {
        return Vec2::floor(self.position.as_vec2() / (TILESIZE as f32));
    }
}

#[derive(Bundle)]
struct ChunkBundle {
    chunk:Chunk,
    //spacial:SpatialBundle,
}

#[derive(Clone, Copy)]
struct Block{
    block_id : i16,
    wall_id : i8
}

fn create_chunk_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    asset_server: Res<AssetServer>,
){
    for i in 0..X_CHUNKS{
        for j in 0..Y_CHUNKS{
            let chunkTransform = Transform::from_xyz((i as f32 * TILE_PER_CHUNK as f32 * TILESIZE) as f32, (j as f32 * TILE_PER_CHUNK as f32 * TILESIZE) as f32, 0.);
            commands.spawn((
                Name::new("Chunk"),
                ChunkBundle {
                    chunk: Chunk::empty(i,j),
                    //spacial: SpatialBundle::from_transform(chunkTransform)
                    
                },
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::default()).into(),
                    transform: chunkTransform.with_scale(Vec3::splat(TILESIZE * TILE_PER_CHUNK as f32)),
                    material: materials.add(ChunkMaterial {
                        color: Color::WHITE,
                        color_texture : Some(asset_server.load("textures/blocks/blocks.png"))
                    }),
                    ..default()
                }
            ));
        }
    }
}

fn render_chunk_system(mut gizmos : Gizmos, query: Query<(&Chunk, &Transform)>){
    for (chunk, transform) in query.iter(){
        gizmos.rect_2d(transform.translation.xy() + vec2(TILESIZE  * TILE_PER_CHUNK as f32 /2., TILESIZE * TILE_PER_CHUNK  as f32 /2.), 0., vec2(TILESIZE  * TILE_PER_CHUNK as f32, TILESIZE * TILE_PER_CHUNK  as f32), Color::BLACK);

    }
}

fn move_camera_system(mut query: Query<(&mut Transform), With<Camera2d>>, keyboard: Res<ButtonInput<KeyCode>>){
    for mut transform in query.iter_mut(){
        if(keyboard.pressed(KeyCode::ArrowRight)){
            transform.translation.x += 10.;
        }
        if(keyboard.pressed(KeyCode::ArrowUp)){
            transform.translation.y += 10.;
        }
        if(keyboard.pressed(KeyCode::ArrowLeft)){
            transform.translation.x -= 10.;
        }
        if(keyboard.pressed(KeyCode::ArrowDown)){
            transform.translation.y -= 10.;
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ChunkMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}


impl Material2d for ChunkMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chunk_shader.wgsl".into()
    }
}

