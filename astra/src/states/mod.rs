mod create_project;
mod load_project;
mod main_state;
mod select_project;

pub use create_project::*;
pub use load_project::*;
pub use main_state::*;
pub use select_project::*;

pub enum AppState {
    CreateProject(CreateProjectState),
    SelectProject,
    LoadProject(LoadProjectState),
    Main(MainState),
}
