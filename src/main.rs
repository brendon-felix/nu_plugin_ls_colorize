use lscolors::{LsColors, Style};
use nu_ansi_term::Style as AnsiStyle;
use nu_path::{expand_path_with, expand_to_real_path};
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, Type, Value};
use nu_utils::get_ls_colors;
use std::path::Path;

mod color_utils;

#[macro_export]
macro_rules! type_error {
    ($value:expr, $call:expr) => {
        LabeledError::new("Expected String or List of Strings").with_label(
            format!(
                "requires string or list of strings; got {}",
                $value.get_type()
            ),
            $call.head,
        )
    };
}

pub struct LsColorizePlugin;

impl Plugin for LsColorizePlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(LsColorize)]
    }
}

pub struct LsColorize;

impl SimplePluginCommand for LsColorize {
    type Plugin = LsColorizePlugin;

    fn name(&self) -> &str {
        "ls-colorize"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .category(Category::Path)
            .switch(
                "get-color",
                "Return the color instead of the colored string",
                Some('c'),
            )
            .input_output_types(vec![
                (Type::String, Type::String),
                (
                    Type::List(Box::new(Type::String)),
                    Type::List(Box::new(Type::String)),
                ),
                (Type::String, Type::Record(vec![].into())),
                (
                    Type::List(Box::new(Type::String)),
                    Type::List(Box::new(Type::Record(vec![].into()))),
                ),
            ])
    }

    fn description(&self) -> &str {
        "Color a path based on LS_COLORS environment variable"
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                example: "\"file.txt\" | ls-colorize",
                description: "Color a single file path based on LS_COLORS",
                result: None,
            },
            Example {
                example: "[\"file.txt\" \"directory\" \"script.sh\"] | ls-colorize",
                description: "Color multiple file paths based on LS_COLORS",
                result: None,
            },
            Example {
                example: "ls | get name | ls-colorize",
                description: "Color file names from ls output",
                result: None,
            },
            Example {
                example: "\"file.txt\" | ls-colorize --get-color",
                description: "Get the color record for a single file path",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &LsColorizePlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let ls_colors_str = engine
            .get_env_var("LS_COLORS")?
            .and_then(|v| v.as_str().ok().map(|s| s.to_string()));
        let ls_colors = get_ls_colors(ls_colors_str);
        let cwd = engine.get_current_dir()?;
        let get_color = call.has_flag("get-color")?;

        match input {
            Value::String { val: path, .. } => {
                let style = get_style(&path, &cwd, &ls_colors);
                if get_color {
                    Ok(color_utils::ansi_style_to_record(style, call.head))
                } else {
                    Ok(Value::string(style.paint(path).to_string(), call.head))
                }
            }
            Value::List { vals, .. } => {
                let mut results = vec![];
                for v in vals {
                    match v {
                        Value::String { val: path, .. } => {
                            let style = get_style(&path, &cwd, &ls_colors);
                            if get_color {
                                results.push(color_utils::ansi_style_to_record(style, call.head));
                            } else {
                                results
                                    .push(Value::string(style.paint(path).to_string(), call.head));
                            }
                        }
                        _ => return Err(type_error!(v, call)),
                    }
                }
                Ok(Value::list(results, call.head))
            }
            _ => Err(type_error!(input, call)),
        }
    }
}

pub fn get_style(path: &str, cwd: &str, ls_colors: &LsColors) -> AnsiStyle {
    // lifted from nu-explore/src/explore/nu_common/lscolor.rs
    let mut style = ls_colors.style_for_str(path);
    let is_likely_dir = style.is_none();
    if is_likely_dir {
        let mut meta = std::fs::symlink_metadata(path).ok();
        if meta.is_none() {
            let mut expanded_path = expand_to_real_path(path);
            let try_cwd = expanded_path.as_path() == Path::new(path);
            if try_cwd {
                let cwd_path = format!("./{path}");
                expanded_path = expand_path_with(cwd_path, cwd, false);
            }
            meta = std::fs::symlink_metadata(expanded_path.as_path()).ok();
            style = ls_colors.style_for_path_with_metadata(expanded_path.as_path(), meta.as_ref());
        } else {
            style = ls_colors.style_for_path_with_metadata(path, meta.as_ref());
        }
    }
    style.map(Style::to_nu_ansi_term_style).unwrap_or_default()
}

fn main() {
    serve_plugin(&LsColorizePlugin, MsgPackSerializer);
}
