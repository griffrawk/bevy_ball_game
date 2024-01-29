use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const BUTTON_STYLE: Style = {
    // https://bevyengine.org/learn/migration-guides/0.10-0.11/#add-css-grid-support-to-bevy-ui
    // https://bevyengine.org/learn/migration-guides/0.10-0.11/#flatten-ui-style-properties-that-use-size-remove-size
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(600.0);
    style.height = Val::Px(120.0);
    style
};

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
        // Another 0.11 change. Use width, height instead of size, and
        // column_gap / row_gap instead of gap
        // https://bevyengine.org/learn/migration-guides/0.10-0.11/#flatten-ui-style-properties-that-use-size-remove-size
        // All of the available...
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.column_gap = Val::Px(8.0);
    style.row_gap = Val::Px(8.0);
    style
};

// This one is different as it needs the asset server, so it's a function
pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::GOLD,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::GOLD,
    }
}
