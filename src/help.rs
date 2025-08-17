use crate::InputState;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Keybinding {
    pub key: &'static str,
    pub text: &'static str,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum KeybindingContext {
    NonCheatMenu,
    SelectSource,
    SelectDestination,
    CheatMenu,
}

static KEYBINDINGS: &[(Keybinding, KeybindingContext)] = &[
    (
        Keybinding {
            key: "[0-9]",
            text: "Select a row to move cards from",
        },
        KeybindingContext::SelectSource,
    ),
    (
        Keybinding {
            key: "[0-9]",
            text: "Select a row to move cards to",
        },
        KeybindingContext::SelectDestination,
    ),
    (
        Keybinding {
            key: "[0-9]",
            text: "Select a cheat to apply",
        },
        KeybindingContext::SelectDestination,
    ),
    (
        Keybinding {
            key: "[Enter]",
            text: "Deal row of cards",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[u]",
            text: "Undo",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[c]",
            text: "Quit",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[q]",
            text: "Quit",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[C]",
            text: "Cheats",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[R]",
            text: "Restart",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[0-9]",
            text: "Select a cheat",
        },
        KeybindingContext::CheatMenu,
    ),
    (
        Keybinding {
            key: "[q]",
            text: "Exit menu",
        },
        KeybindingContext::CheatMenu,
    ),
    (
        Keybinding {
            key: "[esc]",
            text: "Exit menu",
        },
        KeybindingContext::CheatMenu,
    ),
];

pub fn get_keybindings(state: InputState) -> impl IntoIterator<Item = Keybinding> {
    KEYBINDINGS
        .iter()
        .copied()
        .filter(match state {
            InputState::SelectSource => {
                &(|i: &(Keybinding, KeybindingContext)| {
                    matches!(
                        i.1,
                        KeybindingContext::NonCheatMenu | KeybindingContext::SelectSource
                    )
                }) as &dyn Fn(&(Keybinding, KeybindingContext)) -> bool
            }

            InputState::SelectDestination(_) => &|i: &(Keybinding, KeybindingContext)| {
                matches!(
                    i.1,
                    KeybindingContext::NonCheatMenu | KeybindingContext::SelectDestination
                )
            },
            InputState::CheatMenu => {
                &|i: &(Keybinding, KeybindingContext)| matches!(i.1, KeybindingContext::CheatMenu)
            }
        })
        .map(|i| i.0)
}
