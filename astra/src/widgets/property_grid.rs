use std::collections::HashSet;

use egui::{Grid, Id, Label, Response, ScrollArea, TextEdit, Ui};

use crate::raised_heading;

use super::defaults::DefaultWidget;

struct PropertyGridSection<'a, D> {
    name: &'a str,
    num_columns: usize,
    labels: HashSet<&'a str>,
    fields: Vec<(&'a str, Box<dyn Fn(&mut Ui, &mut D) -> Response + 'a>)>,
}

impl<'a, D> PropertyGridSection<'a, D> {
    pub fn new(name: &'a str, num_columns: usize) -> Self {
        Self {
            name,
            num_columns,
            labels: HashSet::new(),
            fields: vec![],
        }
    }

    pub fn field(
        &mut self,
        label: &'a str,
        add_contents: impl Fn(&mut Ui, &mut D) -> Response + 'a,
    ) {
        self.labels.insert(label);
        self.fields.push((label, Box::new(add_contents)));
    }

    pub fn visible(&self, filter: &str) -> bool {
        filter.is_empty()
            || self
                .labels
                .iter()
                .any(|label| label.to_lowercase().contains(&filter.to_lowercase()))
    }

    pub fn show(&self, ui: &mut Ui, data: &mut D, filter: &str) -> Response {
        if !self.name.is_empty() {
            ui.add(raised_heading(self.name));
        }
        let mut changed = false;
        let mut response = Grid::new(ui.auto_id_with("property_grid"))
            .num_columns(self.num_columns * 2)
            .show(ui, |ui| {
                let mut fields_in_row = 0;
                for (label, add_contents) in &self.fields {
                    if label.to_lowercase().contains(&filter.to_lowercase()) {
                        ui.vertical(|ui| {
                            ui.add(Label::new(*label).wrap(false));
                        });
                        if add_contents(ui, data).changed() {
                            changed = true;
                        }
                        fields_in_row += 1;
                        if fields_in_row >= self.num_columns {
                            fields_in_row = 0;
                            ui.end_row();
                        }
                    }
                }
                if fields_in_row != 0 {
                    ui.end_row();
                }
            })
            .response;
        if changed {
            response.mark_changed();
        }
        response
    }
}

pub struct PropertyGrid<'a, D> {
    id: Id,
    data: &'a mut D,
    sections: Vec<PropertyGridSection<'a, D>>,
}

impl<'a, D> PropertyGrid<'a, D> {
    pub fn new(id_source: &str, data: &'a mut D) -> Self {
        Self {
            data,
            sections: vec![],
            id: Id::new(id_source).with("property_grid"),
        }
    }

    pub fn new_section(mut self, name: &'a str) -> Self {
        self.sections.push(PropertyGridSection::new(name, 1));
        self
    }

    pub fn new_section_with_columns(mut self, name: &'a str, num_columns: usize) -> Self {
        self.sections
            .push(PropertyGridSection::new(name, num_columns));
        self
    }

    pub fn field(
        mut self,
        label: &'a str,
        add_contents: impl Fn(&mut Ui, &mut D) -> Response + 'a,
    ) -> Self {
        if let Some(section) = self.sections.last_mut() {
            section.field(label, add_contents);
        }
        self
    }

    pub fn default_field<F>(
        mut self,
        label: &'a str,
        retrieve_field: impl Fn(&mut D) -> &mut F + 'a,
    ) -> Self
    where
        F: DefaultWidget,
    {
        if let Some(section) = self.sections.last_mut() {
            section.field(label, move |ui: &mut Ui, data: &mut D| {
                retrieve_field(data).default_widget(ui)
            })
        }
        self
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let mut filter: String = ui.memory_mut(|mem| {
            std::mem::take(mem.data.get_persisted_mut_or_default::<String>(self.id))
        });
        let mut changed = false;
        let mut response = ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.add(TextEdit::singleline(&mut filter).hint_text("Search fields..."));
                    ui.separator();
                    for section in &self.sections {
                        if section.visible(&filter)
                            && section.show(ui, self.data, &filter).changed()
                        {
                            changed = true;
                        }
                    }
                })
            })
            .inner
            .response;
        if changed {
            response.mark_changed();
        }
        ui.memory_mut(|mem| mem.data.insert_persisted(self.id, filter));
        response
    }
}
