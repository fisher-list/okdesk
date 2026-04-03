use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u16),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputEvent {
    MouseMove { x: f64, y: f64 },
    MouseButton { button: MouseButton, pressed: bool },
    Key { key: String, pressed: bool },
    Scroll { dx: i32, dy: i32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalMessage {
    Register { id: String },
    Offer { from: String, to: String, sdp: String, password: Option<String> },
    Answer { from: String, to: String, sdp: String },
    IceCandidate { from: String, to: String, candidate: String },
    Error { message: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileCommand {
    List { path: String },
    Download { path: String },
    Upload { path: String, total_size: u64 },
    Delete { path: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChunk {
    pub path: String,
    pub offset: u64,
    pub data: Vec<u8>,
    pub is_last: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileResponse {
    ListResult { path: String, entries: Vec<FileInfo> },
    DownloadStart { path: String, total_size: u64 },
    UploadAck { path: String, offset: u64 },
    Success { message: String },
    Error { message: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ControlMessage {
    Input(InputEvent),
    ClipboardSync { data: String },
    FileCommand(FileCommand),
    FileChunk(FileChunk),
    FileResponse(FileResponse),
    ScreenList(Vec<ScreenInfo>),
    SwitchScreen { id: u32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScreenInfo {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub is_primary: bool,
}
