mod create_project;
mod first_run;
mod load_project;
mod main_state;
mod select_project;

pub use create_project::*;
pub use first_run::*;
pub use load_project::*;
pub use main_state::*;
pub use select_project::*;

pub enum AppState {
    FirstRun,
    CreateProject(Box<CreateProjectState>),
    SelectProject,
    LoadProject(LoadProjectState),
    Main(Box<MainState>),
}
