use egui::Hyperlink;
use egui_modal::Modal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn about_modal(ctx: &egui::Context) -> Modal {
    let modal = Modal::new(ctx, "about_modal");
    modal.show(|ui| {
        modal.title(ui, format!("Astra v{}", VERSION));
        ui.label("Credits");
        ui.horizontal(|ui| {
            ui.label("•");
            ui.add(Hyperlink::from_label_and_url("Raytwo", "https://github.com/Raytwo"));
        });
        ui.horizontal(|ui| {
            ui.label("•");
            ui.add(Hyperlink::from_label_and_url("DeathChaos", "https://github.com/DeathChaos25"));
        });
        ui.horizontal(|ui| {
            ui.label("•");
            ui.add(Hyperlink::from_label_and_url("Araragi Hoozuki (Documentation)", "https://github.com/AraragiHoozuki"));
        });
        ui.horizontal(|ui| {
            ui.label("•");
            ui.add(Hyperlink::from_label_and_url("Perfare (AssetStudio)", "https://github.com/Perfare"));
        });
        modal.buttons(ui, |ui| {
            modal.button(ui, "Close");
        });
    });
    modal
}