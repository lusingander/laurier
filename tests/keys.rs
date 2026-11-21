use laurier::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    key_code, key_code_char,
};

// Exported macros must resolve Crossterm through laurier, even if the caller shadows it.
mod crossterm {}

#[test]
fn test_exported_key_macros() {
    let event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    assert!(matches!(event, key_code!(KeyCode::Esc)));

    let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
    assert!(matches!(event, key_code_char!('a')));
    let key_code_char!(ch) = event else {
        panic!("expected a character key event");
    };
    assert_eq!(ch, 'a');

    let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
    assert!(matches!(event, key_code_char!('a', Ctrl)));
}
