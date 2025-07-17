use egui::{FontData, FontDefinitions, FontFamily, Style};

use crate::model::Theme;
use crate::{
    first_run, main_window, project_creator, project_loader, project_selector, AppConfig, AppState,
};

pub struct AstraApp {
    pub config: AppConfig,
    pub state: AppState,
    pub next_state: Option<AppState>,
}

impl AstraApp {
    pub fn new(cc: &eframe::CreationContext<'_>, config: AppConfig) -> Self {
        match config.theme {
            Theme::Egui => cc.egui_ctx.set_style(Style::default()),
            Theme::Latte => catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::LATTE),
            Theme::Frappe => catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::FRAPPE),
            Theme::Macchiato => catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MACCHIATO),
            Theme::Mocha => catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA),
        }
        

        let mut font_definitions = FontDefinitions::default();
        font_definitions.font_data.insert(
            "noto_sans".to_owned(),
            FontData::from_static(include_bytes!("../assets/NotoSans-Regular.ttf")),
        );
        font_definitions.font_data.insert(
            "noto_sans_jp".to_owned(),
            FontData::from_static(include_bytes!("../assets/NotoSansJP-Regular.otf")),
        );
        let proportional = font_definitions
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap();
        proportional.insert(0, "noto_sans".into());
        proportional.push("noto_sans_jp".into());
        font_definitions
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("noto_sans_jp".to_owned());
        cc.egui_ctx.set_fonts(font_definitions);

        AstraApp {
            state: if config.projects.is_empty() && config.cobalt_path.is_empty() {
                AppState::FirstRun
            } else {
                AppState::SelectProject
            },
            config,
            next_state: None,
        }
    }
}

impl eframe::App for AstraApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Err(err) = self.config.save() {
            println!("{:?}", err);
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(state) = std::mem::take(&mut self.next_state) {
            self.state = state;
        }
        match &mut self.state {
            AppState::FirstRun => first_run(&mut self.config, &mut self.next_state, ctx),
            AppState::CreateProject(state) => {
                project_creator(state, &mut self.config, &mut self.next_state, ctx)
            }
            AppState::SelectProject => {
                project_selector(&mut self.config, &mut self.next_state, ctx)
            }
            AppState::LoadProject(state) => {
                project_loader(state, &self.config, &mut self.next_state, ctx)
            }
            AppState::Main(state) => {
                main_window(state, &mut self.next_state, &mut self.config, ctx)
            }
        }
    }
}
