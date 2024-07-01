use iced::{Element, Font};
use iced::widget::text;

use crate::Message;

pub fn new_icon<'a>() -> Element<'a, Message> {
	icon('\u{F37D}')
}

pub fn save_icon<'a>() -> Element<'a, Message> {
	icon('\u{F7D8}')
}

pub fn save_as_icon<'a>() -> Element<'a, Message> {
	icon('\u{F30A}')
}

pub fn open_icon<'a>() -> Element<'a, Message> {
	icon('\u{F392}')
}

pub fn close_icon<'a>() -> Element<'a, Message> {
	icon('\u{F659}')
}

fn icon<'a>(codepoint: char) -> Element<'a, Message> {
	const ICON_FONT: Font = Font::with_name("bootstrap-icons");

	text(codepoint)
		.font(ICON_FONT)
		.size(18)
		.into()
}