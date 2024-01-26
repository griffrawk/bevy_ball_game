#[allow(unused)]
pub fn confine_sprite_movement(
    mut sprite_query: Query<(&mut Transform, AnyOf<(&Enemy, &Player)>)>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Slightly more generic confine, in that it works for both Enemy and Player, using the sizes
    // stored in the component. That makes it a bit more flexible if a third component comes along with
    // a different size.
    let window = window_query.get_single().unwrap();

    for (mut sprite_transform, target) in sprite_query.iter_mut() {
        let half_sprite_size = match target {
            (Some(sprite), None) => {
                sprite.size / 2.0
            }
            (None, Some(sprite)) => {
                sprite.size / 2.0
            }
            _ => unreachable!("entities should never have Enemy and Player at the same time")
        };

        let x_min = 0.0 + half_sprite_size;
        let x_max = window.width() - half_sprite_size;
        let y_min = 0.0 + half_sprite_size;
        let y_max = window.height() - half_sprite_size;

        let mut translation = sprite_transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        sprite_transform.translation = translation;
    }
}
