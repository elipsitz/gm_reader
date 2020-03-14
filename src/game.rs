#[derive(Debug)]
pub enum Version {
    Unknown = 0,
    Gm530 = 530,
    Gm600 = 600,
    Gm700 = 700,
    Gm800 = 800,
    Gm810 = 810,
}

impl Default for Version {
    fn default() -> Self {
        Version::Unknown
    }
}

#[derive(Default)]
pub struct Game {
    pub version: Version,
    pub debug: bool,
    pub pro: bool,
    pub game_id: u32,
    pub guid: [u32; 4],

    pub settings: Settings,
    pub help: Help,
    pub triggers: Vec<Trigger>,
    pub constants: Vec<Constant>,
    pub sprites: Vec<Sprite>,
    pub sounds: Vec<Sound>,
    pub backgrounds: Vec<Background>,
    pub paths: Vec<Path>,
    pub scripts: Vec<Script>,
    pub fonts: Vec<Font>,
    pub timelines: Vec<Timeline>,
    pub objects: Vec<Object>,
    pub rooms: Vec<Room>,
    pub includes: Vec<Include>,

    pub room_order: Vec<u32>,
}

#[derive(Default, Debug)]
pub struct Settings {
    pub fullscreen: bool,
    pub interpolation: bool,
    pub hide_border: bool,
    pub show_cursor: bool,
    pub scaling: i32,
    pub resizable: bool,
    pub always_on_top: bool,
    pub background_color: u32,

    pub set_resolution: bool,
    pub color_depth: u32,
    pub resolution: u32,
    pub frequency: u32,
    pub hide_buttons: bool,
    pub vsync: bool,
    pub disable_screensaver: bool,

    pub default_f4: bool,
    pub default_f1: bool,
    pub default_esc: bool,
    pub default_f5: bool,
    pub default_f9: bool,
    pub close_as_esc: bool,
    pub priority: u32,
    pub freeze: bool,

    pub loading_bar: u32,
    pub loading_bar_back: Option<Vec<u8>>,
    pub loading_bar_front: Option<Vec<u8>>,
    pub loading_background: Option<Vec<u8>>,

    pub load_transparent: bool,
    pub load_alpha: u32,
    pub load_scale: bool,

    pub error_display: bool,
    pub error_log: bool,
    pub error_abort: bool,
    pub uninitialized_zero: bool,
    pub uninitialized_arguments_error: bool,
}

#[derive(Default, Debug)]
pub struct Trigger {
    pub id: u32,
    pub name: String,
    pub condition: String,
    pub check_moment: u32,
    pub constant_name: String,
}

#[derive(Default, Debug)]
pub struct Constant {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug)]
pub struct Sound {
    pub id: u32,
    pub name: String,
    pub kind: u32,
    pub filetype: String,
    pub filename: String,
    pub data: Vec<u8>,
    pub effects: u32,
    pub volume: f64,
    pub pan: f64,
    pub preload: bool,
}

#[derive(Default, Debug)]
pub struct Sprite {
    pub id: u32,
    pub name: String,
    pub origin: (i32, i32),

    pub frames: Vec<SpriteFrame>,
    pub masks: Vec<SpriteMask>,
}

#[derive(Default, Debug)]
pub struct SpriteFrame {
    pub size: (u32, u32),
    pub data: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct SpriteMask {
    pub size: (u32, u32),
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
    pub top: i32,
    pub data: Vec<bool>,
}

#[derive(Default, Debug)]
pub struct Background {
    pub id: u32,
    pub name: String,
    pub size: (u32, u32),
    pub data: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct Path {
    pub id: u32,
    pub name: String,
    pub connection_type: u32,
    pub closed: bool,
    pub precision: u32,
    pub points: Vec<PathPoint>,
}

#[derive(Default, Debug)]
pub struct PathPoint {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
}

#[derive(Default, Debug)]
pub struct Script {
    pub id: u32,
    pub name: String,
    pub script: String,
}

#[derive(Default)]
pub struct Font {
    pub id: u32,
    pub name: String,
    pub font_name: String,
    pub size: u32,
    pub bold: bool,
    pub italic: bool,
    pub range_start: u32,
    pub range_end: u32,
    pub charset: u32,
    pub aa_level: u32,
    pub atlas: FontAtlas,
}

pub struct FontAtlas {
    pub glyphs: [FontAtlasGlyph; 256],
    pub size: (u32, u32),
    pub data: Vec<u8>,
}

impl Default for FontAtlas {
    fn default() -> Self {
        FontAtlas {
            glyphs: [FontAtlasGlyph::default(); 256],
            size: (0, 0),
            data: vec![],
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct FontAtlasGlyph {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub horizontal_advance: i32,
    pub kerning: i32,
}

#[derive(Default, Debug)]
pub struct Action {
    pub library_id: u32,
    pub action_id: u32,
    pub action_kind: u32,
    pub has_relative: bool,
    pub is_question: bool,
    pub has_target: bool,
    pub action_type: u32,
    pub name: String,
    pub code: String,
    pub parameters_used: u32,
    pub parameters: Vec<u32>,
    pub target: i32,
    pub relative: bool,
    pub arguments: Vec<String>,
    pub negate: bool,
}

#[derive(Default, Debug)]
pub struct Timeline {
    pub id: u32,
    pub name: String,
    pub moments: Vec<TimelineMoment>,
}

#[derive(Default, Debug)]
pub struct TimelineMoment {
    pub position: u32,
    pub actions: Vec<Action>,
}

#[derive(Default, Debug)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub sprite: i32,
    pub solid: bool,
    pub visible: bool,
    pub depth: i32,
    pub persistent: bool,
    pub parent: i32,
    pub mask: i32,
    pub events: Vec<ObjectEvent>,
}

#[derive(Default, Debug)]
pub struct ObjectEvent {
    pub event_type: u32,
    pub event_number: i32,
    pub actions: Vec<Action>,
}

#[derive(Default, Debug)]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub caption: String,
    pub width: u32,
    pub height: u32,
    pub speed: u32,
    pub persistent: bool,
    pub clear_color: u32,
    pub clear: bool,
    pub creation_code: String,
    pub enable_views: bool,
    pub backgrounds: Vec<RoomBackground>,
    pub views: Vec<RoomView>,
    pub instances: Vec<RoomInstance>,
    pub tiles: Vec<RoomTile>,
}

#[derive(Default, Debug)]
pub struct RoomBackground {
    pub visible: bool,
    pub foreground: bool,
    pub background: i32,
    pub x: i32,
    pub y: i32,
    pub tile_h: bool,
    pub tile_v: bool,
    pub h_speed: i32,
    pub v_speed: i32,
    pub stretch: bool,
}

#[derive(Default, Debug)]
pub struct RoomView {
    pub visible: bool,
    pub view_x: u32,
    pub view_y: u32,
    pub view_width: u32,
    pub view_height: u32,
    pub port_x: u32,
    pub port_y: u32,
    pub port_width: u32,
    pub port_height: u32,
    pub h_border: u32,
    pub v_border: u32,
    pub h_speed: i32,
    pub v_speed: i32,
    pub target_object: i32,
}

#[derive(Default, Debug)]
pub struct RoomInstance {
    pub x: i32,
    pub y: i32,
    pub object: i32,
    pub id: i32,
    pub creation_code: String,
}

#[derive(Default, Debug)]
pub struct RoomTile {
    pub x: i32,
    pub y: i32,
    pub background: i32,
    pub tile_x: i32,
    pub tile_y: i32,
    pub width: u32,
    pub height: u32,
    pub depth: i32,
    pub id: i32,
}

#[derive(Default, Debug)]
pub struct Include {
    pub name: String,
    pub original_path: String,
    pub original_chosen: bool,
    pub original_size: u32,
    pub store_in_editable: bool,
    pub data: Vec<u8>,
    pub export: u32,
    pub export_folder: String,
    pub overwrite: bool,
    pub free_memory: bool,
    pub remove_at_end: bool,
}

#[derive(Default, Debug)]
pub struct Help {
    pub background_color: u32,
    pub separate_window: bool,
    pub caption: String,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub show_border: bool,
    pub allow_resize: bool,
    pub always_on_top: bool,
    pub freeze_game: bool,
    pub content: String,
}