macro_rules! enum_combo_box {
    ($name:ident, $target:ty, $($key:expr => $label:expr,)+) => {
        pub fn $name(value: &mut $target) -> impl egui::Widget + '_ {
            move |ui: &mut egui::Ui| {
                {
                    let mut changed = false;
                    let id = ui.auto_id_with("__astra_static_combo");
                    let mut response = egui::ComboBox::from_id_source(id)
                        .width(ui.spacing().text_edit_width)
                        .selected_text(match value {
                            $(
                                $key => format!("{} - {}", $key, $label),
                            )+
                            _ => String::default(),
                        })
                        .show_ui(ui, |ui| {
                            let mut response: Option<egui::Response> = None;
                            $(
                                let value_response = ui.selectable_value(value, $key, format!("{} - {}", $key, $label));
                                changed |= value_response.changed();
                                match response {
                                    Some(r) => response = Some(r.union(value_response)),
                                    None => response = Some(value_response),
                                }
                            )+
                            response.unwrap()
                        })
                        .response;
                    if changed {
                        response.mark_changed();
                    }
                    response
                }
            }
        }
    };
}

enum_combo_box!(force_drop_down, i8,
    0 => "Player",
    1 => "Enemy",
    2 => "Other",
);

enum_combo_box!(gender_drop_down, i8,
    0 => "Other",
    1 => "Male",
    2 => "Female",
);

enum_combo_box!(nation_drop_down, i8,
    0 => "N/A",
    1 => "Lythos",
    2 => "Firene",
    3 => "Brodia",
    4 => "Elusia",
    5 => "Solm",
    6 => "Gradlon",
);

enum_combo_box!(item_kind_drop_down, i8,
    0 => "N/A",
    1 => "Sword",
    2 => "Lance",
    3 => "Axe",
    4 => "Bow",
    5 => "Dagger",
    6 => "Magic",
    7 => "Rod",
    8 => "Fist",
    9 => "Special",
    10 => "Tool",
    11 => "Shield",
    12 => "Accessory",
    13 => "Precious",
    14 => "RefineIron",
    15 => "RefineSteel",
    16 => "RefineSilver",
    17 => "PieceOfBond",
    18 => "Gold",
    19 => "Num",
);

enum_combo_box!(item_use_type_drop_down, i8,
    0 => "N/A",
    1 => "Attack",
    2 => "Heal",
    3 => "RestHeal",
    4 => "Revive",
    5 => "Warp",
    6 => "Rescue",
    7 => "EngageAdd",
    8 => "Rewarp",
    9 => "Freeze",
    10 => "Sleep",
    11 => "Silence",
    12 => "Charm",
    13 => "Berserk",
    14 => "Weakness",
    15 => "Again",
    16 => "Torch",
    17 => "Food",
    18 => "Rest",
    19 => "Sight Boost",
    20 => "Weapon Rank Increase",
    21 => "Stat Increase",
    22 => "Enhance",
    23 => "Master Seal",
    24 => "Secondary Seal",
    25 => "CCExtra",
    26 => "Creation",
    27 => "Draw",
    28 => "GainExp",
    29 => "Stun",
    30 => "Detox",
    31 => "GiveSkill",
    32 => "Foodstuff",
    33 => "Gift",
    34 => "Material",
    35 => "FishingRod",
    36 => "Bless",
    37 => "BlessRest",
    38 => "BlessPlus",
    39 => "BlessRestPlus",
    40 => "Enchanter Seal",
    41 => "Cannoneer Seal",
    42 => "GainSkillPoint",
);

enum_combo_box!(staff_type_drop_down, i8,
    0 => "N/A",
    1 => "Basic",
    2 => "Heal",
    3 => "Interference",
);

enum_combo_box!(job_rank_drop_down, i8,
    0 => "Base",
    1 => "Advanced",
);

enum_combo_box!(weapon_rank_numbered_drop_down, i8,
    0 => "N",
    1 => "E",
    2 => "D",
    3 => "C",
    4 => "B",
    5 => "A",
    6 => "S",
);

enum_combo_box!(skill_weapon_rank_numbered_drop_down, i8,
    0 => "E",
    1 => "D",
    2 => "C",
    3 => "B",
    4 => "A",
    5 => "S",
);

enum_combo_box!(chapter_spot_state, i8,
    0 => "Reserve Hide",
    1 => "Hide",
    2 => "Reserve Active",
    3 => "Active",
    4 => "Reserve Cannot Enter",
    5 => "Cannot Enter",
    6 => "Reserve Broken",
    7 => "Broken",
    8 => "Can Search",
);

enum_combo_box!(chapter_encount_type, i8,
    0 => "None",
    1 => "Corrupted",
    2 => "Firene Soldiers",
    3 => "Brodia Soldiers",
    4 => "Solm Soldiers",
    5 => "Elusia Soldiers",
);

enum_combo_box!(exist_die_timing_drop_down, i8,
    0 => "None",
    1 => "Begin",
    2 => "End",
    3 => "Chapter",
    4 => "Eternal",
);

enum_combo_box!(terrain_prohibition_drop_down, i8,
    0 => "None",
    1 => "All",
    2 => "Ground",
    3 => "Near",
);

enum_combo_box!(terrain_destroyer_drop_down, i8,
    0 => "None",
    1 => "Player",
    2 => "Enemy",
);

enum_combo_box!(skill_cycle_drop_down, i8,
    0 => "None",
    1 => "Map",
    2 => "Phase Before",
    3 => "Phase After",
    4 => "Fixed",
    5 => "Engaged",
    6 => "Battled",
    7 => "Battle (Offense)",
    8 => "Battle (Defense",
);

enum_combo_box!(skill_timing_drop_down, i8,
    0 => "None",
    1 => "Always",
    2 => "Battle Before",
    3 => "Battle Detail",
    4 => "Battle Invoke",
    5 => "Battle Start",
    6 => "Order Start",
    7 => "Action Start",
    8 => "Attack Start",
    9 => "Attack Branch",
    10 => "Hit Before",
    11 => "Hit After",
    12 => "Hit Effect",
    13 => "Attack End",
    14 => "Action End",
    15 => "Order End",
    16 => "Battle End",
    17 => "Battle Result",
    18 => "Battle After",
    19 => "Around",
    20 => "Support",
    21 => "Battle Command",
    22 => "Action Command",
    23 => "Overlap Command",
    24 => "Support Command",
    25 => "Fixed None",
    26 => "Fixed Done",
    27 => "Phase Start",
);

enum_combo_box!(skill_targets_drop_down, i8,
    0 => "Target",
    1 => "Enemy",
    2 => "Friend",
    3 => "Destroy",
    4 => "Pierce",
    5 => "Range",
    6 => "Around",
    7 => "Overlap",
);

enum_combo_box!(skill_frequencies_drop_down, i8,
    0 => "None",
    1 => "Every",
    2 => "First",
    3 => "Last",
);

enum_combo_box!(skill_stance_drop_down, i8,
    0 => "None",
    1 => "Offense",
    2 => "Defense",
);

enum_combo_box!(skill_around_centers_drop_down, i8,
    0 => "None",
    1 => "Self",
    2 => "Target",
    3 => "Link",
);

enum_combo_box!(skill_around_targets_drop_down, i8,
    0 => "None",
    1 => "Friend",
    2 => "Enemy",
    3 => "Both",
);

enum_combo_box!(skill_give_targets_drop_down, i8,
    0 => "Target",
    1 => "Self",
    2 => "Chain",
    3 => "Around",
    4 => "Dance",
);

enum_combo_box!(ring_rank_drop_down, i8,
    0 => "C",
    1 => "B",
    2 => "A",
    3 => "S",
);

macro_rules! string_combo_box {
    ($name:ident, $($key:expr => $label:expr,)+) => {
        pub fn $name(value: &mut String) -> impl egui::Widget + '_ {
            move |ui: &mut egui::Ui| {
                {
                    let mut changed = false;
                    let id = ui.auto_id_with("__astra_static_combo");
                    let mut response = egui::ComboBox::from_id_source(id)
                        .width(ui.spacing().text_edit_width)
                        .selected_text(match value.as_str() {
                            $(
                                $key => $label,
                            )+
                            _ => "",
                        })
                        .show_ui(ui, |ui| {
                            let mut response: Option<egui::Response> = None;
                            $(
                                let value_response = ui.selectable_label(*value == $key, $key);
                                if value_response.clicked() {
                                    *value = $key.to_owned();
                                }
                                changed |= value_response.changed();
                                match response {
                                    Some(r) => response = Some(r.union(value_response)),
                                    None => response = Some(value_response),
                                }
                            )+
                            response.unwrap()
                        })
                        .response;
                    if changed {
                        response.mark_changed();
                    }
                    response
                }
            }
        }
    };
}

string_combo_box!(weapon_rank_drop_down,
    "N" => "N",
    "E" => "E",
    "D" => "D",
    "C" => "C",
    "C+" => "C+",
    "B" => "B",
    "B+" => "B+",
    "A" => "A",
    "A+" => "A+",
    "S" => "S",
);
