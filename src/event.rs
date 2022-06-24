use serde::{Deserialize, Serialize};

use crate::key::Key;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "c", rename_all = "kebab-case")]
pub enum AppEvent {
  Batch { cmds: Vec<AppEvent> },

  Quit,
  ForceQuit,

  ToggleFocus,
  FocusProcs,
  FocusTerm,
  Zoom,

  NextProc,
  PrevProc,
  SelectProc { index: usize },
  StartProc,
  TermProc,
  KillProc,
  RestartProc,
  ForceRestartProc,
  ShowAddProc,
  AddProc { cmd: String },
  ShowRemoveProc,
  RemoveProc { id: usize },

  ScrollDownLines { n: usize },
  ScrollUpLines { n: usize },
  ScrollDown,
  ScrollUp,

  SendKey { key: Key },
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialize() {
    assert_eq!(
      serde_yaml::to_string(&AppEvent::ForceQuit).unwrap(),
      "---\nc: force-quit\n"
    );

    assert_eq!(
      serde_yaml::to_string(&AppEvent::SendKey {
        key: Key::parse("<c-a>").unwrap()
      })
      .unwrap(),
      "---\nc: send-key\nkey: \"<C-a>\"\n"
    );
  }
}
