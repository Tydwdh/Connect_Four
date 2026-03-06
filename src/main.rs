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
        .add_plugins(GameInputPlugin) //游戏输入
        .add_plugins(FourPlugin) //四子棋游戏
        .add_plugins(FallingAnimationPlugin) //棋子掉落动画
        .add_plugins(BoardPlugin) //棋盘动画
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
    assets: Res<PieceAssets>,
) {
    for message in messages.read() {
        let target_y = board_to_world(message.row, message.col).y;
        let start_y = message.world_pos_y.max(target_y);
        let material_handle = match message.player {
            Piece::Yellow => assets.player1_material.clone(),
            Piece::Red => assets.player2_material.clone(),
            Piece::None => panic!("Invalid piece"),
        };
        commands.spawn((
            Transform::from_xyz(col_to_x(message.col), start_y, 10.0),
            PieceBundle::new(assets.mesh.clone(), material_handle),
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
