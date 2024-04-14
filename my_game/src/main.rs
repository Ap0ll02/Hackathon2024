use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::math::primitives::Plane3d;

#[derive(Component)]
struct Skull;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Maze;

#[derive(Component)]
struct Physics {
    velocity: Vec3,
    speed: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Physics {
            velocity: [0.0, 0.0, 0.0].into(),
            speed: 1.0,
        }
    }
}

fn main() {
    // App Setup
    let app_window = Some(Window {
        title: "Hackathon 2024".into(),
        ..default()
    });

    // APP
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: app_window,
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::rgb_u8(155, 202, 224)))
        .add_systems(Startup, (setup_physics, setup).chain())
        .add_systems(Update, input_system)
        .run();
}

// SYSTEMS:
// Setup system, runs once
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>, mut rapier_config: ResMut<RapierConfiguration>, query: Query<(Entity, &AsyncCollider)>,
    mut active_types: Query<&mut ActiveCollisionTypes>, mut active_events: Query<&mut ActiveEvents>,) {
    // Physics engine config
    rapier_config.gravity = Vec3::ZERO;

    // Adding a light
    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight {
            color: Color::rgba_u8(200, 200, 100, 100),
            shadows_enabled : true,
            illuminance : 5000.0, 
            ..Default::default()
        },
        ..default()
    });

    // CAMERA: Adding a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(40.0, 1.0, 0.0).looking_at(Vec3::new(-40., 0., 0.), Vec3::Y),
        ..default()
    });

    // Making the entities {Skull (player), Ground, Maze}

    // Skull
    let skull_entity = commands.spawn(SceneBundle {
        scene: asset_server.load("MetalSkull.glb#Scene0"),
        ..default()
    }).insert(Skull)
    .insert(Collider::ball(1.0)) // Replace AsyncCollider::default() with this
    .insert(Physics { speed: 9.0, ..Default::default() }).id();

    commands.entity(skull_entity)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(GravityScale(1.0))
        .insert(LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED_X)
        .insert(Transform::from_xyz(30.0, 0.0, 8.0));

    // Ground
    let ground = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d { normal: Direction3d::new(Vec3::Y).expect("What the hell"), ..Default::default()})),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            base_color_texture: Some(asset_server.load("Floor.png")),
            ..Default::default()
        }),
        transform: Transform::from_scale(Vec3::new(180.0, 1.0, 180.0)).with_translation(Vec3::new(0.0, -3.0, 0.0)),
        ..Default::default()
    }).insert(Ground)
    .id();

    commands.entity(ground)
            .insert(RigidBody::Fixed);

    // Maze
    let maze_entity = commands.spawn(SceneBundle {
        scene: asset_server.load("Maze.glb#Scene0"),
        ..default()
    }).insert(Maze)
    .insert(AsyncCollider::default())
    .id();

    commands.entity(maze_entity)
            .insert(RigidBody::Fixed)
            .insert(Transform::from_xyz(0.0, 0.0, 0.0));

    for mut active_events in active_events.iter_mut() {
        *active_events = ActiveEvents::COLLISION_EVENTS;
    }
    for mut active_types in active_types.iter_mut() {
        *active_types = ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC;
    }
}

fn setup_physics(mut commands: Commands) {
    // Ground
    commands.spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Transform::from_xyz(0.0, -2.0, 0.0));
}

// Input System
fn input_system(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&Skull, &mut Transform, &Physics)>, time: Res<Time>) {
    // Moving the skull entity {W, A, S, D}
    if input.pressed(KeyCode::KeyW) {
        for(_skull, mut transform, physics) in query.iter_mut() {
            transform.translation.x -= physics.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::KeyS) {
        for(_skull, mut transform, physics) in query.iter_mut() {
            transform.translation.x += physics.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::KeyA) {
        for(_skull, mut transform, physics) in query.iter_mut() {
            transform.translation.z += physics.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::KeyD) {
        for(_skull, mut transform, physics) in query.iter_mut() {
            transform.translation.z -= physics.speed * time.delta_seconds();
        }
    }
}