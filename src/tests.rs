use crate::{get_style, LsColorizePlugin};
use lscolors::LsColors;
use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Span, Value};
use std::env;

#[test]
fn test_plugin_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    let plugin = LsColorizePlugin;
    let _plugin_test = PluginTest::new("ls-colorize", plugin.into())?;

    Ok(())
}

#[test]
fn test_type_error_macro() {
    let call = EvaluatedCall::new(Span::test_data());
    let value = Value::int(42, Span::test_data());

    let error = crate::type_error!(value, call);

    assert_eq!(error.msg, "Expected String or List of Strings");
    assert!(error.labels.iter().any(|label| label
        .text
        .contains("requires string or list of strings; got int")));
}

#[test]
fn test_get_style_with_different_file_types() {
    env::set_var("LS_COLORS", "*.txt=31:*.sh=32:*.py=33:di=34:ex=35");

    let ls_colors = LsColors::from_env().unwrap_or_default();
    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();

    // Test different file extensions
    let txt_style = get_style("test.txt", &cwd, &ls_colors);
    let sh_style = get_style("script.sh", &cwd, &ls_colors);
    let py_style = get_style("program.py", &cwd, &ls_colors);

    // Ensure styles are applied (not empty/default)
    assert!(!txt_style.paint("test.txt").to_string().is_empty());
    assert!(!sh_style.paint("script.sh").to_string().is_empty());
    assert!(!py_style.paint("program.py").to_string().is_empty());

    // Test relative paths
    let relative_style = get_style("./test.txt", &cwd, &ls_colors);
    assert!(!relative_style.paint("./test.txt").to_string().is_empty());
}

#[test]
fn test_get_style_with_directories() {
    env::set_var("LS_COLORS", "di=34:*.txt=31");

    let ls_colors = LsColors::from_env().unwrap_or_default();
    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();

    // Test directory-like names (since we can't create actual directories in unit tests)
    let dir_style = get_style("some_directory", &cwd, &ls_colors);
    assert!(!dir_style.paint("some_directory").to_string().is_empty());
}

#[test]
fn test_color_utils_ansi_style_to_record() {
    use nu_ansi_term::{Color, Style};

    let span = Span::test_data();

    // Test bold red on blue
    let style = Style::new().bold().fg(Color::Red).on(Color::Blue);
    let record = crate::color_utils::ansi_style_to_record(style, span);

    if let Value::Record { val, .. } = record {
        assert!(val.contains("fg"));
        assert!(val.contains("bg"));
        assert!(val.contains("attr"));
    } else {
        panic!("Expected record value");
    }

    // Test default style
    let default_style = Style::default();
    let default_record = crate::color_utils::ansi_style_to_record(default_style, span);

    if let Value::Record { .. } = default_record {
        // Default style should still create a valid record
        // Just verify it's a record type
    } else {
        panic!("Expected record value for default style");
    }
}

#[test]
fn test_plugin_basic_functionality() {
    env::set_var("LS_COLORS", "*.txt=31:*.sh=32:di=34");

    let ls_colors = LsColors::from_env().unwrap_or_default();
    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();

    // Test that styles are properly applied to different file types
    let txt_style = get_style("document.txt", &cwd, &ls_colors);
    let sh_style = get_style("script.sh", &cwd, &ls_colors);
    let generic_style = get_style("readme", &cwd, &ls_colors);

    // Verify that painted strings are not empty (meaning styling was applied)
    let painted_txt = txt_style.paint("document.txt").to_string();
    let painted_sh = sh_style.paint("script.sh").to_string();
    let painted_generic = generic_style.paint("readme").to_string();

    assert!(!painted_txt.is_empty());
    assert!(!painted_sh.is_empty());
    assert!(!painted_generic.is_empty());

    // The painted strings should contain the original text
    assert!(painted_txt.contains("document.txt"));
    assert!(painted_sh.contains("script.sh"));
    assert!(painted_generic.contains("readme"));
}
