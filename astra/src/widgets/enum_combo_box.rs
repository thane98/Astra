macro_rules! enum_combo_box {
    ($name:ident, $target:ty, $($key:expr => $label:expr,)+) => {
        pub fn $name<'a>(value: &'a mut Option<$target>) -> impl egui::Widget + 'a {
            move |ui: &mut egui::Ui| {
                {
                    let mut changed = false;
                    let id = ui.auto_id_with("__astra_static_combo");
                    let mut response = egui::ComboBox::from_id_source(id)
                        .width(ui.spacing().text_edit_width)
                        .selected_text(match value {
                            $(
                                Some($key) => $label,
                            )+
                            _ => "",
                        })
                        .show_ui(ui, |ui| {
                            let mut response: Option<egui::Response> = None;
                            $(
                                let value_response = ui.selectable_value(value, Some($key), $label);
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

macro_rules! string_combo_box {
    ($name:ident, $($key:expr => $label:expr,)+) => {
        pub fn $name<'a>(value: &'a mut String) -> impl egui::Widget + 'a {
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
