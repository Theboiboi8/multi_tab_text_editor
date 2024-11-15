use std::path::PathBuf;

use iced::{Element, Font, Length, Pixels};
use iced::widget::svg::Handle;
use iced::widget::{svg, text};

use crate::Message;

pub fn new_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F37D}', size)
}

pub fn save_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F7D8}', size)
}

pub fn save_as_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F30A}', size)
}

pub fn open_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F392}', size)
}

pub fn close_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F659}', size)
}

pub fn info_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F646}', size)
}

pub fn git_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F69D}', size)
}

pub fn external_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F144}', size)
}

pub fn eye_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F341}', size)
}

pub fn settings_icon<'a>(size: impl Into<Pixels>) -> Element<'a, Message> {
	icon('\u{F3E5}', size)
}

fn icon<'a>(codepoint: char, size: impl Into<Pixels>) -> Element<'a, Message> {
	const ICON_FONT: Font = Font::with_name("bootstrap-icons");

	text(codepoint)
		.font(ICON_FONT)
		.size(size)
		.into()
}

fn svg_icon<'a, P, S>(svg_path: P, size: S) -> Element<'a, Message>
where
	P: Into<PathBuf>,
	S: Into<IconSize>
{
	let size = size.into();
	
	svg(Handle::from_path(svg_path))
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
		IconSize([value.0.into(), value.1.into()])
	}
}