use cursor::Cursor;
use keys::Key;

mod keys;
mod cursor;

static mut FIELD_GUI_EXISTS: bool = false;

pub struct FieldGui {
    width: u32,
    height: u32,
    title: String,
    
}
impl FieldGui {
    pub fn new() -> Self {
        if unsafe {FIELD_GUI_EXISTS} {
            eprintln!("but why do you need two? ping me with your use case")
        }
        todo!()
    }
    pub fn get_cursor() -> Cursor {
        todo!()
    }
    pub fn get_key() -> Key {
        todo!()
    }
}

pub struct GUIDropDown {
    opened: bool,


}