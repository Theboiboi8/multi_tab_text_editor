#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use iced::{Alignment, Application, Background, Border, Color, Command, Element, executor, Font, highlighter, Length, Pixels, Settings, Size, Theme, theme, window};
use iced::alignment::Horizontal;
use iced::highlighter::Highlighter;
use iced::theme::Button;
use iced::widget::{button, container, horizontal_space, row, Row, text, text_editor, tooltip};
use iced::widget::button::Appearance;
use iced::widget::column as col;
use iced::window::{Level, Position};
use iced::window::settings::PlatformSpecific;
use iced_aw::{card, menu, menu_bar, menu_items, Modal, quad, style};
use iced_aw::menu::{Item, Menu};
use iced_aw::widgets::InnerBounds;

mod icons;

const UNCUT_SANS: Font = Font::with_name("UncutSans");
const JETBRAINS_MONO: Font = Font::with_name("JetBrainsMono");

fn main() -> iced::Result {
	Editor::run(Settings {
		id: None,
		window: window::Settings {
			size: Size::new(1024.0, 756.0),
			position: Position::default(),
			min_size: Some(Size::new(420.0, 280.0)),
			max_size: None,
			visible: true,
			resizable: true,
			decorations: true,
			transparent: false,
			level: Level::default(),
			icon: None,
			platform_specific: PlatformSpecific::default(),
			exit_on_close_request: true,
		},
		flags: Default::default(),
		fonts: vec![
			include_bytes!("../assets/bootstrap-icons.ttf")
				.as_slice()
				.into(),
			include_bytes!("../assets/JetBrainsMono.ttf")
				.as_slice()
				.into(),
			include_bytes!("../assets/UncutSans.ttf")
				.as_slice()
				.into(),
		],
		default_font: UNCUT_SANS,
		default_text_size: Pixels(13.0),
		antialiasing: true,
	})
}

struct Editor {
	files: Vec<File>,
	current: usize,
	error: Option<Error>,
	about_shown: bool,
	theme: Theme
}

struct File {
	path: Option<PathBuf>,
	content: text_editor::Content,
	is_modified: bool,
}

impl File {
	fn empty() -> Self {
		File {
			path: None,
			content: text_editor::Content::new(),
			is_modified: false,
		}
	}

	fn sample() -> Self {
		let sample = include_bytes!("../src/main.rs").as_slice();

		let content = verify_content(
			String::from_utf8_lossy(sample)
				.to_string()
		).leak();

		File {
			path: None,
			content: text_editor::Content::with_text(content),
			is_modified: true
		}
	}
}

#[derive(Debug, Clone)]
enum Message {
	Edit(text_editor::Action),
	New,
	Open,
	FileOpened(Result<(PathBuf, Arc<String>), Error>),
	Save,
	SaveAs,
	FileSaved(Result<PathBuf, Error>),
	Close,
	CloseIndex(usize),
	SelectFile(usize),
	OpenURL(&'static str),
	ShowAbout,
	HideAbout,
	None
}

impl Application for Editor {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = ();

	fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
		(
			Self {
				files: vec![File::sample()],
				error: None,
				current: 0,
				about_shown: false,
				theme: Theme::GruvboxDark
			},
			Command::none()
		)
	}

	fn title(&self) -> String {
		format!(
			"{} v{}",
			env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_VERSION")
		)
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Edit(action) => {
				self.files[self.current].is_modified = self.files[self.current].is_modified || action.is_edit();
				self.error = None;

				self.files[self.current].content.perform(action);

				Command::none()
			}
			Message::Open => {
				Command::perform(pick_file(), Message::FileOpened)
			}
			Message::FileOpened(Ok((path, content))) => {
				self.files.push(File::empty());

				self.current = self.files.len() - 1;

				self.files[self.current].path = Some(path);
				self.files[self.current].content = text_editor::Content::with_text(&content);

				Command::none()
			}
			Message::FileOpened(Err(error)) | Message::FileSaved(Err(error)) => {
				self.error = Some(error);

				Command::none()
			}
			Message::New => {
				self.files.push(File::empty());

				self.current = self.files.len() - 1;

				Command::none()
			}
			Message::Save => {
				let text = self.files[self.current].content.text();

				Command::perform(
					save_file(self.files[self.current].path.clone(), text), Message::FileSaved
				)
			}
			Message::SaveAs => {
				let text = self.files[self.current].content.text();

				Command::perform(save_file(None, text), Message::FileSaved)
			}
			Message::FileSaved(Ok(path)) => {
				self.files[self.current].path = Some(path);
				self.files[self.current].is_modified = false;

				Command::none()
			}
			Message::Close => {
				let mut should_remove = true;
				let remove: usize = self.current;

				if self.current != 0 {
					self.current = 0;
				} else if self.files.len() == 1 {
					should_remove = false;
				} else {
					self.current = self.files.len() - 2;
				}

				if should_remove {
					self.files.remove(remove);
				} else {
					self.files[self.current] = File::empty();
				}

				Command::none()
			}
			Message::CloseIndex(index) => {
				let mut should_remove = true;

				if self.current != index {
					self.current = 0;
				} else if self.files.len() == 1 {
					should_remove = false;
				} else {
					self.current = self.files.len() - 2;
				}

				if should_remove {
					self.files.remove(index);
				} else {
					self.files[self.current] = File::empty();
				}

				Command::none()
			}
			Message::SelectFile(index) => {
				self.current = index;

				Command::none()
			}
			Message::OpenURL(url) => {
				if open::that(url).is_err() {
					eprintln!("Failed to open url {url}");
				};

				Command::none()
			}
			Message::ShowAbout => {
				self.about_shown = true;

				Command::none()
			}
			Message::HideAbout => {
				self.about_shown = false;

				Command::none()
			}
			Message::None => {
				Command::none()
			}
		}
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self) -> Element<'_, Self::Message> {
		let card = if self.about_shown {
			Some(
				card(
					row![
						text("About")
							.width(Length::Fill)
							.size(24),
						button(icons::close_icon(16))
							.style(Button::Custom(Box::new(MenuButtonStyle)))
							.width(Length::Shrink)
							.on_press(Message::HideAbout)
					].align_items(Alignment::Center),
					col![
						text("Multi Tab Text Editor"),
						text("A text editor that supports syntax \
						highlighting and multiple files open at once."),
						separator(),
						text("Created by Theboiboi8"),
						text("Build using Rust"),
						separator(),
						row![
							text("Source code is available on GitHub "),
							button(row!["here", icons::external_icon(13)].align_items(Alignment::Center))
								.style(theme::Button::Text)
								.padding(0)
								.height(Length::Shrink)
								.on_press(Message::OpenURL("https://github.com/Theboiboi8/multi_tab_text_editor"))
						]
					]
				)
					.style(style::card::CardStyles::Secondary)
					.width(640)
					.height(360)
			)
		} else {
			None
		};

		//let menu_tpl_1 = |items| Menu::new(items)
		//	.max_width(180.0)
		//	.offset(15.0)
		//	.spacing(5.0);
		let menu_tpl_2 = |items| Menu::new(items)
			.max_width(180.0)
			.offset(0.0)
			.spacing(5.0);

		let menu_bar = menu_bar![
			(menubar_button(text("File"), None, Message::None), {
				let sub_menu = menu_tpl_2(menu_items![
					(menu_button(
						row![
							icons::new_icon(12),
							text("   New"),
						].align_items(Alignment::Center),
						Message::New
					))
					(menu_button(
						row![
							icons::open_icon(12),
							text("   Open..."),
						].align_items(Alignment::Center),
						Message::Open
					))
					(menu_button(
						row![
							icons::save_icon(12),
							text("   Save"),
						].align_items(Alignment::Center),
						Message::Save
					))
					(menu_button(
						row![
							icons::save_as_icon(12),
							text("   Save As"),
						].align_items(Alignment::Center),
						Message::SaveAs
					))
					(menu_button(
						row![
							icons::close_icon(12),
							text("   Close"),
						].align_items(Alignment::Center),
						Message::Close
					))
				]).width(180.0);

				sub_menu
			})

			(menubar_button(text("Help"), None, Message::None), {
				let sub_menu = menu_tpl_2(menu_items![
					(menu_button(
						row![
							icons::info_icon(12),
							text("   About"),
						].align_items(Alignment::Center),
						Message::ShowAbout
					))
					(menu_button(
						row![
							icons::git_icon(12),
							text("   Source"),
						].align_items(Alignment::Center),
						Message::OpenURL("https://github.com/Theboiboi8/multi_tab_text_editor")
					))
				]).width(180.0);

				sub_menu
			})
		].draw_path(menu::DrawPath::Backdrop);

		let mut tabs = Vec::new();

		for (index, file) in self.files.iter().enumerate() {
			tabs.push(tab(
				text(format!(
					"{}{}",
					match &file.path {
						None => {
							"New file"
						}
						Some(path) => {
							match path.file_name() {
								None => {
									"Error"
								}
								Some(file_name) => {
									match file_name.to_str() {
										None => {
											"Error"
										}
										Some(name) => {
											name
										}
									}
								}
							}
						}
					},
					if file.is_modified {
						"*"
					} else {
						""
					}
				))
					.width(Length::Fill)
					.into(),
				Message::SelectFile(index),
				index,
				self.current == index
			));
		}

		let tabs = Row::from_vec(tabs).width(Length::Fill).spacing(5);

		let input = text_editor(&self.files[self.current].content)
			.on_action(Message::Edit)
			.font(JETBRAINS_MONO)
			.height(Length::Fill)
			.highlight::<Highlighter>(highlighter::Settings {
				theme: highlighter::Theme::Base16Mocha,
				extension: self
					.files[self.current]
					.path
					.as_ref()
					.and_then(
						|path| path.extension()?.to_str()
					)
					.unwrap_or("rs")
					.to_string(),
			}, |highlight, _theme| highlight
				.to_format());

		let status_bar = {
			let status = if let Some(Error::IOFailed(error)) = self.error
				.as_ref() {
				text(error.to_string())
			} else {
				match self.files[self.current].path.as_deref().and_then(Path::to_str) {
					Some(path) => {
						text(path).size(14)
					}
					None => {
						text("New file").size(14)
					}
				}
			};

			let position = {
				let (line, column) = self.files[self.current].content.cursor_position();

				text(format!("{}:{}", line + 1, column + 1))
			};

			row![status, horizontal_space(), position]
		};

		Modal::new(
			container(col![menu_bar, tabs, input, status_bar].spacing(10))
				.padding(10),
			card
		).into()
	}

	fn theme(&self) -> Theme {
		self.theme.clone()
	}
}

fn separator() -> quad::Quad {
	quad::Quad {
		quad_color: Color::from([0.5; 3]).into(),
		quad_border: Border {
			radius: [4.0; 4].into(),
			..Default::default()
		},
		inner_bounds: InnerBounds::Ratio(0.98, 0.2),
		height: Length::Fixed(20.0),
		..Default::default()
	}
}

fn menubar_button<'a>(
	content: impl Into<Element<'a, Message>>,
	tooltip: Option<&'a str>,
	action: Message,
) -> Element<'a, Message> {
	let inner = button(
		container(content.into())
			.width(Length::Shrink)
			.center_x()
			.center_y()
			.padding([8, 4])
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
struct MenuButtonStyle;

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

fn menu_button<'a>(
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

fn tab(
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

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
	let handle = rfd::AsyncFileDialog::new()
		.set_title("Open File:")
		.pick_file()
		.await
		.ok_or(Error::DialogClosed)?;

	load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
	let contents = tokio::fs::read_to_string(&path)
		.await
		.map(verify_content)
		.map(Arc::new)
		.map_err(|error| error.kind())
		.map_err(Error::IOFailed)?;

	Ok((path, contents))
}

async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
	let path = if let Some(path) = path {
		path
	} else {
		rfd::AsyncFileDialog::new()
			.set_title("Save File:")
			.save_file()
			.await
			.ok_or(Error::DialogClosed)
			.map(|handle| handle.path().to_owned())?
	};

	tokio::fs::write(&path, text)
		.await
		.map_err(|error| Error::IOFailed(error.kind()))?;

	Ok(path)
}

#[derive(Debug, Clone)]
enum Error {
	DialogClosed,
	IOFailed(io::ErrorKind),
}

#[allow(clippy::needless_pass_by_value)]
fn verify_content(string: String) -> String {
	string
		.replace('\t', "    ")
		.replace("\r\n", "\n")
		.replace('\r', "\n")
}