use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

const CUBE_SIZE: f32 = 1.;
const HALF_CUBE_SIZE: f32 = CUBE_SIZE / 2.;
const CUBE_SUBSTITUTIONS: i32 = 2;
const SUBSTITUTIONS_SIZE: f32 = CUBE_SIZE / CUBE_SUBSTITUTIONS as f32;
const SUBSTITUTIONS_SIZE_PADDING: f32 = SUBSTITUTIONS_SIZE / 100. * 10.;
const NATURAL_SUBSTITUTIONS_SIZE: f32 = SUBSTITUTIONS_SIZE / 100. * 90.;
const HALF_SUBSTITUTIONS_SIZE: f32 = SUBSTITUTIONS_SIZE / 2.;

#[derive(Component)]
struct Sliceble;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EditorPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, slice)
        .run();
}

fn slice(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<
        (
            Entity,
            &GlobalTransform,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
        ),
        With<Sliceble>,
    >,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (entity, global_transform, mesh_handle, material_handle) in query.iter() {
            let Some(mesh) = meshes.get(mesh_handle) else {
                panic!()
            };
            let start_point = global_transform.translation()
                - Vec3::new(HALF_CUBE_SIZE, HALF_CUBE_SIZE, HALF_CUBE_SIZE);
            commands.entity(entity).despawn();

            let mesh = Mesh::from(shape::Cube {
                size: NATURAL_SUBSTITUTIONS_SIZE,
            });

            for x in 0..CUBE_SUBSTITUTIONS {
                for y in 0..CUBE_SUBSTITUTIONS {
                    for z in 0..CUBE_SUBSTITUTIONS {
                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(mesh.clone()),
                                material: material_handle.clone(),
                                transform: Transform::from_xyz(
                                    HALF_SUBSTITUTIONS_SIZE
                                        + start_point.x
                                        + SUBSTITUTIONS_SIZE * x as f32,
                                    HALF_SUBSTITUTIONS_SIZE
                                        + start_point.y
                                        + SUBSTITUTIONS_SIZE * y as f32,
                                    HALF_SUBSTITUTIONS_SIZE
                                        + start_point.z
                                        + SUBSTITUTIONS_SIZE * z as f32,
                                ),
                                ..default()
                            },
                            RigidBody::Dynamic,
                            Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
                                .unwrap(),
                            Sleeping::disabled(),
                        ));
                    }
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh::from(shape::Cube { size: CUBE_SIZE });

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh.clone()),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Sliceble,
        RigidBody::Dynamic,
        Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap(),
    ));

    let circle_mesh_handler = meshes.add(Circle::new(4.0));

    // circular base
    commands.spawn((
        PbrBundle {
            mesh: circle_mesh_handler.clone(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
        RigidBody::Fixed,
        Collider::from_bevy_mesh(
            &Mesh::from(Circle::new(4.0)),
            &ComputedColliderShape::TriMesh,
        )
        .unwrap(),
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
