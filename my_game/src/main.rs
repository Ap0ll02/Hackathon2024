use bevy::prelude::*;

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Skull;

#[derive(Component)]
struct ground;

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
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Biggest Little Hackathon 2024".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb_u8(155, 202, 224)))
        .add_systems(Startup, setup)
        .add_systems(Update, input_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
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

    // Adding a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(35.0, 35.0, 35.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    // Making the entities {Ship, Skull, Ground}

    // Ship
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("VikingShip.glb#Scene0"),
    //     ..default()
    // }).insert(Ship)
    // .insert(Physics { speed: 3.0, ..Default::default() });

    // Skull
    commands.spawn(SceneBundle {
        scene: asset_server.load("MetalSkull.glb#Scene0"),
        ..default()
    }).insert(Skull)
    .insert(Physics { speed: 4.0, ..Default::default() });
}


fn input_system(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&Ship, &mut Transform, &mut Physics)>, time: Res<Time>) {
    // Moving the ship
    if input.pressed(KeyCode::KeyW) {
        for(_ship, mut transform, mut physics) in query.iter_mut() {
            transform.translation.z -= physics.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::KeyS) {
        for(_ship, mut transform, mut physics) in query.iter_mut() {
            transform.translation.z += physics.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::KeyA) {
        for(_ship, mut transform, mut physics) in query.iter_mut() {
            transform.translation.x -= physics.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::KeyD) {
        for(_ship, mut transform, mut physics) in query.iter_mut() {
            transform.translation.x += physics.speed * time.delta_seconds();
        }
    }
}