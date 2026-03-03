use super::*;
#[derive(Component)]
pub struct TurnText;

pub fn update_ui(board: Res<Board>, mut query: Query<&mut Text, With<TurnText>>) {
    for mut text in query.iter_mut() {
        if board.game_over {
            if let Some(winner) = board.winner {
                text.0 = format!("Winner: {:? }！ Reset by pressing the space bar.", winner);
            } else {
                text.0 = "Draw! Reset by pressing the space bar.".to_string();
            }
        } else {
            let player = match board.current_player {
                Piece::Red => "red",
                Piece::Yellow => "yellow",
                _ => "",
            };
            text.0 = format!("Current player: {}", player);
        }
    }
}

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Current player: Red"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor::default(),
        Node {
            // 样式属性直接在 Node 中设置
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            // width: Val::Px(200.0),
            // height: Val::Px(100.0),
            ..default()
        },
        TurnText,
    ));
}
