use iced::alignment::Horizontal;
use iced::widget::{button, container, row, text, tooltip, Column, ComboBox};
use iced::{Alignment, Border, Element, Length, Theme};
use iced_aw::widgets::InnerBounds;
use iced_aw::{card, quad, style};

use crate::editor::icons;
use crate::{Editor, Message};

pub fn separator(theme: &Theme) -> quad::Quad {
	quad::Quad {
		quad_color: theme.extended_palette().primary.weak.color.into(),
		quad_border: Border {
			radius: 4.0.into(),
			..Default::default()
		},
		inner_bounds: InnerBounds::Ratio(0.99, 0.1),
		height: Length::Fixed(20.0),
		..Default::default()
	}
}

pub fn menubar_button<'a>(
	content: impl Into<Element<'a, Message>>,
	tooltip: Option<&'a str>,
	action: Message,
) -> Element<'a, Message> {
	let inner = button(
		container(content.into())
			.center_x(Length::Shrink)
			.center_y(Length::Shrink)
			.padding([2, 4])
	)
		.style(button::text)
		.on_press(action);

	if let Some(tooltip_label) = tooltip {
		iced::widget::tooltip(
			inner,
			tooltip_label,
			tooltip::Position::Bottom,
		)
			.into()
	} else {
		inner.into()
	}
}

pub fn menu_button<'a>(
	content: impl Into<Element<'a, Message>>,
	action: Message,
) -> Element<'a, Message> {
	let inner = button(
		container(content.into())
			.width(Length::Fill)
			.align_x(Horizontal::Left)
			.center_y(Length::Shrink)
			.padding([2, 4])
	)
		.on_press(action)
		.style(button::text);

	inner.into()
}

pub fn menu_button_disabled<'a>(
	content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
	let inner = button(
		container(content.into())
			.width(Length::Fill)
			.align_x(Horizontal::Left)
			.center_y(Length::Shrink)
			.padding([2, 4])
	)
		.style(button::text);

	inner.into()
}

pub fn tab(
	content: Element<Message>,
	on_press: Message,
	index: usize,
	highlighted: bool,
) -> Element<Message> {
	button(
		container(
			row![
					content,
					button(icons::close_icon(16))
						.width(Length::Shrink)
						.on_press(Message::CloseIndex(index))
				]
				.align_y(Alignment::Center)
		)
			.width(128)
			.align_x(Horizontal::Center)
			.center_y(Length::Shrink)
	)
		.style(if highlighted {
			button::primary
		} else {
			button::text
		})
		.on_press(on_press)
		.padding([5, 10])
		.into()
}

pub fn icon_text<'a>(label: &(impl ToString + ?Sized)) -> Element<'a, Message> {
	text(format!("   {}", label.to_string())).into()
}

pub fn about_modal<'a>(theme: &Theme) -> Element<'a, Message> {
	card(
		row![
			text("About")
				.width(Length::Fill)
				.size(24),
			button(icons::close_icon(16))
				.width(Length::Shrink)
				.on_press(Message::HideModal)
		].align_y(Alignment::Center),
		Column::new()
			.push(text("Multi Tab Text Editor"))
			.push(text("A text editor that supports syntax \
			highlighting and multiple files open at once."))
			.push(separator(theme))
			.push(text("Created by TanchevK"))
			.push(text("Build using Rust"))
			.push(separator(theme))
			.push(row![
				text("Source code is available on GitHub "),
				button(
					row!["here", icons::external_icon(13)]
						.align_y(Alignment::Center)
				)
					.style(button::text)
					.padding(0)
					.height(Length::Shrink)
					.on_press(Message::OpenURL("https://github.com/tanchevk/multi_tab_text_editor"))
			])
	)
		.style(style::card::dark)
		.width(640)
		.height(360)
		.into()
}

pub fn settings_modal(state: &Editor) -> Element<Message> {
	card(
		row![
			text("Settings")
				.width(Length::Fill)
				.size(24),
			button(icons::close_icon(16))
				.style(button::text)
				.width(Length::Shrink)
				.on_press(Message::HideModal)
		].align_y(Alignment::Center),
		Column::new()
			.push(text("Selected theme"))
			.push(ComboBox::new(
				&state.themes,
				"Select a theme",
				Some(&state.theme),
				Message::SelectTheme
			))
			.push(separator(&state.theme))
			.push(text("Selected syntax highlighting theme"))
			.push(ComboBox::new(
				&state.highlighter_themes,
				"Select a highlighting theme",
				Some(&state.highlighter_theme),
				Message::SelectSyntaxTheme
			))
			.push(separator(&state.theme))
			.width(600)
	)
		.style(style::card::dark)
		.width(640)
		.height(360)
		.into()
}
