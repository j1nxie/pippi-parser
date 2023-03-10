use crate::Beatmap;

#[derive(Default, Debug, PartialEq)]
pub struct Metadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: Vec<String>,
    pub beatmap_id: u32,
    pub beatmap_set_id: u32,
}

#[allow(clippy::too_many_arguments)]
impl Metadata {
    pub fn new(
        title: String,
        title_unicode: String,
        artist: String,
        artist_unicode: String,
        creator: String,
        version: String,
        source: String,
        tags: Vec<String>,
        beatmap_id: u32,
        beatmap_set_id: u32,
    ) -> Self {
        Self {
            title,
            title_unicode,
            artist,
            artist_unicode,
            creator,
            version,
            source,
            tags,
            beatmap_id,
            beatmap_set_id,
        }
    }
}

pub fn parse_metadata(line: &str, beatmap: &mut Beatmap) {
    let (k, v) = line.split_once(':').unwrap();
    match k.trim() {
        "Title" => beatmap.metadata.title = String::from(v.trim()),
        "TitleUnicode" => beatmap.metadata.title_unicode = String::from(v.trim()),
        "Artist" => beatmap.metadata.artist = String::from(v.trim()),
        "ArtistUnicode" => beatmap.metadata.artist_unicode = String::from(v.trim()),
        "Creator" => beatmap.metadata.creator = String::from(v.trim()),
        "Version" => beatmap.metadata.version = String::from(v.trim()),
        "Source" => beatmap.metadata.source = String::from(v.trim()),
        "Tags" => beatmap.metadata.tags = v.trim().split(' ').map(String::from).collect(),
        "BeatmapID" => beatmap.metadata.beatmap_id = v.trim().parse::<u32>().unwrap(),
        "BeatmapSetID" => beatmap.metadata.beatmap_set_id = v.trim().parse::<u32>().unwrap(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Difficulty, Editor, Format, General};

    #[test]
    fn test_parse_metadata() {
        let test_str = "Title:End Time
            TitleUnicode:End Time
            Artist:Cres
            ArtistUnicode:Cres
            Creator:PaRaDogi
            Version:Dogi
            Source:
            Tags:DeviousPanda
            BeatmapID:2797865
            BeatmapSetID:1351450";
        let mut beatmap = Beatmap::default();
        for line in test_str.lines() {
            parse_metadata(line, &mut beatmap);
        }

        assert_eq!(
            beatmap,
            Beatmap {
                format: Format::default(),
                general: General::default(),
                editor: Editor::default(),
                metadata: Metadata {
                    title: String::from("End Time"),
                    title_unicode: String::from("End Time"),
                    artist: String::from("Cres"),
                    artist_unicode: String::from("Cres"),
                    creator: String::from("PaRaDogi"),
                    version: String::from("Dogi"),
                    source: String::new(),
                    tags: vec![String::from("DeviousPanda")],
                    beatmap_id: 2797865,
                    beatmap_set_id: 1351450,
                },
                difficulty: Difficulty::default(),
                timing_points: Vec::new(),
                hit_objects: Vec::new(),
            }
        );
    }
}
