use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, keyed_add_modal_content, sheet_retriever, EditorState,
    GroupEditorContent, GroupViewItem, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{
    MuscleAssistData, MuscleExerciseDataBook, MuscleExerciseDifficulty, MuscleExercisePrizeData,
    MuscleExerciseSetup, MusclePushUpSpeedData, MuscleSitUpFallData, MuscleSquatJudgeAreaData,
    MuscleSquatMusicSheet, MuscleSquatScoreListData,
};

sheet_retriever!(MuscleExerciseDifficulty, MuscleExerciseDataBook, difficulty, IndexMap<String, MuscleExerciseDifficulty>);

impl ViewItem for MuscleExerciseDifficulty {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for MuscleExerciseDifficulty {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(MuscleExerciseSetup, MuscleExerciseDataBook, setups, IndexMap<String, MuscleExerciseSetup>);

impl ViewItem for MuscleExerciseSetup {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for MuscleExerciseSetup {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(MuscleExercisePrizeData, MuscleExerciseDataBook, prizes, IndexMap<String, MuscleExercisePrizeData>);

impl ViewItem for MuscleExercisePrizeData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for MuscleExercisePrizeData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(MuscleSitUpFallData, MuscleExerciseDataBook, sit_up_fall_data, IndexMap<String, MuscleSitUpFallData>);

impl ViewItem for MuscleSitUpFallData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for MuscleSitUpFallData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(MusclePushUpSpeedData, MuscleExerciseDataBook, push_up_speed, IndexMap<String, MusclePushUpSpeedData>);

impl ViewItem for MusclePushUpSpeedData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for MusclePushUpSpeedData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(MuscleSquatJudgeAreaData, MuscleExerciseDataBook, squat_judge_area, IndexMap<String, MuscleSquatJudgeAreaData>);

impl ViewItem for MuscleSquatJudgeAreaData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for MuscleSquatJudgeAreaData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(MuscleSquatScoreListData, MuscleExerciseDataBook, score_list_data, IndexMap<String, MuscleSquatScoreListData>);

impl ViewItem for MuscleSquatScoreListData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for MuscleSquatScoreListData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(MuscleSquatMusicSheet, MuscleExerciseDataBook, music_sheets, IndexMap<String, Vec<MuscleSquatMusicSheet>>);

impl GroupViewItem for IndexMap<String, Vec<MuscleSquatMusicSheet>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for MuscleSquatMusicSheet {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!(
            "{} {} {} {} {} {} {} {} {} {}",
            self.type_a_l,
            self.type_a_r,
            self.type_b_l,
            self.type_b_r,
            self.type_c_l,
            self.type_c_r,
            self.type_d_l,
            self.type_d_r,
            self.type_e_l,
            self.type_e_r,
        ))
    }
}

sheet_retriever!(MuscleAssistData, MuscleExerciseDataBook, assist_data, IndexMap<String, MuscleAssistData>);

impl ViewItem for MuscleAssistData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for MuscleAssistData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Difficulty,
    Setup,
    PrizeData,
    SitUpFallData,
    PushUpSpeedData,
    SquatJudgeAreaData,
    SquatScoreListData,
    SquatMusicSheet,
    AssistData,
}

pub struct MuscleExerciseDataEditor {
    tab: Tab,
    difficulty: MuscleExerciseDifficultySheet,
    setups: MuscleExerciseSetupSheet,
    prizes: MuscleExercisePrizeDataSheet,
    sit_up_fall_data: MuscleSitUpFallDataSheet,
    push_up_speed: MusclePushUpSpeedDataSheet,
    squat_judge_area: MuscleSquatJudgeAreaDataSheet,
    score_list_data: MuscleSquatScoreListDataSheet,
    music_sheets: MuscleSquatMusicSheetSheet,
    assist_data: MuscleAssistDataSheet,
    difficulty_content: ListEditorContent<
        IndexMap<String, MuscleExerciseDifficulty>,
        MuscleExerciseDifficulty,
        EditorState,
    >,
    setups_content:
        ListEditorContent<IndexMap<String, MuscleExerciseSetup>, MuscleExerciseSetup, EditorState>,
    prizes_content: ListEditorContent<
        IndexMap<String, MuscleExercisePrizeData>,
        MuscleExercisePrizeData,
        EditorState,
    >,
    sit_up_fall_data_content:
        ListEditorContent<IndexMap<String, MuscleSitUpFallData>, MuscleSitUpFallData, EditorState>,
    push_up_speed_content: ListEditorContent<
        IndexMap<String, MusclePushUpSpeedData>,
        MusclePushUpSpeedData,
        EditorState,
    >,
    squat_judge_area_content: ListEditorContent<
        IndexMap<String, MuscleSquatJudgeAreaData>,
        MuscleSquatJudgeAreaData,
        EditorState,
    >,
    score_list_data_content: ListEditorContent<
        IndexMap<String, MuscleSquatScoreListData>,
        MuscleSquatScoreListData,
        EditorState,
    >,
    music_sheets_content: GroupEditorContent,
    assist_data_content:
        ListEditorContent<IndexMap<String, MuscleAssistData>, MuscleAssistData, EditorState>,
}

impl MuscleExerciseDataEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Difficulty,
            difficulty: state.muscle_exercise_difficulty.clone(),
            setups: state.muscle_exercise_setups.clone(),
            prizes: state.muscle_exercise_prizes.clone(),
            sit_up_fall_data: state.muscle_exercise_sit_up_fall_data.clone(),
            push_up_speed: state.muscle_exercise_push_up_speed.clone(),
            squat_judge_area: state.muscle_exercise_squat_judge_area.clone(),
            score_list_data: state.muscle_exercise_score_list_data.clone(),
            music_sheets: state.muscle_exercise_music_sheets.clone(),
            assist_data: state.muscle_exercise_assist_data.clone(),
            difficulty_content: ListEditorContent::new("difficulty_editor")
                .with_add_modal_content(keyed_add_modal_content),
            setups_content: ListEditorContent::new("setups_editor")
                .with_add_modal_content(keyed_add_modal_content),
            prizes_content: ListEditorContent::new("prizes_editor")
                .with_add_modal_content(keyed_add_modal_content),
            sit_up_fall_data_content: ListEditorContent::new("sit_up_fall_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            push_up_speed_content: ListEditorContent::new("push_up_speed_editor")
                .with_add_modal_content(keyed_add_modal_content),
            squat_judge_area_content: ListEditorContent::new("squat_judge_area_editor")
                .with_add_modal_content(keyed_add_modal_content),
            score_list_data_content: ListEditorContent::new("score_list_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            music_sheets_content: GroupEditorContent::new("music_sheets_editor"),
            assist_data_content: ListEditorContent::new("assist_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Difficulty, "Difficulty");
            ui.selectable_value(&mut self.tab, Tab::Setup, "Setup");
            ui.selectable_value(&mut self.tab, Tab::PrizeData, "Prizes");
            ui.selectable_value(&mut self.tab, Tab::SitUpFallData, "Sit Up Fall");
            ui.selectable_value(&mut self.tab, Tab::PushUpSpeedData, "Push Up Speed");
            ui.selectable_value(&mut self.tab, Tab::SquatJudgeAreaData, "Squat Judge Area");
            ui.selectable_value(&mut self.tab, Tab::SquatScoreListData, "Squat Score List");
            ui.selectable_value(&mut self.tab, Tab::SquatMusicSheet, "Squat Music");
            ui.selectable_value(&mut self.tab, Tab::AssistData, "Assist");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::Difficulty => {
                self.difficulty_content
                    .left_panel(ctx, &self.difficulty, state);
                self.difficulty.write(|data| {
                    self.difficulty_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("difficulty", selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .default_field("Exercise Type", |d| &mut d.exercise_type)
                            .default_field("Level", |d| &mut d.level)
                            .default_field("Good Score", |d| &mut d.good_score)
                            .default_field("Perfect Score", |d| &mut d.perfect_score)
                            .default_field("Target Score", |d| &mut d.target_score)
                            .default_field("Endless Goal Count", |d| &mut d.endless_goal_count)
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::Setup => {
                self.setups_content.left_panel(ctx, &self.setups, state);
                self.setups.write(|data| {
                    self.setups_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("setups", selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .default_field("Level", |d| &mut d.level)
                            .default_field("Area P Center", |d| &mut d.area_p_center)
                            .default_field("Area P Radius", |d| &mut d.area_p_radius)
                            .default_field("Area G Center", |d| &mut d.area_g_center)
                            .default_field("Area G Radius", |d| &mut d.area_g_radius)
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::PrizeData => {
                self.prizes_content.left_panel(ctx, &self.prizes, state);
                self.prizes.write(|data| {
                    self.prizes_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("prizes", selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .default_field("Exercise Type", |d| &mut d.exercise_type)
                            .default_field("Bonus Sss", |d| &mut d.bonus_sss)
                            .default_field("Bond Sss", |d| &mut d.bond_sss)
                            .default_field("Border Sss", |d| &mut d.border_sss)
                            .default_field("Bonus Ss", |d| &mut d.bonus_ss)
                            .default_field("Bond Ss", |d| &mut d.bond_ss)
                            .default_field("Border Ss", |d| &mut d.border_ss)
                            .default_field("Bonus S", |d| &mut d.bonus_s)
                            .default_field("Bond S", |d| &mut d.bond_s)
                            .default_field("Border S", |d| &mut d.border_s)
                            .default_field("Bonus A", |d| &mut d.bonus_a)
                            .default_field("Bond A", |d| &mut d.bond_a)
                            .default_field("Border A", |d| &mut d.border_a)
                            .default_field("Bonus B", |d| &mut d.bonus_b)
                            .default_field("Bond B", |d| &mut d.bond_b)
                            .default_field("Border B", |d| &mut d.border_b)
                            .default_field("Bonus C", |d| &mut d.bonus_c)
                            .default_field("Bond C", |d| &mut d.bond_c)
                            .default_field("Border C", |d| &mut d.border_c)
                            .default_field("Bonus D", |d| &mut d.bonus_d)
                            .default_field("Bond D", |d| &mut d.bond_d)
                            .default_field("Border D", |d| &mut d.border_d)
                            .default_field("Bonus E", |d| &mut d.bonus_e)
                            .default_field("Bond E", |d| &mut d.bond_e)
                            .default_field("Border E", |d| &mut d.border_e)
                            .default_field("Bonus F", |d| &mut d.bonus_f)
                            .default_field("Bond F", |d| &mut d.bond_f)
                            .default_field("Border F", |d| &mut d.border_f)
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::SitUpFallData => {
                self.sit_up_fall_data_content
                    .left_panel(ctx, &self.sit_up_fall_data, state);
                self.sit_up_fall_data.write(|data| {
                    self.sit_up_fall_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("sit_up_fall_data", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Level", |d| &mut d.level)
                                .default_field("Perfect Limit", |d| &mut d.perfect_limit)
                                .default_field("Gain Power", |d| &mut d.gain_power)
                                .default_field("Fall Speed", |d| &mut d.fall_speed)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::PushUpSpeedData => {
                self.push_up_speed_content
                    .left_panel(ctx, &self.push_up_speed, state);
                self.push_up_speed.write(|data| {
                    self.push_up_speed_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("push_up_speed", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Level", |d| &mut d.level)
                                .default_field("Speed Min", |d| &mut d.speed_min)
                                .default_field("Speed Max", |d| &mut d.speed_max)
                                .default_field("Level Up Count", |d| &mut d.level_up_count)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::SquatJudgeAreaData => {
                self.squat_judge_area_content
                    .left_panel(ctx, &self.squat_judge_area, state);
                self.squat_judge_area.write(|data| {
                    self.squat_judge_area_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("squat_judge_area", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Bad Frame First", |d| &mut d.bad_frame_first)
                                .default_field("Good Frame First", |d| &mut d.good_frame_first)
                                .default_field("Perfect Frame First", |d| {
                                    &mut d.perfect_frame_first
                                })
                                .default_field("Perfect Frame Latter", |d| {
                                    &mut d.perfect_frame_latter
                                })
                                .default_field("Good Frame Latter", |d| &mut d.good_frame_latter)
                                .default_field("Bad Frame Latter", |d| &mut d.bad_frame_latter)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::SquatScoreListData => {
                self.score_list_data_content
                    .left_panel(ctx, &self.score_list_data, state);
                self.score_list_data.write(|data| {
                    self.score_list_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("score_list_data", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Level", |d| &mut d.level)
                                .default_field("Use Count", |d| &mut d.use_count)
                                .default_field("Is Double Choice", |d| &mut d.is_double_choice)
                                .default_field("Speed", |d| &mut d.speed)
                                .default_field("Length", |d| &mut d.length)
                                .default_field("Type 1", |d| &mut d.type_1)
                                .default_field("Lottery Param 1", |d| &mut d.lottery_param_1)
                                .default_field("Type 2", |d| &mut d.type_2)
                                .default_field("Lottery Param 2", |d| &mut d.lottery_param_2)
                                .default_field("Type 3", |d| &mut d.type_3)
                                .default_field("Lottery Param 3", |d| &mut d.lottery_param_3)
                                .default_field("Type 4", |d| &mut d.type_4)
                                .default_field("Lottery Param 4", |d| &mut d.lottery_param_4)
                                .default_field("Type 5", |d| &mut d.type_5)
                                .default_field("Lottery Param 5", |d| &mut d.lottery_param_5)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::SquatMusicSheet => {
                self.music_sheets_content
                    .left_panel(ctx, &self.music_sheets, state);
                self.music_sheets.write(|data| {
                    self.music_sheets_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("music_sheets", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Type A L", |d| &mut d.type_a_l)
                                .default_field("Type A R", |d| &mut d.type_a_r)
                                .default_field("Type B L", |d| &mut d.type_b_l)
                                .default_field("Type B R", |d| &mut d.type_b_r)
                                .default_field("Type C L", |d| &mut d.type_c_l)
                                .default_field("Type C R", |d| &mut d.type_c_r)
                                .default_field("Type D L", |d| &mut d.type_d_l)
                                .default_field("Type D R", |d| &mut d.type_d_r)
                                .default_field("Type E L", |d| &mut d.type_e_l)
                                .default_field("Type E R", |d| &mut d.type_e_r)
                                .default_field("Ensure", |d| &mut d.ensure)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::AssistData => {
                self.assist_data_content
                    .left_panel(ctx, &self.assist_data, state);
                self.assist_data.write(|data| {
                    self.assist_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("assist_data", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Level 00", |d| &mut d.level_00)
                                .default_field("Level 01", |d| &mut d.level_01)
                                .default_field("Level 02", |d| &mut d.level_02)
                                .default_field("Level 03", |d| &mut d.level_03)
                                .default_field("Level 04", |d| &mut d.level_04)
                                .default_field("Level 05", |d| &mut d.level_05)
                                .default_field("Level 06", |d| &mut d.level_06)
                                .default_field("Level 07", |d| &mut d.level_07)
                                .default_field("Level 08", |d| &mut d.level_08)
                                .default_field("Level 09", |d| &mut d.level_09)
                                .default_field("Level 10", |d| &mut d.level_10)
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
