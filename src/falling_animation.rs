use super::*;

pub struct FallingAnimationPlugin;

impl Plugin for FallingAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, falling_animation_system);
    }
}
// 标记正在下落的棋子，包含下落目标信息
#[derive(Component)]
pub struct FallingPiece {
    pub target_y: f32, // 目标 Y 坐标
    pub player: Piece, // 棋子颜色
    pub row: usize,    // 目标行
    pub col: usize,    // 目标列
}

// 下落动画系统：移动下落中的棋子，到达目标后更新棋盘
pub fn falling_animation_system(
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
