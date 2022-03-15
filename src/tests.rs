mod test_line_operations {
    use crate::config::*;

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
}

mod test_parse_content {
    use crate::config::*;
    use std::path::PathBuf;

    // Type E refers to the type of the expected output
    // Use `impl TestParse<E> for T` to define a test method for type T, expecting type E
    pub trait TestParse<E> {
        fn test(&self, expected: E);
    }

    impl TestParse<&str> for &str {
        fn test(&self, expected: &str) {
            let output = parse_contents(self, PathBuf::new());
            println!("{:#?}", output);
            assert!(output.is_ok());
            let expected_output = parse_contents(expected, PathBuf::new()).unwrap();
            assert_eq!(output.as_ref().unwrap().len(), expected_output.len());
            for item in output.unwrap() {
                assert!(expected_output.contains(&item));
            }
        }
    }

    impl TestParse<Vec<Hotkey>> for &str {
        fn test(&self, expected: Vec<Hotkey>) {
            let output = parse_contents(self, PathBuf::new());
            println!("{:#?}", output);
            assert!(output.is_ok());
            assert_eq!(output.as_ref().unwrap().len(), expected.len());
            for item in output.unwrap() {
                assert!(item.is_hotkey());
                assert!(expected.contains(item.extract_hotkey()));
            }
        }
    }

    impl TestParse<Error> for &str {
        fn test(&self, expected: Error) {
            let output = parse_contents(self, PathBuf::new());
            println!("{:#?}", output);
            assert!(output.is_err());
            let output = format!("{:?}", output.unwrap_err());
            let expected = format!("{:?}", expected);
            assert_eq!(output, expected);
        }
    }

    impl TestParse<Vec<KeyChord>> for &str {
        fn test(&self, expected: Vec<KeyChord>) {
            let output = parse_contents(self, PathBuf::new());
            println!("{:#?}", output);
            assert!(output.is_ok());
            assert_eq!(output.as_ref().unwrap().len(), expected.len());
            for item in output.unwrap() {
                assert!(item.is_keychord());
                assert!(expected.contains(item.extract_keychord()));
            }
        }
    }

    #[test]
    fn test_parse_content_curly_brace() {
        let contents = "
super + {_,ctrl + }{_,shift + }{1-4}
    dwmc {, toggle}{view, tag}ex {0-3}";
        let expected = "
super + {1,2,3,4}
    dwmc viewex {0,1,2,3}
super + ctrl + {1,2,3,4}
    dwmc toggleviewex {0,1,2,3}
super + shift + {1,2,3,4}
    dwmc tagex {0,1,2,3}
super + ctrl + shift + {1,2,3,4}
    dwmc toggletagex {0,1,2,3}";
        contents.test(expected);
    }

    #[test]
    fn test_parse_prefixes() {
        let contents = "
super + @a
    a
super + ~b
    b
super + @~c
    c
super + ~@d
    d";
        let expected = vec![
            Hotkey::new(evdev::Key::KEY_A, vec![Modifier::Super], "a".to_string()).on_release(),
            Hotkey::new(evdev::Key::KEY_B, vec![Modifier::Super], "b".to_string()).send(),
            Hotkey::new(evdev::Key::KEY_C, vec![Modifier::Super], "c".to_string())
                .on_release()
                .send(),
            Hotkey::new(evdev::Key::KEY_D, vec![Modifier::Super], "d".to_string())
                .on_release()
                .send(),
        ];
        contents.test(expected);
    }

    #[test]
    fn test_error_invalid_modifier() {
        let contents = "
super + invalid + a
    a";
        let expected = Error::InvalidConfig(ParseError::InvalidModifier(
            PathBuf::new(),
            2,
            "invalid".to_string(),
        ));
        contents.test(expected);
    }

    #[test]
    fn test_error_invalid_keysym() {
        let contents = "
super + invalid
    a";
        let expected = Error::InvalidConfig(ParseError::InvalidKeysym(
            PathBuf::new(),
            2,
            "invalid".to_string(),
        ));
        contents.test(expected);
    }

    #[test]
    fn test_parse_keychord() {
        let contents = "super + {1,2}; {3-4}; {5,6}
    {1,2,3,4,5-8}";
        let expected = vec![
            KeyChord {
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
                commands: vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()],
            },
            KeyChord {
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
                commands: vec!["5".to_string(), "6".to_string(), "7".to_string(), "8".to_string()],
            },
        ];
        contents.test(expected);
    }
}
