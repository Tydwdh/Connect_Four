use bevy::prelude::*;
mod board;
use board::*;
mod four;
use four::*;
mod ui;
use ui::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Board>() // 自动使用 Board::new()
        .add_systems(Startup, (setup_camera, setup_board, setup_ui))
        .add_systems(
            Update,
            (
                mouse_click_system,
                reset_system,
                update_ui,
                falling_animation_system,
            )
                .chain(),
        )
        .run();
}
const BOARD_WIDTH: f32 = CELL_SIZE * COLS as f32;
const BOARD_HEIGHT: f32 = CELL_SIZE * ROWS as f32;
const OFFSET_X: f32 = -BOARD_WIDTH / 2.0; // 棋盘左下角 X
const OFFSET_Y: f32 = -BOARD_HEIGHT / 2.0; // 棋盘左下角 Y
const CELL_SIZE: f32 = 80.0;
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
fn board_to_world(row: usize, col: usize) -> Vec2 {
    let x = OFFSET_X + (col as f32 + 0.5) * CELL_SIZE;
    let y = OFFSET_Y + (row as f32 + 0.5) * CELL_SIZE;
    Vec2::new(x, y)
}

fn col_to_x(col: usize) -> f32 {
    OFFSET_X + (col as f32 + 0.5) * CELL_SIZE
}

fn top_y() -> f32 {
    OFFSET_Y + BOARD_HEIGHT + CELL_SIZE / 2.0 // 棋盘顶部上方半个格子
}
fn mouse_click_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    falling_query: Query<&FallingPiece>, // 如果有棋子正在下落，则忽略点击
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    // 游戏结束或正在下落时不能下子
    if board.game_over || !falling_query.is_empty() {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_q.single().unwrap();
    if let Some(cursor_pos) = window.unwrap().cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // 计算点击的列
            let col = ((world_pos.x - OFFSET_X) / CELL_SIZE).floor() as usize;
            if col >= COLS {
                return;
            }
            let temp_row = ((world_pos.y - OFFSET_Y) / CELL_SIZE).floor() as usize;
            if temp_row >= ROWS {
                return;
            }

            // 查找该列的第一个空行
            if let Some(row) = board.find_empty_row(col) {
                let player = board.current_player;

                // 计算下落起始位置（列顶部上方）和目标位置
                let start_y = board_to_world(temp_row, col).y;
                let target_y = board_to_world(row, col).y;

                // 创建棋子实体（圆形）
                let mesh = meshes.add(Circle::new(CELL_SIZE * 0.35));
                let material = materials.add(player.color());

                commands.spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(material),
                    Transform::from_xyz(col_to_x(col), start_y, 1.0),
                    PieceSprite { row, col },
                    FallingPiece {
                        target_y,
                        player,
                        row,
                        col,
                    },
                ));
                // 注意：此时棋盘尚未更新，等待动画完成后更新
            }
        }
    }
}

fn spawn_piece(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    row: usize,
    col: usize,
    player: Piece,
) {
    let cell_size = 80.0;
    let board_width = cell_size * COLS as f32;
    let board_height = cell_size * ROWS as f32;
    let offset_x = -board_width / 2.0;
    let offset_y = -board_height / 2.0;

    let x = offset_x + (col as f32 + 0.5) * cell_size;
    let y = offset_y + (row as f32 + 0.5) * cell_size;

    let color = match player {
        Piece::Red => Color::srgb(1.0, 0.0, 0.0),
        Piece::Yellow => Color::srgb(1.0, 1.0, 0.0),
        Piece::None => unreachable!(),
    };

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(cell_size * 0.35))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(x, y, 1.0),
        PieceSprite { row, col },
    ));
}
// 标记正在下落的棋子，包含下落目标信息
#[derive(Component)]
struct FallingPiece {
    target_y: f32, // 目标 Y 坐标
    player: Piece, // 棋子颜色
    row: usize,    // 目标行
    col: usize,    // 目标列
}
const FALL_SPEED: f32 = 500.0;
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
