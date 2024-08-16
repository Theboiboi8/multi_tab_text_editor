#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::perf, clippy::pedantic)]
#![deny(rust_2024_compatibility)]

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use iced::highlighter::Highlighter;
use iced::widget::combo_box::State;
use iced::widget::{container, horizontal_space, row, text, text_editor, Column, Row};
use iced::window::settings::PlatformSpecific;
use iced::window::{icon, Level, Position};
use iced::{
	executor, highlighter, window, Alignment, Application, Command, Element, Font, Length, Pixels,
	Settings, Size, Theme,
};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu, menu_bar, menu_items, Modal};
use serde::{Deserialize, Serialize};

mod editor;

pub static JETBRAINS_MONO: LazyLock<Font> = LazyLock::new(|| Font::with_name("JetBrains Mono"));

pub static INTER: LazyLock<Font> = LazyLock::new(|| Font::with_name("Inter"));

pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
	let config_path = dirs::config_dir().unwrap_or_default();

	let editor_config_dir = config_path.join("multi_tab_text_editor");

	let config_file_name = PathBuf::from("multi_tab_text_editor_config.json");

	if !editor_config_dir.exists() && std::fs::create_dir_all(&editor_config_dir).is_err() {
		eprintln!("Failed to create config directory");
	}

	editor_config_dir.join(config_file_name)
});

pub static CONFIG: LazyLock<Option<SettingsState>> = LazyLock::new(|| {
	let config_path = &*CONFIG_PATH;

	if config_path.exists() {
		let config: Option<SettingsState> =
			serde_json::from_str(std::fs::read_to_string(config_path).unwrap().leak())
				.unwrap_or_else(|error| {
					eprintln!("Failed to read config: {error}");
					None
				});

		config
	} else {
		None
	}
});

#[must_use]
pub fn theme_to_key(theme: &Theme) -> &str {
	match theme {
		Theme::Light => "theme.light",
		Theme::Dark => "theme.dark",
		Theme::Dracula => "theme.dracula",
		Theme::Nord => "theme.nord",
		Theme::SolarizedLight => "theme.solarized.light",
		Theme::SolarizedDark => "theme.solarized.dark",
		Theme::GruvboxLight => "theme.gruvbox.light",
		Theme::GruvboxDark => "theme.gruvbox.dark",
		Theme::CatppuccinLatte => "theme.catppuccin.latte",
		Theme::CatppuccinFrappe => "theme.catppuccin.frappe",
		Theme::CatppuccinMacchiato => "theme.catppuccin.macchiato",
		Theme::CatppuccinMocha => "theme.catppuccin.mocha",
		Theme::TokyoNight => "theme.tokionight",
		Theme::TokyoNightStorm => "theme.tokionight.storm",
		Theme::TokyoNightLight => "theme.tokyonight.light",
		Theme::KanagawaWave => "theme.kanagawa.wave",
		Theme::KanagawaDragon => "theme.kanagawa.dragon",
		Theme::KanagawaLotus => "theme.kanagawa.lotus",
		Theme::Moonfly => "theme.moonfly",
		Theme::Nightfly => "theme.nightfly",
		Theme::Oxocarbon => "theme.oxocarbon",
		Theme::Custom(_) => "theme.unknown",
	}
}

#[must_use]
pub fn key_to_theme(key: &str) -> Theme {
	match key {
		"theme.dark" => Theme::Dark,
		"theme.dracula" => Theme::Dracula,
		"theme.nord" => Theme::Nord,
		"theme.solarized.light" => Theme::SolarizedLight,
		"theme.solarized.dark" => Theme::SolarizedDark,
		"theme.gruvbox.light" => Theme::GruvboxLight,
		"theme.gruvbox.dark" => Theme::GruvboxDark,
		"theme.catppuccin.latte" => Theme::CatppuccinLatte,
		"theme.catppuccin.frappe" => Theme::CatppuccinFrappe,
		"theme.catppuccin.macchiato" => Theme::CatppuccinMacchiato,
		"theme.catppuccin.mocha" => Theme::CatppuccinMocha,
		"theme.tokyonight" => Theme::TokyoNight,
		"theme.tokyonight.storm" => Theme::TokyoNightStorm,
		"theme.tokyonight.light" => Theme::TokyoNightLight,
		"theme.kanagawa.wave" => Theme::KanagawaWave,
		"theme.kanagawa.dragon" => Theme::KanagawaDragon,
		"theme.kanagawa.lotus" => Theme::KanagawaLotus,
		"theme.moonfly" => Theme::Moonfly,
		"theme.nightfly" => Theme::Nightfly,
		"theme.oxocarbon" => Theme::Oxocarbon,
		_ => Theme::Light,
	}
}

#[must_use]
pub fn syntax_theme_to_key(theme: &highlighter::Theme) -> &str {
	use highlighter::Theme;

	match theme {
		Theme::SolarizedDark => "syntax.solarized.dark",
		Theme::Base16Mocha => "syntax.base16.mocha",
		Theme::Base16Ocean => "syntax.base16.ocean",
		Theme::Base16Eighties => "syntax.base16.eighties",
		Theme::InspiredGitHub => "syntax.inspired-github",
	}
}

#[must_use]
pub fn key_to_syntax_theme(key: &str) -> highlighter::Theme {
	use highlighter::Theme;

	match key {
		"syntax.solarized.dark" => Theme::SolarizedDark,
		"syntax.base16.mocha" => Theme::Base16Mocha,
		"syntax.base16.ocean" => Theme::Base16Ocean,
		"syntax.inspired-github" => Theme::InspiredGitHub,
		_ => Theme::Base16Eighties,
	}
}

fn invoke_config_update(state: &Editor) {
	let config = SettingsState {
		theme: theme_to_key(&state.theme).to_string(),
		syntax_theme: syntax_theme_to_key(&state.highlighter_theme).to_string(),
	};

	let config_path = &*CONFIG_PATH;

	if std::fs::write(config_path, serde_json::to_string(&config).unwrap()).is_err() {
		eprintln!("Failed to write config to file");
	}
}

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
			icon: Some(icon::from_file_data(include_bytes!("../assets/icon.png"), None).unwrap()),
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
			include_bytes!("../assets/Inter-Regular.ttf")
				.as_slice()
				.into(),
		],
		default_font: *INTER,
		default_text_size: Pixels(13.0),
		antialiasing: true,
	})
}

struct Editor {
	files: Vec<File>,
	current: usize,
	error: Option<Error>,
	modal_shown: bool,
	modal_type: ModalType,
	theme: Theme,
	themes: State<Theme>,
	highlighter_theme: highlighter::Theme,
	highlighter_themes: State<highlighter::Theme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsState {
	theme: String,
	syntax_theme: String,
}

pub struct File {
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

		let content = verify_content(String::from_utf8_lossy(sample).to_string()).leak();

		File {
			path: None,
			content: text_editor::Content::with_text(content),
			is_modified: true,
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
	ShowInExplorer(PathBuf),
	ShowModal(ModalType),
	HideModal,
	SelectTheme(Theme),
	SelectSyntaxTheme(highlighter::Theme),
	None,
}

#[derive(Debug, Clone)]
enum ModalType {
	About,
	Settings,
}

pub const THEMES: [Theme; 21] = [
	Theme::Light,
	Theme::Dark,
	Theme::Dracula,
	Theme::Nord,
	Theme::SolarizedLight,
	Theme::SolarizedDark,
	Theme::GruvboxLight,
	Theme::GruvboxDark,
	Theme::CatppuccinLatte,
	Theme::CatppuccinFrappe,
	Theme::CatppuccinMacchiato,
	Theme::CatppuccinMocha,
	Theme::TokyoNight,
	Theme::TokyoNightStorm,
	Theme::TokyoNightLight,
	Theme::KanagawaWave,
	Theme::KanagawaDragon,
	Theme::KanagawaLotus,
	Theme::Moonfly,
	Theme::Nightfly,
	Theme::Oxocarbon,
];

impl Application for Editor {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = ();

	fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
		let (theme, syntax) = if let Some(config) = &*CONFIG {
			(
				key_to_theme(&config.theme),
				key_to_syntax_theme(&config.syntax_theme),
			)
		} else {
			(Theme::Dark, highlighter::Theme::Base16Eighties)
		};

		(
			Self {
				files: vec![File::sample()],
				error: None,
				current: 0,
				modal_shown: false,
				modal_type: ModalType::About,
				theme,
				themes: State::new(THEMES.to_vec()),
				highlighter_theme: syntax,
				highlighter_themes: State::new(highlighter::Theme::ALL.to_vec()),
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		format!(
			"Multi Tab Text Editor | {}",
			match &self.files[self.current].path {
				None => format!(
					"New File{}",
					if self.files[self.current].is_modified {
						"*"
					} else {
						""
					}
				),
				Some(path) => {
					let mut name = path.file_name()
						.unwrap_or(OsStr::new(""))
						.to_str()
						.unwrap_or("")
						.to_string();
					
					if self.files[self.current].is_modified {
						name.push('*');
					}
					
					name
				}
			}
		)
	}

	#[allow(clippy::too_many_lines)]
	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::Edit(action) => {
				assert!(self.current < self.files.len());
				
				self.files[self.current].is_modified =
					self.files[self.current].is_modified || action.is_edit();
				self.error = None;

				self.files[self.current].content.perform(action);

				Command::none()
			}
			Message::Open => Command::perform(pick_file(), Message::FileOpened),
			Message::FileOpened(Ok((path, content))) => {
				assert!(self.current < self.files.len());

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
				assert!(self.current < self.files.len());

				let text = self.files[self.current].content.text();

				Command::perform(
					save_file(self.files[self.current].path.clone(), text),
					Message::FileSaved,
				)
			}
			Message::SaveAs => {
				assert!(self.current < self.files.len());

				let text = self.files[self.current].content.text();

				Command::perform(save_file(None, text), Message::FileSaved)
			}
			Message::FileSaved(Ok(path)) => {
				assert!(self.current < self.files.len());

				self.files[self.current].path = Some(path);
				self.files[self.current].is_modified = false;

				Command::none()
			}
			Message::Close => {
				assert!(self.current < self.files.len());

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
				assert!(self.current < self.files.len());

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
				if opener::open(url).is_err() {
					eprintln!("Failed to open url {url}");
				}

				Command::none()
			}
			Message::ShowInExplorer(path) => {
				if opener::open(path.clone()).is_err() {
					eprintln!("Failed to open path {}", path.display());
				}

				Command::none()
			}
			Message::ShowModal(modal_type) => {
				self.modal_shown = true;
				self.modal_type = modal_type;

				Command::none()
			}
			Message::HideModal => {
				self.modal_shown = false;

				Command::none()
			}
			Message::SelectTheme(theme) => {
				self.theme = theme;

				invoke_config_update(self);

				Command::none()
			}
			Message::SelectSyntaxTheme(theme) => {
				self.highlighter_theme = theme;

				invoke_config_update(self);

				Command::none()
			}
			Message::None => Command::none(),
		}
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self) -> Element<'_, Self::Message> {
		let card = if self.modal_shown {
			Some(match self.modal_type {
				ModalType::About => editor::components::about_modal(&self.theme),
				ModalType::Settings => editor::components::settings_modal(self),
			})
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

		let menu_bar = menu_bar![(
            editor::components::menubar_button(text("File"), None, Message::None),
            {
                let sub_menu = menu_tpl_2(menu_items![(editor::components::menu_button(
                    row![editor::icons::new_icon(12), text("   New"),]
                        .align_items(Alignment::Center),
                    Message::New
                ))(
                    editor::components::menu_button(
                        row![editor::icons::open_icon(12), text("   Open..."),]
                            .align_items(Alignment::Center),
                        Message::Open
                    )
                )(
                    editor::components::menu_button(
                        row![editor::icons::save_icon(12), text("   Save"),]
                            .align_items(Alignment::Center),
                        Message::Save
                    )
                )(
                    editor::components::menu_button(
                        row![editor::icons::save_as_icon(12), text("   Save As"),]
                            .align_items(Alignment::Center),
                        Message::SaveAs
                    )
                )(
                    if let Some(path) = self.files[self.current].path.clone() {
                        editor::components::menu_button(
                            row![editor::icons::eye_icon(12), text("   Show in Explorer"),]
                                .align_items(Alignment::Center),
                            Message::ShowInExplorer(path),
                        )
                    } else {
                        editor::components::menu_button_disabled(
                            row![editor::icons::eye_icon(12), text("   Show in Explorer"),]
                                .align_items(Alignment::Center),
                        )
                    }
                )(
                    editor::components::menu_button(
                        row![editor::icons::close_icon(12), text("   Close"),]
                            .align_items(Alignment::Center),
                        Message::Close
                    )
                )(
                    editor::components::separator(&self.theme)
                )(
                    editor::components::menu_button(
                        row![editor::icons::settings_icon(12), text("   Settings"),]
                            .align_items(Alignment::Center),
                        Message::ShowModal(ModalType::Settings)
                    )
                )])
                .width(180.0);

                sub_menu
            }
        )(
            editor::components::menubar_button(text("Help"), None, Message::None),
            {
                let sub_menu = menu_tpl_2(menu_items![(editor::components::menu_button(
                    row![editor::icons::info_icon(12), text("   About"),]
                        .align_items(Alignment::Center),
                    Message::ShowModal(ModalType::About)
                ))(
                    editor::components::menu_button(
                        row![editor::icons::git_icon(12), text("   Source"),]
                            .align_items(Alignment::Center),
                        Message::OpenURL("https://github.com/Theboiboi8/multi_tab_text_editor")
                    )
                )])
                .width(180.0);

                sub_menu
            }
        )]
			.draw_path(menu::DrawPath::Backdrop);

		let mut tabs = Vec::new();

		for (index, file) in self.files.iter().enumerate() {
			tabs.push(editor::components::tab(
				text(format!(
					"{}{}",
					match &file.path {
						None => {
							"New file"
						}
						Some(path) => {
							match path.file_name() {
								None => "Error",
								Some(file_name) => file_name.to_str().unwrap_or("Error"),
							}
						}
					},
					if file.is_modified { "*" } else { "" }
				))
					.width(Length::Fill)
					.into(),
				Message::SelectFile(index),
				index,
				self.current == index,
			));
		}

		let tabs = Row::from_vec(tabs).width(Length::Fill).spacing(5);

		let input = text_editor(&self.files[self.current].content)
			.on_action(Message::Edit)
			.font(*JETBRAINS_MONO)
			.height(Length::Fill)
			.highlight::<Highlighter>(
				highlighter::Settings {
					theme: self.highlighter_theme,
					extension: self.files[self.current]
						.path
						.as_ref()
						.and_then(|path| path.extension()?.to_str())
						.unwrap_or("rs")
						.to_string(),
				},
				|highlight, _theme| highlight.to_format(),
			);

		let status_bar = {
			let status = if let Some(Error::IOFailed(error)) = self.error.as_ref() {
				text(error.to_string())
			} else {
				match self.files[self.current]
					.path
					.as_deref()
					.and_then(Path::to_str)
				{
					Some(path) => text(path).size(14),
					None => text("New file").size(14),
				}
			};

			let position = {
				let (line, column) = self.files[self.current].content.cursor_position();

				text(format!("{}:{}", line + 1, column + 1))
			};

			row![status, horizontal_space(), position]
		};

		Modal::new(
			container(
				Column::new()
					.push(menu_bar)
					.push(tabs)
					.push(input)
					.push(status_bar)
					.spacing(10),
			)
				.padding(10),
			card,
		)
			.into()
	}

	fn theme(&self) -> Theme {
		self.theme.clone()
	}
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
	let handle = rfd::AsyncFileDialog::new()
		.set_title("Open File:")
		.pick_file()
		.await
		.ok_or(Error::DialogClosed)?;

	load_file(handle.path()).await
}

async fn load_file(path: &Path) -> Result<(PathBuf, Arc<String>), Error> {
	let contents = tokio::fs::read_to_string(&path)
		.await
		.map(verify_content)
		.map(Arc::new)
		.map_err(|error| error.kind())
		.map_err(Error::IOFailed)?;

	Ok((PathBuf::from(path), contents))
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

