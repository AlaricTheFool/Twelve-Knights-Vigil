use super::*;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take, take_until, take_while};
use nom::character::complete::char;
use nom::combinator::{eof, opt};
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{Err, IResult};

fn square_brackets(input: &str) -> IResult<&str, &str> {
    delimited(char('['), is_not("]"), char(']'))(input)
}

fn define_speaker(input: &str) -> IResult<&str, String> {
    let (remaining, cmd) = square_brackets(input)?;
    let (cmd, _) = tag("DEFINE_SPEAKER:")(cmd)?;
    let (_, (with, replace)) = separated_pair(is_not("=>"), tag("=>"), is_not("\t\r\n"))(cmd)?;

    let with = &format!("[{}]", with.trim());
    let replace = &format!("[{}]", replace.trim());

    eprintln!("'{}' with '{}'", replace.trim(), with.trim());
    let replaced = remaining.replace(replace.trim(), with.trim());
    let replaced = replaced.trim().to_string();
    Ok(("", replaced))
}

fn parse_dialogue(input: &str) -> IResult<&str, VNEvent> {
    let (remaining, speaker_key) = square_brackets(input)?;
    let (remaining, dialogue) = take_until("[")(remaining)?;
    eprintln!("Speaker: '{speaker_key}'\nDialogue: '{dialogue}'\nRemaining: '{remaining}'");

    if let Ok(speaker) = Speaker::from_key(speaker_key) {
        let event = VNEvent::Dialogue(speaker, dialogue.trim().to_string());
        Ok((remaining, event))
    } else {
        Err(Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Fail,
        )))
    }
}

fn sp(input: &str) -> IResult<&str, &str> {
    let chars = " \t\n\r";

    take_while(move |c| chars.contains(c))(input)
}

fn parse_scene(input: &str) -> IResult<&str, Vec<VNEvent>> {
    terminated(many0(parse_dialogue), eof)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_brackets() {
        let input = "[SOUND_LOOP: BIRD_FX_1]";
        let expected = "SOUND_LOOP: BIRD_FX_1";
        let result = square_brackets(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_define_speaker() {
        let input = "[DEFINE_SPEAKER: PLAYER => P] [P] PPP";
        let expected = "[PLAYER] PPP".to_string();
        let result = define_speaker(input);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_dialogue_line() {
        let input = "[PLAYER] Blargasnarg\nBorganorg[Butts]";
        let expected = VNEvent::Dialogue(Speaker::player(), "Blargasnarg\nBorganorg".to_string());

        let result = parse_dialogue(input);
        assert_eq!(result, Ok(("[Butts]", expected)))
    }

    #[test]
    #[ignore]
    fn test_big() {
        let input = include_bytes!("../../assets/vn_scenes/test_scenes/test_dialogue.dlog");
        let result = parse_scene(std::str::from_utf8(input).unwrap());
        eprintln!("{:?}", result.unwrap());

        assert!(false);
    }
}
