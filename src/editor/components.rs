use iced::{Alignment, Background, Border, Element, Length, Theme, theme};
use iced::alignment::Horizontal;
use iced::theme::Button;
use iced::widget::{button, Column, ComboBox, container, row, text, tooltip};
use iced::widget::button::Appearance;
use iced_aw::{card, quad, style};
use iced_aw::widgets::InnerBounds;

use crate::{Editor, Message};
use crate::editor::icons;

pub fn separator(theme: &Theme) -> quad::Quad {
	quad::Quad {
		quad_color: theme.extended_palette().primary.weak.color.into(),
		quad_border: Border {
			radius: [4.0; 4].into(),
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
			.width(Length::Shrink)
			.center_x()
			.center_y()
			.padding([2, 4])
	)
		.on_press(action)
		.style(Button::Text);

	if let Some(tooltip_label) = tooltip {
		iced::widget::tooltip(
			inner,
			tooltip_label,
			tooltip::Position::Bottom,
		)
			.style(theme::Container::Box)
			.into()
	} else {
		inner.into()
	}
}

#[derive(Copy, Clone)]
pub struct MenuButtonStyle;

impl button::StyleSheet for MenuButtonStyle {
	type Style = Theme;

	fn active(&self, style: &Self::Style) -> Appearance {
		let palette = style.extended_palette();

		let appearance = Appearance {
			border: Border::with_radius(2),
			..Appearance::default()
		};

		Appearance {
			text_color: palette.background.base.text,
			..appearance
		}
	}

	fn hovered(&self, style: &Self::Style) -> Appearance {
		let palette = style.extended_palette();

		let active = self.active(style);

		Appearance {
			background: Some(Background::from(palette.background.weak.color)),
			..active
		}
	}

	fn pressed(&self, style: &Self::Style) -> Appearance {
		self.hovered(style)
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
			.center_y()
			.padding([2, 4])
	)
		.on_press(action)
		.style(Button::Custom(Box::new(MenuButtonStyle)));

	inner.into()
}

pub fn menu_button_disabled<'a>(
	content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
	let inner = button(
		container(content.into())
			.width(Length::Fill)
			.align_x(Horizontal::Left)
			.center_y()
			.padding([2, 4])
	)
		.style(Button::Custom(Box::new(MenuButtonStyle)));

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
						.style(Button::Custom(Box::new(MenuButtonStyle)))
						.width(Length::Shrink)
						.on_press(Message::CloseIndex(index))
				]
				.align_items(Alignment::Center)
		)
			.width(128)
			.align_x(Horizontal::Center)
			.center_y()
	)
		.style(if highlighted {
			Button::Primary
		} else {
			Button::Custom(Box::new(MenuButtonStyle))
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
				.style(Button::Custom(Box::new(MenuButtonStyle)))
				.width(Length::Shrink)
				.on_press(Message::HideModal)
		].align_items(Alignment::Center),
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
				button(row!["here", icons::external_icon(13)].align_items(Alignment::Center))
					.style(Button::Text)
					.padding(0)
					.height(Length::Shrink)
					.on_press(Message::OpenURL("https://github.com/tanchevk/multi_tab_text_editor"))
			])
	)
		.style(style::card::CardStyles::Dark)
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
				.style(Button::Custom(Box::new(MenuButtonStyle)))
				.width(Length::Shrink)
				.on_press(Message::HideModal)
		].align_items(Alignment::Center),
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
		.style(style::card::CardStyles::Dark)
		.width(640)
		.height(360)
		.into()
}
