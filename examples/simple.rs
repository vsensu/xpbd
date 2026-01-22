use bevy::prelude::*;
use xpbd::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(XPBDPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Mesh::from(Sphere { radius: 0.5 }));

    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });

    // spawn circle
    commands.spawn((
        Mesh3d(sphere),
        MeshMaterial3d(white.clone()),
        ParticleBundle::new_with_pos_and_vel(Vec2::ZERO, Vec2::new(2., 0.)),
    ));

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scale: 0.01,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(Vec3::new(0., 0., 100.)),
        Msaa::Sample4,
    ));
}
