use std::path::PathBuf;

use include_dir::{include_dir, Dir};

use crate::{icon_conversion::IconResolution, options::ColorScheme};

static RESOURCES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/resources");

pub fn get_folder_icon(color_scheme: ColorScheme, resolution: &IconResolution) -> &'static [u8] {
    let mut path = PathBuf::new();
    path.push(match color_scheme {
        ColorScheme::Light => "GenericFolderIcon.BigSur.iconset",
        ColorScheme::Dark => "GenericFolderIcon.BigSur.dark.iconset",
    });
    path.push(resolution.icon_file());
    RESOURCES_DIR.get_file(&path).unwrap().contents()
}
