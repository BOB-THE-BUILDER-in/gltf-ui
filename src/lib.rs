//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::{prelude::*, render::mesh::Indices};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages; 
use bevy_panorbit_camera::{PanOrbitCameraPlugin,PanOrbitCamera};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_plugins(EguiPlugin)
        .insert_resource(SliderValue(50.0))
        .add_systems(Update, ui_system)
        
        .run();
    
}
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    ///Users/jain_aditya/Documents/Rust/bevvy/load_gltf/assets/models/tree.gltf
   
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("/Users/jain_aditya/Documents/Rust/bevvy/load_gltf/assets/models/tree.gltf"), // for web just use "tree.gltf"
    )),  // Load GLTF as SceneRoot
        Transform::from_scale(Vec3::splat(1.0)),                // Initial transform with scale
        GlobalTransform::default(),                             // Required for rendering
        GltfMarker                                              // Marker component
    ));
}
#[derive(Resource)]
struct SliderValue(f32);

#[derive(Component)]
struct GltfMarker;

fn ui_system(
    mut contexts: EguiContexts,
    mut slider_value: ResMut<SliderValue>,
    mut gltf_transforms: Query<&mut Transform, With<GltfMarker>>,
) {
    egui::Window::new("Slider Example")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            // Create the slider
            ui.add(
                egui::Slider::new(&mut slider_value.0, 0.0..=1000.0)
                    .text("Value")
                    .step_by(1.0),
            );

            // Display the current value
            ui.label(format!("Current value: {:.1}", slider_value.0));
        });

    // Update the scale of GLTF assets based on the slider value
    for mut transform in gltf_transforms.iter_mut() {
        let scale = slider_value.0 / 500.0; // Convert slider value (0-1000) to scale (0-2)
        transform.scale = Vec3::splat(scale); // Apply the scale to the GLTF asset
    }
}