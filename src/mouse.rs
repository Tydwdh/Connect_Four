use super::*;
#[derive(Message)]
pub struct SpawnPieceEvent {
    pub col: usize,
    pub row: usize,
    pub player: Piece,
    pub world_pos_y: f32,
}

pub fn detect_board_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    board: Res<Board>,
    mut messages: MessageWriter<SpawnPieceEvent>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera_q.single().unwrap();
    if let Some(cursor_pos) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
    {
        let col = ((world_pos.x - OFFSET_X) / CELL_SIZE).floor();
        if !(0.0..=COLS as f32).contains(&col) {
            return;
        }
        let col = col as usize;

        if let Some(row) = board.find_empty_row(col) {
            let row_temp = ((world_pos.y - OFFSET_Y) / CELL_SIZE).floor();
            if !(row as f32..ROWS as f32).contains(&row_temp) {
                return;
            }
            messages.write(SpawnPieceEvent {
                col,
                row,
                player: board.current_player,
                world_pos_y: world_pos.y,
            });
        }
    }
}
