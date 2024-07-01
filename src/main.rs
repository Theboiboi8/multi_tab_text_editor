#![windows_subsystem = "windows"]

mod icons;

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use iced::{alignment, Application, Command, Element, executor, Font, highlighter, Length, Pixels, Settings, Size, Theme, theme, window};
use iced::highlighter::Highlighter;
use iced::widget::{button, column, container, horizontal_space, row, Row, text, text_editor, tooltip};
use iced::window::{Level, Position};
use iced::window::settings::PlatformSpecific;
use iced_aw::menu::{Menu, Item};
use iced_aw::{menu, menu_bar, menu_items};

const UNCUT_SANS: Font = Font::with_name("UncutSans");
const JETBRAINS_MONO: Font = Font::with_name("JetBrainsMono");

fn main() -> iced::Result {
	Editor::run(Settings {
		id: None,
		window: window::Settings {
			size: Size::new(1024.0, 768.0),
			position: Position::default(),
			min_size: None,
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
	SelectFile(usize),
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
			Message::SelectFile(index) => {
				self.current = index;

				Command::none()
			},
			Message::None => {
				Command::none()
			}
		}
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self) -> Element<'_, Self::Message> {
		//let menu_tpl_1 = |items| Menu::new(items)
		//	.max_width(180.0)
		//	.offset(15.0)
		//	.spacing(5.0);
		let menu_tpl_2 = |items| Menu::new(items)
			.max_width(180.0)
			.offset(0.0)
			.spacing(5.0);

		let menu_bar = menu_bar![
			(menu_button_s("File", Message::None), {
				let sub_menu = menu_tpl_2(menu_items![
					(menu_button("New", Message::New))
					(menu_button("Open...", Message::Open))
					(menu_button("Save", Message::Save))
					(menu_button("Save As", Message::SaveAs))
					(menu_button("Close", Message::Close))
				]).width(180.0);
				
				sub_menu
			})
		].draw_path(menu::DrawPath::Backdrop);

		let mut tabs = Vec::new();

		for (index, file) in self.files.iter().enumerate() {
			tabs.push(action(
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
				)).into(),
				"Open this file",
				Message::SelectFile(index),
				Some(Length::Fill),
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

		container(column![menu_bar, tabs, input, status_bar].spacing(10))
			.padding(10)
			.into()
	}

	fn theme(&self) -> Theme {
		Theme::KanagawaDragon
	}
}

fn action<'a>(
	content: Element<'a, Message>,
	label: &'a str,
	on_press: Message,
	width: Option<Length>,
	highlighted: bool,
) -> Element<'a, Message> {
	tooltip(
		button(container(content).width(if width.is_some() {
			#[allow(clippy::unnecessary_unwrap)]
			width.unwrap()
		} else {
			Length::Fixed(24.0)
		}).center_x())
			.style(if highlighted {
				theme::Button::Primary
			} else {
				theme::Button::Secondary
			})
			.on_press(on_press)
			.padding([5, 10]),
		label,
		tooltip::Position::FollowCursor,
	)
		.style(theme::Container::Box)
		.into()
}

fn base_button<'a>(
	content: impl Into<Element<'a, Message>>,
	action: Message,
) -> button::Button<'a, Message> {
	button(content)
		.padding([4, 8])
		.style(theme::Button::Secondary)
		.on_press(action)
}

fn labeled_button(
	label: &str,
	action: Message,
) -> button::Button<Message, Theme, iced::Renderer> {
	base_button(
		text(label).vertical_alignment(alignment::Vertical::Center),
		action,
	)
}

fn menu_button(
	label: &str,
	action: Message
) -> button::Button<Message, Theme, iced::Renderer> {
	labeled_button(label, action).width(Length::Fill)
}

fn menu_button_s(
	label: &str,
	action: Message
) -> button::Button<Message, Theme, iced::Renderer> {
	labeled_button(label, action).width(Length::Shrink)
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