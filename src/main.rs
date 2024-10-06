use bevy::prelude::*;
use bevy_hanabi::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_hanabi::HanabiPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

#[derive(Resource)]
pub struct MyEffectHandle(pub Handle<EffectAsset>);

fn startup(mut commands: Commands, assets: Res<AssetServer>) {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(100.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(100.0, 0.0, 0.0, 0.0));
    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.1));
    size_gradient1.add_key(0.3, Vec2::splat(0.1));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));
    let writer = ExprWriter::new();
    let age = writer.lit(0.).uniform(writer.lit(0.2)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.8).uniform(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let accel = writer.lit(Vec3::Y * -8.).expr();
    let update_accel = AccelModifier::new(accel);
    let drag = writer.lit(5.).expr();
    let update_drag = LinearDragModifier::new(drag);
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.).expr(),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(20.) + writer.lit(60.)).expr(),
    };
    let effect_asset = EffectAsset::new(
        vec![32768],
        Spawner::once(2500.0.into(), true),
        writer.finish(),
    )
    .with_name("firework")
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .update(update_drag)
    .update(update_accel)
    .render(ColorOverLifetimeModifier {
        gradient: color_gradient1,
    })
    .render(SizeOverLifetimeModifier {
        gradient: size_gradient1,
        screen_space_size: false,
    });

    commands.insert_resource(MyEffectHandle(assets.add(effect_asset)));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., -20.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn update(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    handle: Res<MyEffectHandle>,
    mut times_pressed: Local<u32>,
) {
    if input.just_pressed(KeyCode::KeyA) {
        *times_pressed += 1;
        println!("Pressed the button {} times!", *times_pressed);
        commands.spawn(ParticleEffectBundle::new(handle.0.clone()));
    }
}
