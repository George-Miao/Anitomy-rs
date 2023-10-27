use std::collections::HashMap;

use widestring::Utf32String;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ElementCategory {
    AnimeSeason,
    AnimeSeasonPrefix,
    AnimeTitle,
    AnimeType,
    AnimeYear,
    AudioTerm,
    DeviceCompatibility,
    EpisodeNumber,
    EpisodeNumberAlt,
    EpisodePrefix,
    EpisodeTitle,
    FileChecksum,
    FileExtension,
    FileName,
    Language,
    Other,
    ReleaseGroup,
    ReleaseInformation,
    ReleaseVersion,
    Source,
    Subtitles,
    VideoResolution,
    VideoTerm,
    VolumeNumber,
    VolumePrefix,
    Unknown,
}

pub struct Elements {
    elements: HashMap<ElementCategory, Utf32String>,
}

impl std::ops::DerefMut for Elements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl std::ops::Deref for Elements {
    type Target = HashMap<ElementCategory, Utf32String>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}
