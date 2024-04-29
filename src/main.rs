//! A simple 3D scene with light shining over a cube sitting on a plane.
mod block_material;

use crate::block_material::{BlockMaterial, BlockMaterialPlugin};
use bevy::asset::LoadState;
use bevy::math::uvec3;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlockMaterialPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_custom_block)
        .run();
}

#[derive(Resource)]
struct LoadingTexture {
    is_loaded: bool,
    handle: Handle<Image>,
}

fn spawn_custom_block(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BlockMaterial>>,
) {
    if loading_texture.is_loaded
        || asset_server.load_state(loading_texture.handle.id()) != LoadState::Loaded
    {
        return;
    }
    loading_texture.is_loaded = true;
    let image = images.get_mut(&loading_texture.handle).unwrap();

    // Create a new array texture asset from the loaded texture.
    let array_layers = 4;
    image.reinterpret_stacked_2d_as_array(array_layers);

    // emissive texture
    let emissive_texture = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: array_layers,
        },
        TextureDimension::D2,
        &[0; 4],
        TextureFormat::Rgba8Unorm,
        default(),
    );

    // metallic roughness texture
    let metallic_roughness_texture = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: array_layers,
        },
        TextureDimension::D2,
        // red = unused, green = roughness, blue = metallic
        &[0, 255, 0, 255],
        TextureFormat::Rgba8Unorm,
        default(),
    );

    // Spawn some cubes using the array texture
    let material_handle = materials.add(BlockMaterial {
        base_color_array_texture: loading_texture.handle.clone(),
        emissive_array_texture: images.add(emissive_texture),
        metallic_roughness_array_texture: images.add(metallic_roughness_texture),
        // haven't tested other alpha modes with the shader
        alpha_mode: AlphaMode::Opaque,
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(cube_mesh()),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.5, 0.0, -0.5),
        ..Default::default()
    });
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Start loading the texture.
    commands.insert_resource(LoadingTexture {
        is_loaded: false,
        handle: asset_server.load("array_texture.png"),
    });
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn cube_mesh() -> Mesh {
    let min = uvec3(0, 0, 0);
    let max = uvec3(1, 1, 1);

    // Suppose Y-up right hand, and camera look from +Z to -Z
    let vertices = &[
        // Front
        ([min.x, min.y, max.z], [0.0_f32, 0.0, 1.0], [0.0_f32, 0.0]),
        ([max.x, min.y, max.z], [0.0, 0.0, 1.0], [1.0, 0.0]),
        ([max.x, max.y, max.z], [0.0, 0.0, 1.0], [1.0, 1.0]),
        ([min.x, max.y, max.z], [0.0, 0.0, 1.0], [0.0, 1.0]),
        // Back
        ([min.x, max.y, min.z], [0.0, 0.0, -1.0], [1.0, 0.0]),
        ([max.x, max.y, min.z], [0.0, 0.0, -1.0], [0.0, 0.0]),
        ([max.x, min.y, min.z], [0.0, 0.0, -1.0], [0.0, 1.0]),
        ([min.x, min.y, min.z], [0.0, 0.0, -1.0], [1.0, 1.0]),
        // Right
        ([max.x, min.y, min.z], [1.0, 0.0, 0.0], [0.0, 0.0]),
        ([max.x, max.y, min.z], [1.0, 0.0, 0.0], [1.0, 0.0]),
        ([max.x, max.y, max.z], [1.0, 0.0, 0.0], [1.0, 1.0]),
        ([max.x, min.y, max.z], [1.0, 0.0, 0.0], [0.0, 1.0]),
        // Left
        ([min.x, min.y, max.z], [-1.0, 0.0, 0.0], [1.0, 0.0]),
        ([min.x, max.y, max.z], [-1.0, 0.0, 0.0], [0.0, 0.0]),
        ([min.x, max.y, min.z], [-1.0, 0.0, 0.0], [0.0, 1.0]),
        ([min.x, min.y, min.z], [-1.0, 0.0, 0.0], [1.0, 1.0]),
        // Top
        ([max.x, max.y, min.z], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([min.x, max.y, min.z], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([min.x, max.y, max.z], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([max.x, max.y, max.z], [0.0, 1.0, 0.0], [1.0, 1.0]),
        // Bottom
        ([max.x, min.y, max.z], [0.0, -1.0, 0.0], [0.0, 0.0]),
        ([min.x, min.y, max.z], [0.0, -1.0, 0.0], [1.0, 0.0]),
        ([min.x, min.y, min.z], [0.0, -1.0, 0.0], [1.0, 1.0]),
        ([max.x, min.y, min.z], [0.0, -1.0, 0.0], [0.0, 1.0]),
    ];

    let vert_data: Vec<_> = vertices
        .iter()
        .map(|(p, n, uv)| make_vertex_uvec3((*p).into(), (*n).into(), (*uv).into(), 0))
        .collect();

    let indices = Indices::U32(vec![
        0, 1, 2, 2, 3, 0, // front
        4, 5, 6, 6, 7, 4, // back
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // top
        20, 21, 22, 22, 23, 20, // bottom
    ]);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(BlockMaterial::ATTRIBUTE_VERT_DATA, vert_data);
    mesh.insert_indices(indices);

    mesh
}

fn make_vertex_uvec3(pos: UVec3, normal: Vec3, uv: Vec2, texture_layer: u16) -> UVec3 {
    // You can store numbers 0 to 32 in 6 bits. Increase/reduce it based on your needs
    // you could probably get away with 3 bits per coordinate if cube shapes is all you want.

    // Make normal values all positive since we'll be storing the normal unsigned
    let normal = (normal + Vec3::ONE) * 16.;
    assert!(normal.min_element() >= 0.0);
    assert!(normal.max_element() <= 32.0);
    // dbg!(normal);
    uvec3(
        // we need 6 bits per position coordinate to store values from 0 to 63
        pos.x | pos.y << 6 | pos.z << 12,
        (normal.x as u32)
            | (normal.y as u32) << 6
            | (normal.z as u32) << 12
        // 14 bits left for texture layer
            | (texture_layer as u32) << 18,
        // 11 bits per uv allows us to store 0 to 2047, adjust based on your needs
        (uv.x * 32.) as u32 | ((uv.y * 32.) as u32) << 11,
    )
}
