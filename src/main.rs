use bevy::prelude::*;
mod board;
use board::*;
mod four;
use four::*;
mod ui;
use ui::*;
mod config;
pub use config::*;
mod mouse;
use mouse::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Board>() // 自动使用 Board::new()
        .add_message::<SpawnPieceEvent>()
        .add_systems(Startup, (setup_camera, setup_board, setup_ui))
        .add_systems(
            Update,
            (
                detect_board_click.run_if(not(is_falling).and(not(is_game_over))), // 条件1: 没有下落
                mouse_click_system,
                reset_system,
                update_ui,
                falling_animation_system,
            )
                .chain(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn board_to_world(row: usize, col: usize) -> Vec2 {
    let x = OFFSET_X + (col as f32 + 0.5) * CELL_SIZE;
    let y = OFFSET_Y + (row as f32 + 0.5) * CELL_SIZE;
    Vec2::new(x, y)
}

fn col_to_x(col: usize) -> f32 {
    OFFSET_X + (col as f32 + 0.5) * CELL_SIZE
}
fn is_falling(query: Query<&FallingPiece>) -> bool {
    !query.is_empty()
}

fn is_game_over(board: Res<Board>) -> bool {
    board.game_over
}

fn mouse_click_system(
    mut messages: MessageReader<SpawnPieceEvent>,
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
            Transform::from_xyz(col_to_x(message.col), start_y, 1.0),
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

// 标记正在下落的棋子，包含下落目标信息
#[derive(Component)]
struct FallingPiece {
    target_y: f32, // 目标 Y 坐标
    player: Piece, // 棋子颜色
    row: usize,    // 目标行
    col: usize,    // 目标列
}

// 下落动画系统：移动下落中的棋子，到达目标后更新棋盘
fn falling_animation_system(
    time: Res<Time>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &FallingPiece)>,
) {
    for (entity, mut transform, falling) in query.iter_mut() {
        // 向下移动delta_secs
        transform.translation.y -= FALL_SPEED * time.delta_secs();

        // 检查是否到达或超过目标
        if transform.translation.y <= falling.target_y {
            transform.translation.y = falling.target_y; // 精确对齐

            // 更新棋盘状态
            board.grid[falling.row][falling.col] = falling.player;

            // 检查胜利/平局
            if board.check_win(falling.row, falling.col) {
                board.game_over = true;
                board.winner = Some(falling.player);
            } else if board.is_full() {
                board.game_over = true;
                board.winner = None;
            } else {
                board.switch_player();
            }

            // 移除下落组件（棋子变为静止）
            commands.entity(entity).remove::<FallingPiece>();
        }
    }
}
fn reset_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    pieces: Query<Entity, With<PieceSprite>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // 删除所有棋子实体
        for entity in pieces.iter() {
            commands.entity(entity).despawn();
        }
        // 重置棋盘
        *board = Board::new();
    }
}
