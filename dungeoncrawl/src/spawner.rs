use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: Color::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
