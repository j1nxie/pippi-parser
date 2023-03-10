use crate::Beatmap;
use std::str::FromStr;
use strum::ParseError::VariantNotFound;
use strum_macros::EnumString;

#[derive(Debug, PartialEq)]
pub enum Countdown {
    None,
    Normal,
    Half,
    Double,
}

impl FromStr for Countdown {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<i32>() {
            Ok(t) => match t {
                0 => Ok(Countdown::None),
                1 => Ok(Countdown::Normal),
                2 => Ok(Countdown::Half),
                3 => Ok(Countdown::Double),
                _ => Err(VariantNotFound),
            },
            Err(_) => Err(VariantNotFound),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SampleSet {
    Default,
    Normal,
    Soft,
    Drum,
}

impl FromStr for SampleSet {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "0" => Ok(Self::Default),
            "1" | "Normal" => Ok(Self::Default),
            "2" | "Soft" => Ok(Self::Soft),
            "3" | "Drum" => Ok(Self::Drum),
            _ => Err(VariantNotFound),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Osu,
    Taiko,
    Catch,
    Mania,
}

impl FromStr for Mode {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<i32>() {
            Ok(t) => match t {
                0 => Ok(Mode::Osu),
                1 => Ok(Mode::Taiko),
                2 => Ok(Mode::Catch),
                3 => Ok(Mode::Mania),
                _ => Err(VariantNotFound),
            },
            Err(_) => Err(VariantNotFound),
        }
    }
}

#[derive(Debug, PartialEq, EnumString)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}

#[derive(Debug, PartialEq)]
pub struct General {
    pub audio_filename: String,
    pub audio_lead_in: u32,
    pub audio_hash: String,
    pub preview_time: i32,
    pub countdown: Countdown,
    pub sample_set: SampleSet,
    pub stack_leniency: f32,
    pub mode: Mode,
    pub letterbox_in_breaks: bool,
    pub story_fire_in_front: bool,
    pub use_skin_sprites: bool,
    pub always_show_playfield: bool,
    pub overlay_position: OverlayPosition,
    pub skin_preference: String,
    pub epilepsy_warning: bool,
    pub countdown_offset: u32,
    pub special_style: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback_rate: bool,
}

impl General {
    pub fn new() -> Self {
        Self {
            audio_filename: String::new(),
            audio_lead_in: 0,
            audio_hash: String::new(),
            preview_time: -1,
            countdown: Countdown::Normal,
            sample_set: SampleSet::Normal,
            stack_leniency: 0.7,
            mode: Mode::Osu,
            letterbox_in_breaks: false,
            story_fire_in_front: true,
            use_skin_sprites: false,
            always_show_playfield: false,
            overlay_position: OverlayPosition::NoChange,
            skin_preference: String::new(),
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
}

impl Default for General {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_general(line: &str, beatmap: &mut Beatmap) {
    let (k, v) = line.split_once(':').unwrap();
    match k.trim() {
        "AudioFilename" => beatmap.general.audio_filename = String::from(v.trim()),
        "AudioLeadIn" => beatmap.general.audio_lead_in = v.trim().parse::<u32>().unwrap(),
        "AudioHash" => beatmap.general.audio_hash = String::from(v.trim()),
        "PreviewTime" => beatmap.general.preview_time = v.trim().parse::<i32>().unwrap(),
        "Countdown" => beatmap.general.countdown = Countdown::from_str(v.trim()).unwrap(),
        "SampleSet" => beatmap.general.sample_set = SampleSet::from_str(v.trim()).unwrap(),
        "StackLeniency" => beatmap.general.stack_leniency = v.trim().parse::<f32>().unwrap(),
        "Mode" => beatmap.general.mode = Mode::from_str(v.trim()).unwrap(),
        "LetterboxInBreaks" => {
            beatmap.general.letterbox_in_breaks = v.trim().parse::<u8>().unwrap() != 0
        }
        "StoryFireInFront" => {
            beatmap.general.story_fire_in_front = v.trim().parse::<u8>().unwrap() != 0
        }
        "UseSkinSprites" => beatmap.general.use_skin_sprites = v.trim().parse::<u8>().unwrap() != 0,
        "AlwaysShowPlayfield" => {
            beatmap.general.always_show_playfield = v.trim().parse::<u8>().unwrap() != 0
        }
        "OverlayPosition" => {
            beatmap.general.overlay_position = OverlayPosition::from_str(v.trim()).unwrap()
        }
        "SkinPreference" => beatmap.general.skin_preference = String::from(v.trim()),
        "EpilepsyWarning" => {
            beatmap.general.epilepsy_warning = v.trim().parse::<u8>().unwrap() != 0
        }
        "CountdownOffset" => beatmap.general.countdown_offset = v.trim().parse::<u32>().unwrap(),
        "SpecialStyle" => beatmap.general.special_style = v.trim().parse::<u8>().unwrap() != 0,
        "WidescreenStoryboard" => {
            beatmap.general.widescreen_storyboard = v.trim().parse::<u8>().unwrap() != 0
        }
        "SamplesMatchPlaybackRate" => {
            beatmap.general.samples_match_playback_rate = v.trim().parse::<u8>().unwrap() != 0
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Difficulty, Editor, Format, Metadata};

    #[test]
    fn test_parse_general() {
        let test_str = "AudioFilename: audio.mp3
            AudioLeadIn: 0
            AudioHash: afjskldfjaldksfjklasf
            PreviewTime: 10
            Countdown: 0
            SampleSet: Drum
            StackLeniency: 0.75
            Mode: 1
            LetterboxInBreaks: 1
            StoryFireInFront: 0
            UseSkinSprites: 1
            AlwaysShowPlayfield: 1
            OverlayPosition: Below
            SkinPreference: Seoul v10
            EpilepsyWarning: 1
            CountdownOffset: 1
            SpecialStyle: 1
            WidescreenStoryboard: 1
            SamplesMatchPlaybackRate: 1";
        let mut beatmap = Beatmap::default();
        for line in test_str.lines() {
            parse_general(line, &mut beatmap)
        }

        assert_eq!(
            beatmap,
            Beatmap {
                format: Format::default(),
                general: General {
                    audio_filename: String::from("audio.mp3"),
                    audio_lead_in: 0,
                    audio_hash: String::from("afjskldfjaldksfjklasf"),
                    preview_time: 10,
                    countdown: Countdown::None,
                    sample_set: SampleSet::Drum,
                    stack_leniency: 0.75,
                    mode: Mode::Taiko,
                    letterbox_in_breaks: true,
                    story_fire_in_front: false,
                    use_skin_sprites: true,
                    always_show_playfield: true,
                    overlay_position: OverlayPosition::Below,
                    skin_preference: String::from("Seoul v10"),
                    epilepsy_warning: true,
                    countdown_offset: 1,
                    special_style: true,
                    widescreen_storyboard: true,
                    samples_match_playback_rate: true,
                },
                editor: Editor::default(),
                metadata: Metadata::default(),
                difficulty: Difficulty::default(),
                timing_points: Vec::new(),
                hit_objects: Vec::new(),
            }
        );
    }
}
