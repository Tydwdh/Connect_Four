use super::*;
// 定义棋子颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    None,
    Red,
    Yellow,
}
impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Piece::Red => Color::srgb(1.0, 0.0, 0.0),
            Piece::Yellow => Color::srgb(1.0, 1.0, 0.0),
            Piece::None => unreachable!(),
        }
    }
}

pub struct FourPlugin;

impl Plugin for FourPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_systems(Startup, setup_board);
    }
}

#[derive(Resource)]
pub struct Board {
    pub grid: [[Piece; COLS]; ROWS],
    pub current_player: Piece, // 当前应该下棋的玩家
    pub game_over: bool,
    pub winner: Option<Piece>, // 胜者
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[Piece::None; COLS]; ROWS],
            current_player: Piece::Red,
            game_over: false,
            winner: None,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Piece::None; COLS]; ROWS],
            current_player: Piece::Red,
            game_over: false,
            winner: None,
        }
    }
    pub fn find_empty_row(&self, col: usize) -> Option<usize> {
        if col >= COLS {
            return None;
        }
        (0..ROWS).find(|&row| self.grid[row][col] == Piece::None)
    }

    // 切换玩家
    pub fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Piece::Red => Piece::Yellow,
            Piece::Yellow => Piece::Red,
            Piece::None => Piece::Red, // 不会发生
        };
    }

    // 检查胜利（在(row, col)处落子后）
    pub fn check_win(&self, row: usize, col: usize) -> bool {
        let piece = self.grid[row][col];
        if piece == Piece::None {
            return false;
        }

        // 四个方向向量：水平、垂直、主对角线、副对角线
        let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

        for (dx, dy) in directions {
            let mut count = 1;
            // 正方向延伸
            for step in 1..4 {
                let r = row as isize + dy * step;
                let c = col as isize + dx * step;
                if r < 0 || r >= ROWS as isize || c < 0 || c >= COLS as isize {
                    break;
                }
                if self.grid[r as usize][c as usize] == piece {
                    count += 1;
                } else {
                    break;
                }
            }
            // 负方向延伸
            for step in 1..4 {
                let r = row as isize - dy * step;
                let c = col as isize - dx * step;
                if r < 0 || r >= ROWS as isize || c < 0 || c >= COLS as isize {
                    break;
                }
                if self.grid[r as usize][c as usize] == piece {
                    count += 1;
                } else {
                    break;
                }
            }
            if count >= 4 {
                return true;
            }
        }
        false
    }

    // 检查棋盘是否已满（平局）
    pub fn is_full(&self) -> bool {
        self.grid
            .iter()
            .all(|row| row.iter().all(|&p| p != Piece::None))
    }
}
