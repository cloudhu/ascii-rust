use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{component::*, util::*, resource::Fonts};

use super::TurretFireEvent;

pub fn fire_rocket_launcher(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage)>,
    parent_query: Query<&Transform>,
    fonts: Res<Fonts>,
) {
    for ev in fire_event.iter() {
        match ev.class {
            TurretClass::RocketLauncher => {

                // Get Turret Info
                let Ok((parent, targets, damage)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                // Spawn rocket
                let origin = parent_transform.translation.truncate();
                commands.spawn((
                    Bullet::new(3.0),
                    Text2dBundle {
                        text: Text::from_section(
                            "!",
                            TextStyle {
                                font: fonts.primary.clone(),
                                font_size: 12.0,
                                color: Colour::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform {
                            translation: origin.extend(RenderLayer::Bullet.as_z()),
                            ..Default::default()
                        },
                        ..default()
                    },
                    BaseGlyphRotation {
                        rotation: Quat::from_rotation_z(PI / 2.0),
                    },
                    Physics {
                        acceleration: Vec2::ZERO,
                        velocity: Vec2::ZERO,
                        drag: 0.0,
                    },
                    Engine::new_with_steering(40.0, 10.0, 0.5),
                    Seeker(target),
                    Collider { radius: 5.0 },
                    Owner(parent.get()),
                    ExplodesOnDespawn::default(),
                    AoeDamage { damage: damage.amount, range: 40.0 },
                    DespawnWithScene,
                ));

            },
            _ => (),
        }
    }
}