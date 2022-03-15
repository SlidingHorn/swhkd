#[cfg(test)]
mod test_parse {
    use crate::config::*;
    use std::path::PathBuf;
    #[test]
    fn test_join_line() {
        let line1 = Line::new("ctrl+shift+\\".to_string(), LineType::Key, 3);
        let line2 = Line::new("b".to_string(), LineType::Key, 3);
        assert_eq!(
            line1.join_line(&line2),
            Line::new("ctrl+shift+b".to_string(), LineType::Key, 3)
        );
    }

    #[test]
    fn test_mark_line() {
        let key = "ctrl+shift+\\".to_string();
        let command = " a".to_string();
        let comment = "# a".to_string();
        let empty = "".to_string();
        assert_eq!(LineType::Key, Line::mark_line(&key));
        assert_eq!(LineType::Command, Line::mark_line(&command));
        assert_eq!(LineType::Other, Line::mark_line(&comment));
        assert_eq!(LineType::Other, Line::mark_line(&empty));
    }

    #[test]
    fn test_join_lines() {
        let content = "super + b
    b
super + \\
a
    a\\
    a";
        let lines = load_to_lines(content);
        let joined_lines = join_lines(lines);
        assert_eq!(
            joined_lines,
            vec![
                Line::new("super + b".to_string(), LineType::Key, 1),
                Line::new("b".to_string(), LineType::Command, 2),
                Line::new("super + a".to_string(), LineType::Key, 3),
                Line::new("aa".to_string(), LineType::Command, 5),
            ]
        );
    }

    #[test]
    fn test_parse_line_basic() {
        let keyline =
            Line { content: "super + b".to_string(), linetype: LineType::Key, linenumber: 1 };
        let commandline =
            Line { content: "b".to_string(), linetype: LineType::Command, linenumber: 2 };
        let output = parse_line(keyline, commandline, PathBuf::new());
        assert_eq!(
            output.unwrap()[0],
            ParseOutput::Hotkey(Hotkey::new(
                evdev::Key::KEY_B,
                vec![Modifier::Super],
                "b".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_line_curly_brace() {
        let keyline = Line {
            content: "super + {1,2,3,4}".to_string(),
            linetype: LineType::Key,
            linenumber: 1,
        };
        let commandline =
            Line { content: "{1,2,3,4}".to_string(), linetype: LineType::Command, linenumber: 2 };
        let output = parse_line(keyline, commandline, PathBuf::new());
        assert_eq!(
            output.unwrap(),
            vec![
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_1,
                    vec![Modifier::Super],
                    "1".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_2,
                    vec![Modifier::Super],
                    "2".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_3,
                    vec![Modifier::Super],
                    "3".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_4,
                    vec![Modifier::Super],
                    "4".to_string()
                )),
            ]
        );
    }

    #[test]
    fn test_parse_line_multiple_curly_braces() {
        let keyline = Line {
            content: "super + {shift+, ctrl+} {1,2,3,4}".to_string(),
            linetype: LineType::Key,
            linenumber: 1,
        };
        let commandline = Line {
            content: "{1,2,3,4, 5,6,  7,8}".to_string(),
            linetype: LineType::Command,
            linenumber: 2,
        };
        let output = parse_line(keyline, commandline, PathBuf::new());
        assert_eq!(
            output.unwrap(),
            vec![
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_1,
                    vec![Modifier::Super, Modifier::Shift],
                    "1".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_2,
                    vec![Modifier::Super, Modifier::Shift],
                    "2".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_3,
                    vec![Modifier::Super, Modifier::Shift],
                    "3".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_4,
                    vec![Modifier::Super, Modifier::Shift],
                    "4".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_1,
                    vec![Modifier::Super, Modifier::Control],
                    "5".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_2,
                    vec![Modifier::Super, Modifier::Control],
                    "6".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_3,
                    vec![Modifier::Super, Modifier::Control],
                    "7".to_string()
                )),
                ParseOutput::Hotkey(Hotkey::new(
                    evdev::Key::KEY_4,
                    vec![Modifier::Super, Modifier::Control],
                    "8".to_string()
                )),
            ]
        );
    }

    #[test]
    fn test_parse_line_keychord_basic() {
        let keyline =
            Line { content: "super + a; b".to_string(), linetype: LineType::Key, linenumber: 1 };
        let commandline =
            Line { content: "a".to_string(), linetype: LineType::Command, linenumber: 2 };
        let output = parse_line(keyline, commandline, PathBuf::new());
        assert_eq!(
            output.unwrap(),
            vec![ParseOutput::KeyChord(KeyChord {
                entry: KeyBinding::new(evdev::Key::KEY_A, vec![Modifier::Super]),
                chords: vec![vec![KeyBinding::new(evdev::Key::KEY_B, vec![])]],
                commands: vec!["a".to_string()],
            })]
        );
    }

    #[test]
    fn test_parse_line_keychord_complex() {
        let keyline = Line {
            content: "super + {1,2}; {3,4}; {5,6}".to_string(),
            linetype: LineType::Key,
            linenumber: 1,
        };
        let commandline = Line {
            content: "{1,2,3,4,5,6,7,8}".to_string(),
            linetype: LineType::Command,
            linenumber: 2,
        };
        let output = parse_line(keyline, commandline, PathBuf::new());
        println!("{:#?}", output);
        assert_eq!(
            output.unwrap(),
            vec![
                ParseOutput::KeyChord(KeyChord {
                    entry: KeyBinding::new(evdev::Key::KEY_1, vec![Modifier::Super]),
                    chords: vec![
                        vec![
                            KeyBinding::new(evdev::Key::KEY_3, vec![]),
                            KeyBinding::new(evdev::Key::KEY_5, vec![]),
                        ],
                        vec![
                            KeyBinding::new(evdev::Key::KEY_3, vec![]),
                            KeyBinding::new(evdev::Key::KEY_6, vec![]),
                        ],
                        vec![
                            KeyBinding::new(evdev::Key::KEY_4, vec![]),
                            KeyBinding::new(evdev::Key::KEY_5, vec![]),
                        ],
                        vec![
                            KeyBinding::new(evdev::Key::KEY_4, vec![]),
                            KeyBinding::new(evdev::Key::KEY_6, vec![]),
                        ],
                    ],
                    commands: vec![
                        "1".to_string(),
                        "2".to_string(),
                        "3".to_string(),
                        "4".to_string(),
                    ],
                }),
                ParseOutput::KeyChord(KeyChord {
                    entry: KeyBinding::new(evdev::Key::KEY_2, vec![Modifier::Super]),
                    chords: vec![
                        vec![
                            KeyBinding::new(evdev::Key::KEY_3, vec![]),
                            KeyBinding::new(evdev::Key::KEY_5, vec![]),
                        ],
                        vec![
                            KeyBinding::new(evdev::Key::KEY_3, vec![]),
                            KeyBinding::new(evdev::Key::KEY_6, vec![]),
                        ],
                        vec![
                            KeyBinding::new(evdev::Key::KEY_4, vec![]),
                            KeyBinding::new(evdev::Key::KEY_5, vec![]),
                        ],
                        vec![
                            KeyBinding::new(evdev::Key::KEY_4, vec![]),
                            KeyBinding::new(evdev::Key::KEY_6, vec![]),
                        ],
                    ],
                    commands: vec![
                        "5".to_string(),
                        "6".to_string(),
                        "7".to_string(),
                        "8".to_string(),
                    ],
                }),
            ]
        );
    }
}
