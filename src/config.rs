use std::path::PathBuf;
use std::sync::LazyLock;
use iced::{highlighter, Theme};
use crate::{Editor, SettingsState};

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
					eprintln!("Failed to read configuration file: {error}");
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

pub fn save(state: &Editor) {
	let config = SettingsState {
		theme: theme_to_key(&state.theme).to_string(),
		syntax_theme: syntax_theme_to_key(&state.highlighter_theme).to_string(),
	};

	let config_path = &*CONFIG_PATH;

	if let Err(error) = std::fs::write(config_path, serde_json::to_string(&config).unwrap()) {
		eprintln!("Failed to write configuration to file: {error}");
	}
}