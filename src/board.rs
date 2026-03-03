use super::*;
#[derive(Component)]
pub struct PieceSprite {
    pub row: usize,
    pub col: usize,
}

pub fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 绘制网格线（简单起见，画一些线段）
    let line_color = Color::WHITE;
    let thickness = 2.0;
    let cell_size = 80.0; // 每个格子像素大小
    let board_width = cell_size * COLS as f32;
    let board_height = cell_size * ROWS as f32;
    let offset_x = -board_width / 2.0;
    let offset_y = -board_height / 2.0;

    // 垂直线
    for col in 0..=COLS {
        let x = offset_x + col as f32 * cell_size;
        commands.spawn((
            Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(thickness, board_height)),
                ..default()
            },
            Transform::from_xyz(x, 0.0, 0.0),
        ));
    }
    // 水平线
    for row in 0..=ROWS {
        let y = offset_y + row as f32 * cell_size;
        commands.spawn((
            Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(board_width, thickness)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.0),
        ));
    }
}
