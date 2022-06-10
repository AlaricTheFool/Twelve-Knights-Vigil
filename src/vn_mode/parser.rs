use super::*;
use std::collections::HashMap;

pub fn is_bracketed(input: &str) -> bool {
    input.starts_with("[") && input.ends_with("]")
}

#[derive(Debug, PartialEq)]
pub enum VNParseError {
    InvalidScenePath(String),
    MismatchedBrackets(String),
    UnrecognizedCommand(String),
    InvalidArguments(String, String),
}

#[derive(PartialEq, Debug)]
struct VNBracketParse {
    command: String,
    args: Option<String>,
}

impl VNBracketParse {
    fn arg_error(&self) -> VNParseError {
        if self.args.is_some() {
            VNParseError::InvalidArguments(self.command.clone(), self.args.clone().unwrap())
        } else {
            VNParseError::InvalidArguments(self.command.clone(), "NO ARGS PROVIDED".to_string())
        }
    }

    fn parse_line(input: &str) -> Result<Self, VNParseError> {
        if !is_bracketed(input) {
            return Err(VNParseError::MismatchedBrackets(input.to_string()));
        }

        let input = input.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
        let parts: Vec<&str> = input.split(":").map(|s| s.trim()).collect();

        let args = if parts.len() == 1 {
            None
        } else {
            Some(parts[1].to_string())
        };
        Ok(Self {
            command: parts[0].to_string(),
            args,
        })
    }

    fn has_args(&self) -> Result<(), VNParseError> {
        if self.args.is_none() {
            return Err(VNParseError::InvalidArguments(
                self.command.to_owned(),
                "no args provided".to_string(),
            ));
        }

        Ok(())
    }

    fn to_vn_parse_command(&self) -> Result<VNParseCommand, VNParseError> {
        match self.command.as_str() {
            "DEFINE" => {
                self.has_args()?;

                let arg_string = self.args.clone().unwrap();
                let args: Vec<&str> = arg_string.split("=>").map(|s| s.trim()).collect();

                if args.len() != 2 {
                    return Err(VNParseError::InvalidArguments(
                        self.command.to_owned(),
                        self.args.clone().unwrap(),
                    ));
                }

                Ok(VNParseCommand::SpeakerRename(
                    args[1].to_string(),
                    args[0].to_string(),
                ))
            }

            "SCENE" => {
                self.has_args()?;

                let arg_string = self.args.clone().unwrap();

                Ok(VNParseCommand::ChangeBackground(arg_string))
            }

            "APPEAR" => {
                self.has_args()?;

                let arg_string = self.args.clone().unwrap();

                let side = match arg_string.as_str() {
                    "LEFT" => Side::Left,
                    "RIGHT" => Side::Right,
                    _ => {
                        return Err(self.arg_error());
                    }
                };

                Ok(VNParseCommand::SpeakerDisplayEvent(SpeakerEvent::Appear(
                    side,
                )))
            }
            "HIDE" => Ok(VNParseCommand::SpeakerDisplayEvent(SpeakerEvent::Hide)),

            "SOUND_LOOP" => Ok(VNParseCommand::UnimplementedCommand(
                self.command.to_string(),
            )),

            _ => {
                if let Ok(speaker) = Speaker::from_key(self.command.as_str()) {
                    Ok(VNParseCommand::DefineSpeaker(speaker))
                } else {
                    Err(VNParseError::UnrecognizedCommand(self.command.to_owned()))
                }
            }
        }
    }

    fn with_redefines(&self, map: &HashMap<String, String>) -> Self {
        let mut command = self.command.to_owned();

        while map.contains_key(&command) {
            command = map[&command].to_owned();
        }

        Self {
            command,
            args: self.args.clone(),
        }
    }
}

#[derive(Debug)]
pub enum VNParseCommand {
    SpeakerRename(String, String),
    DefineSpeaker(Speaker),
    ChangeBackground(String),
    SpeakerDisplayEvent(SpeakerEvent),
    UnimplementedCommand(String),
}

pub fn parse_text(input: &str) -> Result<Vec<VNEvent>, VNParseError> {
    let mut renames = HashMap::new();
    let mut result = Vec::new();
    let mut current_speaker = Speaker::unknown();
    let mut current_line = "".to_string();

    input.lines().try_for_each(|line| {
        if is_bracketed(line) {
            let parse = VNBracketParse::parse_line(line)?;
            let parse = parse.with_redefines(&renames);
            let command = parse.to_vn_parse_command()?;

            match command {
                VNParseCommand::SpeakerRename(from, to) => {
                    renames.insert(from, to);
                },

                VNParseCommand::DefineSpeaker(new_speaker) => {
                    current_speaker = new_speaker;
                },

                VNParseCommand::ChangeBackground(new_bg_key) => {
                    result.push(VNEvent::ChangeBackground(new_bg_key));
                }

                VNParseCommand::SpeakerDisplayEvent(event) => {
                    result.push(VNEvent::ChangeSpeakerDisplay(current_speaker.clone(), event));
                }

                VNParseCommand::UnimplementedCommand(cmd) => {
                    error!("The command '{cmd}' has not yet been implemented but is a planned feature.");
                }
            }
        } else {
            if line != "" {
                current_line.push_str(line);
            } else if current_line != "" {
                let dialogue = VNEvent::Dialogue(current_speaker.clone(), current_line.clone());
                result.push(dialogue);
                current_line = "".to_string();
            }
        }

        Ok(())
    })?;

    Ok(result)
}

pub fn load_scene(scene_path: &str) -> Result<Vec<VNEvent>, VNParseError> {
    use std::fs::File;
    use std::io::prelude::*;

    let formatted_path = format!("assets/vn_scenes/{}.dlog", scene_path);

    if let Ok(mut file) = File::open(&formatted_path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        parse_text(&contents)
    } else {
        Err(VNParseError::InvalidScenePath(scene_path.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bracket_line_success() -> Result<(), VNParseError> {
        let input = "[COMMAND: ARG ARG2 ARG3]";
        let actual = VNBracketParse::parse_line(&input)?;
        let expected = VNBracketParse {
            command: "COMMAND".to_string(),
            args: Some("ARG ARG2 ARG3".to_string()),
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_bracket_line_no_args() -> Result<(), VNParseError> {
        let input = "[OTHER_COMMAND]";
        let actual = VNBracketParse::parse_line(&input)?;
        let expected = VNBracketParse {
            command: "OTHER_COMMAND".to_string(),
            args: None,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_bracket_bracket_error() -> Result<(), VNParseError> {
        let input = "[NO_CLOSING_BRACKET";
        let actual = VNBracketParse::parse_line(&input).unwrap_err();
        let expected = VNParseError::MismatchedBrackets(input.to_string());

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_invalid_command() -> Result<(), VNParseError> {
        let input = "[INVALIDCOMMAND]";
        let parse = VNBracketParse::parse_line(&input)?;
        let actual = parse.to_vn_parse_command().unwrap_err();
        let expected = VNParseError::UnrecognizedCommand("INVALIDCOMMAND".to_string());

        assert_eq!(actual, expected);
        Ok(())
    }
}
