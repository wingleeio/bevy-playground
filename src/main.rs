use bevy::prelude::*;

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Direction {
    x: f32,
    y: f32,
}

fn add_players(
    mut commands: Commands
) {
    commands
        .spawn()
        .insert(Player)
        .insert(Position { x: 10.3, y: 0.4})
        .insert(Direction { x: 0.0, y: 0.0});
}

fn move_players(
    query: Query<(&Position, &Direction, With<Player>)>
) {
    for (position, direction, _) in query.iter() {
        println!("posX: {}, posY: {}", position.x, position.y);
        println!("dirX: {}, dirY: {}", direction.x, direction.y);
    }
}

fn main() {
    let mut app = App::new();

    app.add_startup_system(add_players);

    app.add_system(move_players);

    app.run();
}
