use egui::{Key, KeyboardShortcut, Modifiers};

pub static ADD_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::A);

pub static INSERT_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::I);

pub static DUPLICATE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::D);

pub static DELETE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::Delete);

pub static MOVE_UP_SHORTCUT: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND, Key::ArrowUp);

pub static MOVE_DOWN_SHORTCUT: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND, Key::ArrowDown);

pub static COPY_TO_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

pub static NEXT_TAB_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::T);

pub static PREV_TAB_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers {
        alt: false,
        ctrl: false,
        shift: true,
        mac_cmd: false,
        command: true,
    },
    Key::T,
);

pub static UP_ENTRY_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers {
        alt: false,
        ctrl: true,
        shift: true,
        mac_cmd: false,
        command: true,
    },
    Key::ArrowUp,
);

pub static DOWN_ENTRY_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers {
        alt: false,
        ctrl: true,
        shift: true,
        mac_cmd: false,
        command: true,
    },
    Key::ArrowDown,
);
