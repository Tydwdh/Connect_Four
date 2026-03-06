use super::*;

/// Board 插件：负责棋盘显示、棋子放置预览和列高亮功能
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighlightColumn>()
            .add_systems(Startup, setup_board)
            .add_systems(
                Update,
                (
                    mouse_highlight_system,
                    update_highlight_system,
                    update_preview_system,
                )
                    .chain(),
            );
    }
}

// ===================================
// 棋子相关组件和资源定义
// ===================================

/// 棋子实体 Bundle
#[derive(Bundle)]
pub struct PieceBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    sprite: PieceSprite,
}

impl PieceBundle {
    pub fn new(mesh: Handle<Mesh>, material: Handle<ColorMaterial>) -> Self {
        Self {
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(material),
            sprite: PieceSprite,
        }
    }
}

/// 存储棋子资源（网格和材质）
#[derive(Resource)]
pub struct PieceAssets {
    pub mesh: Handle<Mesh>, // 所有棋子共用相同的圆形网格
    pub player1_material: Handle<ColorMaterial>,
    pub player2_material: Handle<ColorMaterial>,
}

/// 棋子精灵组件标识
#[derive(Component)]
pub struct PieceSprite;

// ===================================
// 高亮列相关组件和资源定义
// ===================================

/// 存储高亮列实体 ID 的资源
#[derive(Resource)]
pub struct HighlightColumnEntity(pub Entity);

/// 高亮列精灵组件标识
#[derive(Component)]
pub struct ColumnHighlight;

/// 存储当前高亮列索引的资源
#[derive(Resource, Default)]
pub struct HighlightColumn(pub Option<usize>);

/// 预览棋子组件标识
#[derive(Component)]
pub struct PreviewPiece;

// ===================================
// 棋盘初始化系统
// ===================================

/// 设置棋盘，包括底板、网格、高亮指示器和棋子资源
pub fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let board_width = CELL_SIZE * COLS as f32;
    let board_height = CELL_SIZE * ROWS as f32;
    let offset_x = -board_width / 2.0;
    let offset_y = -board_height / 2.0;
    let line_color = Color::srgba(0.9, 0.8, 0.6, 1.0); // 金色线条
    let bg_color = Color::srgb(0.3, 0.2, 0.1); // 深木色底板
    let cell_color = Color::srgb(0.5, 0.35, 0.2); // 单元格浅木色
    let empty_slot_color = Color::srgba(0.8, 0.8, 0.8, 0.3); // 半透灰

    // 1. 底板（大矩形）
    commands.spawn((
        Sprite {
            color: bg_color,
            custom_size: Some(Vec2::new(board_width + 20.0, board_height + 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 2. 单元格背景 + 空位圆形标记
    // 预先创建圆形 mesh 和材质（所有格子共用）
    let circle_radius = CELL_SIZE * 0.35;
    let circle_mesh = meshes.add(Circle::new(circle_radius));
    let circle_material = materials.add(ColorMaterial::from(empty_slot_color));

    for row in 0..ROWS {
        for col in 0..COLS {
            let x = offset_x + col as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            let y = offset_y + row as f32 * CELL_SIZE + CELL_SIZE / 2.0;

            // 单元格背景
            commands.spawn((
                Sprite {
                    color: cell_color,
                    custom_size: Some(Vec2::new(CELL_SIZE - 4.0, CELL_SIZE - 4.0)),
                    ..default()
                },
                Transform::from_xyz(x, y, 1.0),
            ));

            commands.spawn((
                Transform::from_xyz(x, y, 2.0),
                Mesh2d(circle_mesh.clone()),
                MeshMaterial2d(circle_material.clone()),
            ));
        }
    }

    // 3. 网格线（垂直线）
    let thickness = 2.0;
    for col in 0..=COLS {
        let x = offset_x + col as f32 * CELL_SIZE;
        commands.spawn((
            Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(thickness, board_height)),
                ..default()
            },
            Transform::from_xyz(x, 0.0, 2.0),
        ));
    }
    // 水平线
    for row in 0..=ROWS {
        let y = offset_y + row as f32 * CELL_SIZE;
        commands.spawn((
            Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(board_width, thickness)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 2.0),
        ));
    }

    // 4. 高亮列指示器
    let highlight_color = Color::srgba(255.0, 255.0, 255.0, 0.2);
    let highlight_height = board_height;
    let highlight_width = CELL_SIZE - 4.0; // 略小于格子宽度，避免遮挡边框
    let highlight_entity = commands
        .spawn((
            Sprite {
                color: highlight_color,
                custom_size: Some(Vec2::new(highlight_width, highlight_height)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 5.0), // 位置无关紧要，因为隐藏
            Visibility::Hidden,                 // 初始隐藏
            ColumnHighlight,
        ))
        .id();
    commands.insert_resource(HighlightColumnEntity(highlight_entity));

    let mesh = meshes.add(Circle::new(CELL_SIZE * 0.35));

    // 创建两种颜色的材质
    let player2_material = materials.add(Piece::Red.color()); // 假设玩家2为红色
    let player1_material = materials.add(Piece::Yellow.color()); // 玩家1为黄色
    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(player2_material.clone()),
        Transform::from_xyz(0.0, 0.0, 3.0), // Z 轴介于棋子(3)和高亮(5)之间
        Visibility::Hidden,
        PreviewPiece,
        PieceSprite,
    ));
    commands.insert_resource(PieceAssets {
        mesh,
        player1_material,
        player2_material,
    });
}

// ===================================
// 更新系统
// ===================================

/// 更新高亮列显示系统
/// 根据当前高亮列资源更新高亮精灵的位置和可见性
pub fn update_highlight_system(
    highlight_col: Res<HighlightColumn>,
    highlight_entity: Res<HighlightColumnEntity>,
    mut transforms: Query<&mut Transform, With<ColumnHighlight>>,
    mut visibilities: Query<&mut Visibility, With<ColumnHighlight>>,
) {
    let mut transform = transforms.get_mut(highlight_entity.0).unwrap();
    let mut visibility = visibilities.get_mut(highlight_entity.0).unwrap();
    if let Some(col) = highlight_col.0 {
        // 计算该列中心 x 坐标
        let x = OFFSET_X + col as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        transform.translation.x = x;
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}

/// 鼠标高亮系统
/// 检测鼠标位置并更新高亮列资源
pub fn mouse_highlight_system(
    mut cursor_events: MessageReader<CursorMoved>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut highlight_col: ResMut<HighlightColumn>,
) {
    // 获取最后一个鼠标移动事件
    let cursor_pos = if let Some(event) = cursor_events.read().last() {
        event.position
    } else {
        return;
    };

    // 获取摄像机
    let (camera, camera_transform) = camera_q.single().unwrap();

    // 将屏幕坐标转换为世界坐标
    let world_pos = camera
        .viewport_to_world_2d(camera_transform, cursor_pos)
        .unwrap();

    // 棋盘参数（应与 setup_board 中一致）
    let cell_size = 80.0;
    let board_width = cell_size * COLS as f32;
    let board_height = cell_size * ROWS as f32;
    let offset_x = -board_width / 2.0;
    let offset_y = -board_height / 2.0;

    // 判断是否在棋盘区域内（y 方向）
    if world_pos.y >= offset_y && world_pos.y <= offset_y + board_height {
        // 计算列索引
        let col_f = (world_pos.x - offset_x) / cell_size;
        if col_f >= 0.0 && col_f < COLS as f32 {
            let col = col_f as usize;
            *highlight_col = HighlightColumn(Some(col));
            return;
        }
    }

    // 鼠标不在棋盘内，清除高亮
    *highlight_col = HighlightColumn(None);
}

/// 更新预览棋子系统
/// 根据当前高亮列和游戏状态更新预览棋子的位置、颜色和可见性
pub fn update_preview_system(
    highlight_col: Res<HighlightColumn>,
    board: Res<Board>,
    mut preview_query: Query<
        (
            &mut Transform,
            &mut Visibility,
            &mut MeshMaterial2d<ColorMaterial>,
        ),
        With<PreviewPiece>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&FallingPiece>,
) {
    let (mut transform, mut visibility, mut material_handle) = preview_query.single_mut().unwrap();

    if let Some(col) = highlight_col.0
        && query.is_empty()
        && !board.game_over
    {
        // 检查该列是否有空行
        if let Some(row) = board.find_empty_row(col) {
            // 计算预览棋子的世界坐标
            let x = col_to_x(col);
            let y = row_to_y(row);
            transform.translation = Vec3::new(x, y, 4.0);

            // 更新材质颜色为当前玩家颜色，并设置半透明
            let player_color = board.current_player.color();

            *material_handle = MeshMaterial2d(materials.add(player_color));

            *visibility = Visibility::Visible;
        } else {
            // 该列已满，隐藏预览
            *visibility = Visibility::Hidden;
        }
    } else {
        // 鼠标不在棋盘上，隐藏预览
        *visibility = Visibility::Hidden;
    }
}
