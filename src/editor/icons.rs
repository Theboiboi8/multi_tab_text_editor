use crate::Message;
use iced::widget::svg::Handle;
use iced::widget::svg;
use iced::{Element, Pixels, Theme};
use include_dir::{include_dir, Dir};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub static ICON_MAP: LazyLock<HashMap<&str, SvgIcon>> = LazyLock::new(|| {
	HashMap::from([
		("add", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/addFile"), "_dark")),
		("file", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/anyType"), "_dark")),
		("close", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/close"), "_dark")),
		("save_as", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/download"), "_dark")),
		("empty", SvgIcon::single(&PathBuf::from("assets/icons/empty"))),
		("exit", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/exit"), "_dark")),
		("external", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/externalLink"), "_dark")),
		("info", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/infoOutline"), "_dark")),
		("settings", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/inlaySettings"), "_dark")),
		("eye", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/inspectionsEye"), "_dark")),
		("save", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/save"), "_dark")),
		("vcs", SvgIcon::with_dark_suffix(&PathBuf::from("assets/icons/vcs"), "_dark")),
	])
});

const ICONS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/icons");

#[derive(Debug, Clone)]
pub struct SvgIcon {
	pub light: PathBuf,
	pub dark: PathBuf,
	pub dark_suffix: Option<String>,
}

impl SvgIcon {
	pub fn with_dark_suffix(path: &dyn AsRef<Path>, suffix: &str) -> Self {
		let path = path.as_ref();
		
		let mut dark_path = path.to_str().unwrap().to_string();

		dark_path.push_str(suffix);

		Self {
			light: path.with_extension("svg"),
			dark: PathBuf::from(dark_path).with_extension("svg"),
			dark_suffix: Some(suffix.to_string())
		}
	}
	
	pub fn single(path: &dyn AsRef<Path>) -> Self {
		let path = path.as_ref();
		
		Self {
			light: path.to_path_buf().with_extension("svg"),
			dark: path.to_path_buf().with_extension("svg"),
			dark_suffix: None
		}
	}
	
	#[inline]
	pub fn get_variant(&self, theme: &Theme) -> PathBuf {
		if self.dark_suffix.is_none() {
			return self.light.clone()
		}
		
		if crate::config::LIGHT_THEMES.contains(theme) {
			self.light.clone()
		} else {
			self.dark.clone()
		}
	}
}

fn get_icon_by_name(name: &str) -> &SvgIcon {
	ICON_MAP.get(name).unwrap_or_else(|| {
		panic!("Failed to get icon: {name}");
	})
}

fn get_icon_handle_by_name(name: &str, theme: &Theme) -> Handle {
	get_icon_handle(SvgIcon::get_variant(get_icon_by_name(name), theme))
}

fn get_icon_handle(icon_path: impl AsRef<Path>) -> Handle {
	let icon_path = icon_path.as_ref();
	
	let glob = &*format!("**/**/{}", icon_path.to_path_buf().file_name().unwrap().to_str().unwrap());

	let icon = ICONS
		.find(glob)
		.unwrap_or_else(|error| {
			panic!("Failed to match glob pattern \"{glob}\": {error}");
		})
		.last()
		.unwrap_or_else(|| {
			panic!("Failed to find icon: {}", icon_path.display());
		})
		.as_file()
		.unwrap_or_else(|| {
			panic!("Error: Matched glob is not a file");
		});

	Handle::from_memory(icon.contents())
}

pub fn new_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("add", theme), size)
}

pub fn save_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("save", theme), size)
}

pub fn save_as_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("save_as", theme), size)
}

pub fn open_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("file", theme), size)
}

pub fn close_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("close", theme), size)
}

pub fn info_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("info", theme), size)
}

pub fn git_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("vcs", theme), size)
}

pub fn external_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("external", theme), size)
}

pub fn eye_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("eye", theme), size)
}

pub fn settings_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("settings", theme), size)
}

#[allow(dead_code)]
pub fn empty_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("empty", theme), size)
}

pub fn exit_icon<'a>(size: impl Into<IconSize>, theme: &Theme) -> Element<'a, Message> {
	icon(get_icon_handle_by_name("exit", theme), size)
}

fn icon<'a, S>(handle: Handle, size: S) -> Element<'a, Message>
where
	S: Into<IconSize>
{
	let size = size.into();

	svg(handle)
		.width(size.0[0])
		.height(size.0[1])
		.into()
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct IconSize(pub [Pixels; 2]);

impl<T, U> From<(T, U)> for IconSize
where
	T: Into<Pixels>,
	U: Into<Pixels>,
{
	fn from(value: (T, U)) -> Self {
		Self([value.0.into(), value.1.into()])
	}
}

impl From<f32> for IconSize {
	fn from(value: f32) -> Self {
		Self([value.into(), value.into()])
	}
}