use super::*;

#[derive(Message)]
pub struct SpawnPieceMessage {
    pub col: usize,
    pub row: usize,
    pub player: Piece,
    pub world_pos_y: f32,
}
pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnPieceMessage>()
            .add_message::<ClearBoardMessage>()
            .add_systems(
                Update,
                (
                    detect_board_click.run_if(not(is_falling).and(not(is_game_over))),
                    detect_keyboard,
                ),
            );
    }
}

pub fn detect_board_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    board: Res<Board>,
    mut messages: MessageWriter<SpawnPieceMessage>,
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
            messages.write(SpawnPieceMessage {
                col,
                row,
                player: board.current_player,
                world_pos_y: world_pos.y,
            });
            info!("发送 SpawnPieceEvent: col={}, row={}", col, row);
        }
    }
}

#[derive(Message)]
pub struct ClearBoardMessage;
pub fn detect_keyboard(
    key_button: Res<ButtonInput<KeyCode>>,
    mut messages: MessageWriter<ClearBoardMessage>,
) {
    if key_button.just_pressed(KeyCode::Space) {
        messages.write(ClearBoardMessage);
    }
}
