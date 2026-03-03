// 棋盘：6行7列，行0是底部，行5是顶部（方便重力计算）
pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const BOARD_WIDTH: f32 = CELL_SIZE * COLS as f32;
pub const BOARD_HEIGHT: f32 = CELL_SIZE * ROWS as f32;
pub const OFFSET_X: f32 = -BOARD_WIDTH / 2.0; // 棋盘左下角 X
pub const OFFSET_Y: f32 = -BOARD_HEIGHT / 2.0; // 棋盘左下角 Y
pub const CELL_SIZE: f32 = 80.0;
pub const FALL_SPEED: f32 = 500.0;
