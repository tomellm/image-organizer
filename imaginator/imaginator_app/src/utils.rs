use std::cmp::Ordering;

pub fn dir() -> String {
    std::env::var("ROOT_DIR").expect("The ROOT_DIR environment variable is not present")
}

pub fn images_dir() -> String {
    format!(
        "{}{}/",
        dir(),
        std::env::var("IMAGES_DIR").expect("The IMAGES_DIR environment variable is not present.")
    )
}

pub fn images_dir_with_file(file: &str) -> String {
    format!("{}{file}", images_dir())
}

pub fn user_files() -> String {
    format!(
        "{}/imaginator{}/",
        dir(),
        std::env::var("USER_FILES").expect("The USER_FILES environment variable is not present.")
    )
}

pub fn user_files_with_file(file: &str) -> String {
    format!("{}{file}", user_files())
}

pub fn scale_down_to_max(width: u32, height: u32) -> (u32, u32) {
    const MAX: u32 = 200u32;
    const MAX_F32: f32 = MAX as f32;
    let width_f32 = width as f32;
    let height_f32 = height as f32;
    match width.cmp(&height) {
        Ordering::Greater => {
            let short_side = (MAX_F32 * (height_f32 / width_f32)).round() as u32;
            (MAX, short_side)
        }
        Ordering::Equal => (MAX, MAX),
        Ordering::Less => {
            let short_side = (MAX_F32 * (width_f32 / height_f32)).round() as u32;
            (short_side, MAX)
        }
    }
}
