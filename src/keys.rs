#[macro_export]
macro_rules! key_code {
    ( $code:pat ) => {
        ratatui::crossterm::event::KeyEvent { code: $code, .. }
    };
}

#[macro_export]
macro_rules! key_code_char {
    ( $c:ident ) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Char($c),
            ..
        }
    };
    ( $c:expr ) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Char($c),
            ..
        }
    };
    ( $c:expr, Ctrl ) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Char($c),
            modifiers: ratatui::crossterm::event::KeyModifiers::CONTROL,
            ..
        }
    };
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_key_code() {
        let e = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        assert!(matches!(e, key_code!(KeyCode::Esc)));
        assert!(!matches!(e, key_code!(KeyCode::Enter)));

        let e = KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE);
        assert!(matches!(e, key_code!(KeyCode::F(1))));
    }

    #[test]
    fn test_key_code_char() {
        let e = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert!(matches!(e, key_code_char!('a')));
        assert!(!matches!(e, key_code_char!('b')));
        assert!(!matches!(e, key_code_char!('a', Ctrl)));

        let e = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        assert!(matches!(e, key_code_char!('a')));
        assert!(matches!(e, key_code_char!('a', Ctrl)));

        let e = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::SHIFT);
        assert!(matches!(e, key_code_char!('a')));
        assert!(!matches!(e, key_code_char!('a', Ctrl)));

        let e = KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        );
        assert!(matches!(e, key_code_char!('a')));
        assert!(!matches!(e, key_code_char!('a', Ctrl)));

        let e = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        if let key_code_char!(ch) = e {
            assert_eq!(ch, 'a');
        } else {
            panic!()
        }
    }
}
