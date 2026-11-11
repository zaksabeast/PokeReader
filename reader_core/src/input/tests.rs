use crate::alloc::format;
use crate::alloc::string::String;

use super::{
    action::InputAction,
    binding::{
        BUTTON_A, BUTTON_B, BUTTON_DDOWN, BUTTON_DUP, BUTTON_L, BUTTON_R, BUTTON_SELECT, BUTTON_START,
        BUTTON_X, BUTTON_Y, BUTTON_ZL, BUTTON_ZR, BindingParseError, Chord, InputBinding,
    },
    config::{ActiveDevices, ConfigParseError, InputConfiguration, InputDevice, parse_config},
    manager::InputManager,
    repository::{self, LoadStatus, SaveResult},
};

fn parse(text: &str) -> InputBinding {
    InputBinding::parse(text).expect("binding should parse")
}

mod defaults {
    use super::*;

    #[test]
    fn legacy_defaults_match_the_previous_controls() {
        let config = InputConfiguration::legacy();

        assert_eq!(
            config.binding(InputDevice::Keyboard, InputAction::Pause),
            parse("Start+Select")
        );
        assert_eq!(
            config.binding(InputDevice::Keyboard, InputAction::StepFrame),
            parse("Select")
        );
        assert_eq!(
            config.binding(InputDevice::Keyboard, InputAction::Resume),
            parse("Start|A")
        );
        assert_eq!(
            config.binding(InputDevice::Keyboard, InputAction::ToggleOverlay),
            parse("Start+Up")
        );
        assert_eq!(
            config.binding(InputDevice::Keyboard, InputAction::ToggleLock),
            parse("X+Y")
        );
    }

    #[test]
    fn default_configuration_enables_both_profiles() {
        let config = InputConfiguration::default();

        assert_eq!(config, InputConfiguration::legacy());
        assert_eq!(config.active_devices(), ActiveDevices::BOTH);
        assert!(config.is_device_active(InputDevice::Keyboard));
        assert!(config.is_device_active(InputDevice::Gamepad));
    }

    #[test]
    fn conflict_free_preset_keeps_the_critical_actions_isolated() {
        let config = InputConfiguration::conflict_free();

        // Pause, frame step and resume never reach the game.
        for action in [InputAction::Pause, InputAction::StepFrame, InputAction::Resume] {
            let binding = config.binding(InputDevice::Keyboard, action);
            assert!(!binding.is_unbound(), "every action should be bound");
            assert!(
                !InputConfiguration::binding_leaks_to_game(binding),
                "{:?} must not use gameplay buttons",
                action
            );
        }

        // Overlay and lock need a third button. They use L/R (which the games
        // ignore outside of menus) instead of the D-Pad.
        for action in [InputAction::ToggleOverlay, InputAction::ToggleLock] {
            let binding = config.binding(InputDevice::Keyboard, action);
            assert!(!binding.is_unbound());
            assert!(binding.buttons() & (BUTTON_ZL | BUTTON_ZR) != 0);
        }
    }
}

mod binding {
    use super::*;

    #[test]
    fn parses_single_chord() {
        let binding = parse("Start+Select");

        assert_eq!(binding.chords().len(), 1);
        assert_eq!(binding.chords()[0], Chord::new(BUTTON_START | BUTTON_SELECT));
    }

    #[test]
    fn parses_alternatives_and_ignores_spaces_and_case() {
        let binding = parse(" start | a ");

        assert_eq!(binding.chords().len(), 2);
        assert_eq!(binding.to_config_string(), "Start|A");
    }

    #[test]
    fn empty_values_are_unbound() {
        assert!(parse("").is_unbound());
        assert!(parse("-").is_unbound());
        assert_eq!(parse("").to_config_string(), "-");
    }

    #[test]
    fn rejects_unknown_buttons() {
        assert_eq!(
            InputBinding::parse("Start+Nope"),
            Err(BindingParseError::UnknownButton)
        );
    }

    #[test]
    fn rejects_chords_that_are_too_long() {
        assert_eq!(
            InputBinding::parse("A+B+X+Y+L"),
            Err(BindingParseError::TooManyButtons)
        );
    }

    #[test]
    fn rejects_too_many_alternatives() {
        assert_eq!(
            InputBinding::parse("A|B|X"),
            Err(BindingParseError::TooManyAlternatives)
        );
    }

    #[test]
    fn matches_require_all_buttons_of_the_chord() {
        let chord = Chord::new(BUTTON_ZL | BUTTON_ZR);

        assert!(!chord.is_held(BUTTON_ZL));
        assert!(chord.is_held(BUTTON_ZL | BUTTON_ZR));
        assert!(chord.is_held(BUTTON_ZL | BUTTON_ZR | BUTTON_A));
    }

    #[test]
    fn matches_require_a_new_press() {
        let binding = parse("Start+Select");

        assert!(binding.matches(BUTTON_START, BUTTON_START | BUTTON_SELECT));
        assert!(!binding.matches(0, BUTTON_START | BUTTON_SELECT));
    }

    #[test]
    fn displays_compact_and_human_readable_forms() {
        let binding = parse("Start+Select|A");

        assert_eq!(binding.to_compact_string(), "Start+Select/A");
        assert_eq!(binding.to_display_string(), "Start + Select / A");
    }

    #[test]
    fn round_trips_through_the_config_format() {
        for text in ["ZL", "Start|A", "Start+Select", "ZL+ZR+Up"] {
            assert_eq!(parse(text).to_config_string(), text);
        }
    }
}

mod configuration {
    use super::*;

    #[test]
    fn parses_a_custom_file() {
        let text = "\
# comment
version=1
active=keyboard
keyboard.pause=ZL
keyboard.step_frame=ZR
keyboard.resume=ZL+ZR
keyboard.toggle_overlay=ZL+ZR+Up
keyboard.toggle_lock=ZL+ZR+Down
gamepad.pause=-
";

        let parsed = parse_config(text).expect("config should parse");
        let config = parsed.config;

        assert_eq!(parsed.warnings, 0);
        assert!(!parsed.migrated);
        assert_eq!(config.active_devices(), ActiveDevices::KEYBOARD);
        assert_eq!(
            config.binding(InputDevice::Keyboard, InputAction::Pause),
            parse("ZL")
        );
        assert_eq!(
            config.binding(InputDevice::Keyboard, InputAction::Resume),
            parse("ZL+ZR")
        );
        assert!(
            config
                .binding(InputDevice::Gamepad, InputAction::Pause)
                .is_unbound()
        );
    }

    #[test]
    fn missing_file_content_uses_defaults_and_is_migrated() {
        let parsed = parse_config("").expect("empty config should parse");

        assert_eq!(parsed.config, InputConfiguration::legacy());
        assert!(parsed.migrated);
    }

    #[test]
    fn older_files_are_migrated_key_by_key() {
        let parsed = parse_config("keyboard.pause=ZL\n").expect("config should parse");

        assert!(parsed.migrated);
        assert_eq!(
            parsed.config.binding(InputDevice::Keyboard, InputAction::Pause),
            parse("ZL")
        );
        assert_eq!(
            parsed
                .config
                .binding(InputDevice::Gamepad, InputAction::StepFrame),
            parse("Select")
        );
    }

    #[test]
    fn accepts_windows_line_endings() {
        let parsed = parse_config("version=1\r\nkeyboard.pause=ZL\r\n").expect("config should parse");

        assert_eq!(
            parsed.config.binding(InputDevice::Keyboard, InputAction::Pause),
            parse("ZL")
        );
    }

    #[test]
    fn newer_versions_are_rejected() {
        assert_eq!(
            parse_config("version=2\n"),
            Err(ConfigParseError::UnsupportedVersion(2))
        );
    }

    #[test]
    fn invalid_values_keep_the_default_and_report_a_warning() {
        let parsed = parse_config("keyboard.pause=NotAButton\n").expect("config should parse");

        assert_eq!(parsed.warnings, 1);
        assert_eq!(
            parsed.config.binding(InputDevice::Keyboard, InputAction::Pause),
            InputConfiguration::legacy().binding(InputDevice::Keyboard, InputAction::Pause)
        );
    }

    #[test]
    fn unknown_keys_are_ignored_for_forward_compatibility() {
        let parsed = parse_config("version=1\nfuture.key=value\n").expect("config should parse");

        assert_eq!(parsed.warnings, 0);
    }

    #[test]
    fn serialization_round_trips() {
        let mut config = InputConfiguration::conflict_free();
        config.set_binding(InputDevice::Keyboard, InputAction::Pause, parse("L+ZL"));
        config.clear_binding(InputDevice::Gamepad, InputAction::Resume);
        config.set_active_devices(ActiveDevices::KEYBOARD);

        let text = config.to_config_string();
        let parsed = parse_config(&text).expect("config should parse");

        assert_eq!(parsed.config, config);
        assert_eq!(parsed.warnings, 0);
        assert!(!parsed.migrated);
    }

    #[test]
    fn detects_conflicting_bindings() {
        let config = InputConfiguration::legacy();

        assert_eq!(
            config.conflicting_action(InputAction::StepFrame, parse("Start+Select")),
            Some((InputDevice::Keyboard, InputAction::Pause))
        );
        assert_eq!(config.conflicting_action(InputAction::Pause, parse("ZL")), None);
    }

    #[test]
    fn more_specific_chords_are_not_conflicts() {
        let config = InputConfiguration::conflict_free();

        assert_eq!(
            config.conflicting_action(InputAction::Resume, parse("ZL+ZR")),
            None
        );
    }

    #[test]
    fn reports_bindings_that_leak_to_gameplay() {
        assert!(InputConfiguration::binding_leaks_to_game(parse("Start")));
        assert!(InputConfiguration::binding_leaks_to_game(parse("ZL+Up")));
        assert!(!InputConfiguration::binding_leaks_to_game(parse("ZL+ZR")));
    }

    #[test]
    fn devices_can_be_switched_and_cleared() {
        let mut config = InputConfiguration::legacy();

        assert_eq!(ActiveDevices::BOTH.cycle(), ActiveDevices::KEYBOARD);
        assert_eq!(ActiveDevices::KEYBOARD.cycle(), ActiveDevices::GAMEPAD);
        assert_eq!(ActiveDevices::GAMEPAD.cycle(), ActiveDevices::NONE);
        assert_eq!(ActiveDevices::NONE.cycle(), ActiveDevices::BOTH);

        config.set_active_devices(ActiveDevices::GAMEPAD);
        assert!(!config.is_device_active(InputDevice::Keyboard));
        assert!(config.is_device_active(InputDevice::Gamepad));

        config.clear_profile(InputDevice::Gamepad);
        for action in InputAction::ALL {
            assert!(config.binding(InputDevice::Gamepad, action).is_unbound());
        }
    }

    #[test]
    fn restoring_defaults_undoes_custom_bindings() {
        let mut config = InputConfiguration::conflict_free();
        config.clear_all_bindings();

        for device in InputDevice::ALL {
            for action in InputAction::ALL {
                assert!(config.binding(device, action).is_unbound());
            }
        }

        config = InputConfiguration::legacy();
        assert_eq!(
            config.binding(InputDevice::Keyboard, InputAction::Pause),
            parse("Start+Select")
        );
    }

    #[test]
    fn active_device_tokens_are_parsed() {
        assert_eq!(
            ActiveDevices::from_config_str("keyboard,gamepad"),
            Some(ActiveDevices::BOTH)
        );
        assert_eq!(ActiveDevices::from_config_str("none"), Some(ActiveDevices::NONE));
        assert_eq!(ActiveDevices::from_config_str(""), Some(ActiveDevices::NONE));
        assert_eq!(ActiveDevices::from_config_str("mouse"), None);
    }
}

mod manager {
    use super::*;

    fn legacy_manager() -> InputManager {
        InputManager::new(InputConfiguration::legacy())
    }

    #[test]
    fn pause_is_triggered_by_start_and_select() {
        let held = BUTTON_START | BUTTON_SELECT;
        let mut manager = legacy_manager();

        manager.update(held, 0);

        assert!(manager.should_pause(held, 0));
    }

    #[test]
    fn pause_is_not_triggered_while_the_chord_is_held() {
        let held = BUTTON_START | BUTTON_SELECT;
        let mut manager = legacy_manager();

        manager.update(held, 0);
        assert!(manager.should_pause(held, 0));

        manager.update(held, held);
        assert!(!manager.should_pause(held, held));
    }

    #[test]
    fn step_waits_for_the_button_to_be_released() {
        let mut config = InputConfiguration::legacy();
        config.set_binding(InputDevice::Keyboard, InputAction::StepFrame, parse("ZL"));
        config.set_binding(InputDevice::Gamepad, InputAction::StepFrame, parse("ZL"));
        let mut manager = InputManager::new(config);

        manager.update(BUTTON_ZL, 0);
        assert!(!manager.should_step_frame(BUTTON_ZL, 0));

        assert!(manager.should_step_frame(0, BUTTON_ZL));
        assert!(!manager.should_step_frame(0, BUTTON_ZL));
    }

    #[test]
    fn resume_waits_for_the_button_to_be_released() {
        let mut manager = legacy_manager();

        manager.update(BUTTON_A, 0);
        assert!(!manager.should_resume(BUTTON_A, 0));

        assert!(manager.should_resume(0, BUTTON_A));
        assert!(!manager.should_resume(0, BUTTON_A));
    }

    #[test]
    fn step_does_not_fire_without_a_step_press() {
        let mut manager = legacy_manager();

        manager.update(BUTTON_ZL, 0);
        manager.update(0, BUTTON_ZL);

        assert!(!manager.should_step_frame(0, BUTTON_ZL));
    }

    #[test]
    fn more_specific_chords_win() {
        let mut manager = InputManager::new(InputConfiguration::conflict_free());
        let held = BUTTON_ZL | BUTTON_ZR;

        manager.update(held, 0);

        assert!(!manager.action_just_pressed(held, 0, InputAction::Pause));
        assert!(!manager.action_just_pressed(held, 0, InputAction::StepFrame));
        assert!(manager.action_just_pressed(held, 0, InputAction::Resume));
    }

    #[test]
    fn single_buttons_still_trigger_their_own_actions() {
        let mut manager = InputManager::new(InputConfiguration::conflict_free());

        manager.update(BUTTON_ZL, 0);
        assert!(manager.action_just_pressed(BUTTON_ZL, 0, InputAction::Pause));

        manager.update(BUTTON_ZR, 0);
        assert!(manager.action_just_pressed(BUTTON_ZR, 0, InputAction::StepFrame));
    }

    #[test]
    fn disabled_devices_ignore_input() {
        let mut config = InputConfiguration::legacy();
        config.set_active_devices(ActiveDevices::NONE);
        let mut manager = InputManager::new(config);
        let held = BUTTON_START | BUTTON_SELECT;

        manager.update(held, 0);

        assert!(!manager.should_pause(held, 0));
    }

    #[test]
    fn keyboard_and_gamepad_profiles_are_independent() {
        let mut config = InputConfiguration::legacy();
        config.clear_all_bindings();
        config.set_binding(InputDevice::Keyboard, InputAction::Pause, parse("ZL"));
        config.set_binding(InputDevice::Gamepad, InputAction::Pause, parse("ZR"));

        config.set_active_devices(ActiveDevices::KEYBOARD);
        let mut manager = InputManager::new(config);
        manager.update(BUTTON_ZR, 0);
        assert!(!manager.should_pause(BUTTON_ZR, 0));
        manager.update(BUTTON_ZL, 0);
        assert!(manager.should_pause(BUTTON_ZL, 0));

        config.set_active_devices(ActiveDevices::GAMEPAD);
        let mut manager = InputManager::new(config);
        manager.update(BUTTON_ZL, 0);
        assert!(!manager.should_pause(BUTTON_ZL, 0));
        manager.update(BUTTON_ZR, 0);
        assert!(manager.should_pause(BUTTON_ZR, 0));
    }

    #[test]
    fn recording_suppresses_actions() {
        let held = BUTTON_START | BUTTON_SELECT;
        let mut manager = legacy_manager();
        manager.set_recording(true);

        manager.update(held, 0);

        assert!(!manager.should_pause(held, 0));
    }

    #[test]
    fn holds_the_crystal_shortcut_only_for_the_default_binding() {
        let mut manager = legacy_manager();
        manager.enable_legacy_crystal_pause();
        manager.update(BUTTON_L | BUTTON_R, 0);
        assert!(manager.should_pause(BUTTON_L | BUTTON_R, 0));

        let mut config = InputConfiguration::legacy();
        config.set_binding(InputDevice::Keyboard, InputAction::Pause, parse("ZL"));
        config.set_binding(InputDevice::Gamepad, InputAction::Pause, parse("ZL"));
        let mut manager = InputManager::new(config);
        manager.enable_legacy_crystal_pause();
        manager.update(BUTTON_L | BUTTON_R, 0);
        assert!(!manager.should_pause(BUTTON_L | BUTTON_R, 0));
    }

    #[test]
    fn reports_held_actions_for_the_test_view() {
        let mut manager = legacy_manager();
        manager.update(BUTTON_A, 0);
        assert!(manager.is_action_held(InputAction::Resume));
        assert!(!manager.is_action_held(InputAction::Pause));

        manager.update(BUTTON_START | BUTTON_SELECT, BUTTON_A);
        assert!(manager.is_action_held(InputAction::Pause));
    }

    #[test]
    fn uses_the_gate_of_the_triggering_action() {
        let mut config = InputConfiguration::legacy();
        config.set_binding(InputDevice::Keyboard, InputAction::Resume, parse("A"));
        config.set_binding(InputDevice::Gamepad, InputAction::Resume, parse("A"));
        let mut manager = InputManager::new(config);

        manager.update(BUTTON_A, 0);
        assert!(!manager.should_resume(BUTTON_A, 0));
        assert!(manager.should_resume(BUTTON_B, BUTTON_A));
    }
}

mod storage {
    use super::*;
    #[test]
    fn missing_configuration_falls_back_to_the_defaults() {
        let loaded = repository::load();

        assert_eq!(loaded.status, LoadStatus::Missing);
        assert_eq!(loaded.config, InputConfiguration::legacy());
        assert_eq!(loaded.path, None);
    }

    #[test]
    fn write_failures_are_reported() {
        let result = repository::save(&InputConfiguration::legacy(), None);

        assert_eq!(result, SaveResult::Failed);
    }

    #[test]
    fn load_status_messages_are_stable() {
        assert_eq!(LoadStatus::Missing.message(), "Default config");
        assert_eq!(LoadStatus::Loaded.message(), "Config loaded");
        assert_eq!(LoadStatus::Invalid.message(), "Config invalid");
        assert_eq!(LoadStatus::UnsupportedVersion.message(), "Config too new");
        assert!(LoadStatus::UnsupportedVersion.blocks_saving());
        assert!(!LoadStatus::Loaded.blocks_saving());
    }
}

mod adversarial {
    use super::*;

    #[test]
    fn config_parser_survives_huge_values() {
        let huge = "A".repeat(10_000);
        let text = format!("version=1\nkeyboard.pause={}\n", huge);

        let parsed = parse_config(&text).expect("huge values must not abort parsing");

        assert_eq!(parsed.warnings, 1);
        assert_eq!(
            parsed.config.binding(InputDevice::Keyboard, InputAction::Pause),
            InputConfiguration::legacy().binding(InputDevice::Keyboard, InputAction::Pause)
        );
    }

    #[test]
    fn config_parser_survives_malformed_lines() {
        let text = "\
version=1
nonsense
=value
keyboard.=ZL
.keyboard.pause=ZL
keyboard.pause.extra=ZL
keyboard.unknown_action=ZL
keyboard.pause==ZL
version=not-a-number
";

        let parsed = parse_config(text).expect("malformed lines must not abort parsing");

        assert!(parsed.warnings >= 3);
        // The malformed binding did not replace the default.
        assert_eq!(
            parsed.config.binding(InputDevice::Keyboard, InputAction::Pause),
            InputConfiguration::legacy().binding(InputDevice::Keyboard, InputAction::Pause)
        );
    }

    #[test]
    fn duplicate_keys_use_the_last_value() {
        let parsed = parse_config("version=1\nkeyboard.pause=ZL\nkeyboard.pause=ZR\n")
            .expect("duplicate keys must not abort parsing");

        assert_eq!(
            parsed.config.binding(InputDevice::Keyboard, InputAction::Pause),
            parse("ZR")
        );
    }

    #[test]
    fn extreme_versions_are_handled() {
        assert_eq!(
            parse_config("version=65535\n"),
            Err(ConfigParseError::UnsupportedVersion(65535))
        );
        assert!(
            parse_config("version=-1\n")
                .expect("negative version is a warning")
                .migrated
        );
        assert!(
            parse_config("version=0\n")
                .expect("version 0 is migrated")
                .migrated
        );
    }

    #[test]
    fn binding_parser_survives_separator_only_input() {
        for text in ["+", "|", "++", "||", "+|", " | + | "] {
            let binding = InputBinding::parse(text).expect("separator-only input is valid");
            assert!(binding.is_unbound(), "{} should be unbound", text);
        }
    }

    #[test]
    fn binding_parser_handles_boundaries() {
        assert!(InputBinding::parse("A+B+X+Y").is_ok());
        assert_eq!(
            InputBinding::parse("A+B+X+Y+L"),
            Err(BindingParseError::TooManyButtons)
        );
        assert!(InputBinding::parse("A|B").is_ok());
        assert_eq!(
            InputBinding::parse("A|B|X"),
            Err(BindingParseError::TooManyAlternatives)
        );
    }

    #[test]
    fn binding_parser_rejects_unicode_and_control_characters() {
        for text in ["Start\u{200b}", "A\u{0000}B", "↑", "Seleçt"] {
            assert_eq!(
                InputBinding::parse(text),
                Err(BindingParseError::UnknownButton),
                "{} should be rejected",
                text.escape_debug()
            );
        }
    }

    #[test]
    fn duplicate_buttons_collapse_into_one_chord() {
        let binding = parse("A+A+A+A");

        assert_eq!(binding.chords().len(), 1);
        assert_eq!(binding.chords()[0].len(), 1);
        assert_eq!(binding.to_config_string(), "A");
    }

    #[test]
    fn parser_never_panics_on_generated_input() {
        // Small deterministic LCG; no external dependencies.
        let alphabet = b"version=active=.-+#| \r\nkeyboard gamepad pause ZL ZR A 0 \\ \t";
        let mut state = 0x1234_5678u32;

        for length in 0..80 {
            let mut text = String::new();
            for _ in 0..length {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let byte = alphabet[(state as usize) % alphabet.len()];
                text.push(byte as char);
            }

            // Both parsers must return instead of panicking.
            let _ = parse_config(&text);
            let _ = InputBinding::parse(&text);
        }
    }

    #[test]
    fn manager_survives_repeated_edges() {
        let held = BUTTON_START | BUTTON_SELECT;
        let mut manager = InputManager::new(InputConfiguration::legacy());

        for _ in 0..50 {
            manager.update(held, 0);
            assert!(manager.should_pause(held, 0));
            manager.update(0, held);
            assert!(!manager.should_pause(0, held));
        }
    }

    #[test]
    fn step_is_consumed_exactly_once() {
        let mut config = InputConfiguration::legacy();
        config.set_binding(InputDevice::Keyboard, InputAction::StepFrame, parse("ZL"));
        config.set_binding(InputDevice::Gamepad, InputAction::StepFrame, parse("ZL"));
        let mut manager = InputManager::new(config);

        manager.update(BUTTON_ZL, 0);
        assert!(!manager.should_step_frame(BUTTON_ZL, 0));
        assert!(manager.should_step_frame(0, BUTTON_ZL));
        assert!(!manager.should_step_frame(0, BUTTON_ZL));
        // A second press arms a second step.
        manager.update(BUTTON_ZL, 0);
        assert!(manager.should_step_frame(0, BUTTON_ZL));
    }
}
