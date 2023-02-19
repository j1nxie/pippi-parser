use crate::file_sections::{get_section, FileSections};

mod file_sections;

#[derive(Debug, PartialEq)]
pub struct Difficulty {
    hp: f32,
    cs: f32,
    od: f32,
    ar: f32,
    slider_multiplier: f32,
    slider_tickrate: f32,
}

impl Difficulty {
    fn new() -> Difficulty {
        Difficulty {
            hp: 5.0,
            cs: 5.0,
            od: 5.0,
            ar: 5.0,
            slider_multiplier: 1.4,
            slider_tickrate: 1.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct General {
    audio_filename: String,
    audio_lead_in: u32,
    audio_hash: String,
    preview_time: i32,
    countdown: u32,     // TODO: this should be an enum
    sample_set: String, // TODO: this should also be an enum
    stack_leniency: f32,
    mode: u32, // TODO: think about whether this should be an enum
    letterbox_in_breaks: bool,
    story_fire_in_front: bool,
    use_skin_sprites: bool,
    always_show_playfield: bool,
    overlay_position: String, // TODO: this probably should be an enum
    skin_preference: String,
    epilepsy_warning: bool,
    countdown_offset: u32,
    special_style: bool,
    widescreen_storyboard: bool,
    samples_match_playback_rate: bool,
}

impl General {
    fn new() -> General {
        General {
            audio_filename: String::new(),
            audio_lead_in: 0,
            audio_hash: String::new(),
            preview_time: -1,
            countdown: 1,
            sample_set: String::from("Normal"),
            stack_leniency: 0.7,
            mode: 0,
            letterbox_in_breaks: false,
            story_fire_in_front: true,
            use_skin_sprites: false,
            always_show_playfield: false,
            overlay_position: String::from("NoChange"),
            skin_preference: String::new(),
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Beatmap {
    general: General,
    difficulty: Difficulty,
}

impl Beatmap {
    fn new() -> Beatmap {
        Beatmap {
            general: General::new(),
            difficulty: Difficulty::new(),
        }
    }
}

pub fn parse(text: &str) -> Beatmap {
    let mut beatmap = Beatmap::new();
    let mut current_section = FileSections::Format;

    for line in text.lines() {
        if line.trim().is_empty() && !line.starts_with("//") {
            if get_section(line) != FileSections::None(line.to_string()) {
                current_section = get_section(line);
            } else {
                match current_section {
                    FileSections::General => parse_general(line, &mut beatmap),
                    FileSections::Difficulty => parse_difficulty(line, &mut beatmap),
                    _ => todo!(),
                }
            }
        }
    }

    beatmap
}

fn parse_general(line: &str, beatmap: &mut Beatmap) {
    let (k, v) = line.split_once(':').unwrap();
    match k.trim() {
        "AudioFilename" => beatmap.general.audio_filename = String::from(v.trim()),
        "AudioLeadIn" => beatmap.general.audio_lead_in = v.trim().parse::<u32>().unwrap(),
        "AudioHash" => beatmap.general.audio_hash = String::from(v.trim()),
        "PreviewTime" => beatmap.general.preview_time = v.trim().parse::<i32>().unwrap(),
        "Countdown" => beatmap.general.countdown = v.trim().parse::<u32>().unwrap(),
        "SampleSet" => beatmap.general.sample_set = String::from(v.trim()),
        "StackLeniency" => beatmap.general.stack_leniency = v.trim().parse::<f32>().unwrap(),
        "Mode" => beatmap.general.mode = v.trim().parse::<u32>().unwrap(),
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
        "OverlayPosition" => beatmap.general.overlay_position = String::from(v.trim()),
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

fn parse_difficulty(line: &str, beatmap: &mut Beatmap) {
    let (k, v) = line.split_once(':').unwrap();
    match k.trim() {
        "HPDrainRate" => beatmap.difficulty.hp = v.trim().parse::<f32>().unwrap(),
        "CircleSize" => beatmap.difficulty.cs = v.trim().parse::<f32>().unwrap(),
        "OverallDifficulty" => beatmap.difficulty.od = v.trim().parse::<f32>().unwrap(),
        "ApproachRate" => beatmap.difficulty.ar = v.trim().parse::<f32>().unwrap(),
        "SliderMultiplier" => {
            beatmap.difficulty.slider_multiplier = v.trim().parse::<f32>().unwrap()
        }
        "SliderTickRate" => beatmap.difficulty.slider_tickrate = v.trim().parse::<f32>().unwrap(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut beatmap = Beatmap::new();
        for line in test_str.lines() {
            parse_general(line, &mut beatmap)
        }

        assert_eq!(
            beatmap,
            Beatmap {
                general: General {
                    audio_filename: String::from("audio.mp3"),
                    audio_lead_in: 0,
                    audio_hash: String::from("afjskldfjaldksfjklasf"),
                    preview_time: 10,
                    countdown: 0,
                    sample_set: String::from("Drum"),
                    stack_leniency: 0.75,
                    mode: 1,
                    letterbox_in_breaks: true,
                    story_fire_in_front: false,
                    use_skin_sprites: true,
                    always_show_playfield: true,
                    overlay_position: String::from("Below"),
                    skin_preference: String::from("Seoul v10"),
                    epilepsy_warning: true,
                    countdown_offset: 1,
                    special_style: true,
                    widescreen_storyboard: true,
                    samples_match_playback_rate: true,
                },
                difficulty: Difficulty::new(),
            }
        );
    }

    #[test]
    fn test_parse_difficulty() {
        let test_str = "HPDrainRate:6.0
            CircleSize:4.2
            OverallDifficulty:8.5
            ApproachRate:9.8
            SliderMultiplier:1.4
            SliderTickRate:2.0";
        let mut beatmap = Beatmap::new();
        for line in test_str.lines() {
            parse_difficulty(line, &mut beatmap);
        }

        assert_eq!(
            beatmap,
            Beatmap {
                general: General::new(),
                difficulty: Difficulty {
                    hp: 6.0,
                    cs: 4.2,
                    od: 8.5,
                    ar: 9.8,
                    slider_multiplier: 1.4,
                    slider_tickrate: 2.0,
                }
            }
        );
    }
}
