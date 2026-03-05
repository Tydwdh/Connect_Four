use bevy::prelude::*;
mod board;
use board::*;
mod four;
use four::*;
mod ui;
use ui::*;
mod config;
pub use config::*;
mod game_input;
use game_input::*;
mod falling_animation;
use falling_animation::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameInputPlugin)
        .add_plugins(FourPlugin)
        .add_plugins(FallingAnimationPlugin)
        .add_plugins(BoardPlugin)
        .init_resource::<HighlightColumn>()
        .add_systems(Startup, (setup_camera, setup_ui))
        .add_systems(
            Update,
            (
                place_piece.run_if(not(is_falling).and(not(is_game_over))),
                reset_system,
                update_ui,
            )
                .chain(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn place_piece(
    mut messages: MessageReader<SpawnPieceMessage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for message in messages.read() {
        let mesh = meshes.add(Circle::new(CELL_SIZE * 0.35));
        let material = materials.add(message.player.color());
        let target_y = board_to_world(message.row, message.col).y;
        let start_y = message.world_pos_y.max(target_y);
        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_xyz(col_to_x(message.col), start_y, 10.0),
            PieceSprite,
            FallingPiece {
                target_y,
                player: message.player,
                row: message.row,
                col: message.col,
            },
        ));
    }
}

fn reset_system(
    mut board: ResMut<Board>,
    mut commands: Commands,
    pieces: Query<Entity, With<PieceSprite>>,
    mut messages: MessageReader<ClearBoardMessage>,
) {
    for _message in messages.read() {
        for entity in pieces.iter() {
            commands.entity(entity).despawn();
        }
        // 重置棋盘
        *board = Board::new();
    }
}
