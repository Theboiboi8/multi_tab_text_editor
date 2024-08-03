use iced::{Element, Font, Pixels};
use iced::widget::text;

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