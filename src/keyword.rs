use std::{cell::OnceCell, collections::HashMap, sync::OnceLock};

use widestring::{utf32str, Utf32Str, Utf32String};

use crate::{search_slice, ElementCategory, Elements, TokenRange};

const OPTIONS_DEFAULT: KeywordOptions = KeywordOptions::const_default();
const OPTIONS_INVALID: KeywordOptions = KeywordOptions::new(true, true, false);
const OPTIONS_UNIDENTIFIABLE: KeywordOptions = KeywordOptions::new(false, true, true);
const OPTIONS_UNIDENTIFIABLE_INVALID: KeywordOptions = KeywordOptions::new(false, true, false);
const OPTIONS_UNIDENTIFIABLE_UNSEARCHABLE: KeywordOptions = KeywordOptions::new(false, false, true);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeywordOptions {
    identifiable: bool,
    searchable: bool,
    valid: bool,
}

impl KeywordOptions {
    pub const fn new(identifiable: bool, searchable: bool, valid: bool) -> Self {
        KeywordOptions {
            identifiable,
            searchable,
            valid,
        }
    }

    pub const fn const_default() -> Self {
        KeywordOptions {
            identifiable: true,
            searchable: true,
            valid: true,
        }
    }
}

impl const Default for KeywordOptions {
    fn default() -> Self {
        Self::const_default()
    }
}

pub struct Keyword {
    category: ElementCategory,
    options: KeywordOptions,
}

pub struct KeywordManager {
    key: HashMap<Utf32String, Keyword>,
    ext: HashMap<Utf32String, Keyword>,
}

impl KeywordManager {
    pub fn instance<'a>() -> &'a Self {
        static INSTANCE: OnceLock<KeywordManager> = OnceLock::new();

        INSTANCE.get_or_init(|| {
            let mut this = KeywordManager {
                key: HashMap::new(),
                ext: HashMap::new(),
            };
            this.add(
                ElementCategory::AnimeSeasonPrefix,
                OPTIONS_UNIDENTIFIABLE,
                [utf32str!("SAISON"), utf32str!("SEASON")],
            )
            .add(
                ElementCategory::AnimeType,
                OPTIONS_UNIDENTIFIABLE,
                [
                    utf32str!("GEKIJOUBAN"),
                    utf32str!("MOVIE"),
                    utf32str!("OAD"),
                    utf32str!("OAV"),
                    utf32str!("ONA"),
                    utf32str!("OVA"),
                    utf32str!("SPECIAL"),
                    utf32str!("SPECIALS"),
                    utf32str!("TV"),
                ],
            )
            .add(
                ElementCategory::AnimeType,
                OPTIONS_UNIDENTIFIABLE_UNSEARCHABLE,
                [utf32str!("SP")],
            ) // e.g. "Yumeiro Patissiere SP Professional"
            .add(
                ElementCategory::AnimeType,
                OPTIONS_UNIDENTIFIABLE_INVALID,
                [
                    utf32str!("ED"),
                    utf32str!("ENDING"),
                    utf32str!("NCED"),
                    utf32str!("NCOP"),
                    utf32str!("OP"),
                    utf32str!("OPENING"),
                    utf32str!("PREVIEW"),
                    utf32str!("PV"),
                ],
            )
            .add(
                ElementCategory::AudioTerm,
                OPTIONS_DEFAULT,
                [
                    // Audio channels
                    utf32str!("2.0CH"),
                    utf32str!("2CH"),
                    utf32str!("5.1"),
                    utf32str!("5.1CH"),
                    utf32str!("7.1"),
                    utf32str!("7.1CH"),
                    utf32str!("DTS"),
                    utf32str!("DTS-ES"),
                    utf32str!("DTS5.1"),
                    utf32str!("DOLBY TRUEHD"),
                    utf32str!("TRUEHD"),
                    utf32str!("TRUEHD5.1"),
                    // Audio codec
                    utf32str!("AAC"),
                    utf32str!("AACX2"),
                    utf32str!("AACX3"),
                    utf32str!("AACX4"),
                    utf32str!("AC3"),
                    utf32str!("EAC3"),
                    utf32str!("E-AC-3"),
                    utf32str!("FLAC"),
                    utf32str!("FLACX2"),
                    utf32str!("FLACX3"),
                    utf32str!("FLACX4"),
                    utf32str!("LOSSLESS"),
                    utf32str!("MP3"),
                    utf32str!("OGG"),
                    utf32str!("VORBIS"),
                    utf32str!("ATMOS"),
                    utf32str!("DOLBY ATMOS"),
                    // Audio language
                    utf32str!("DUALAUDIO"),
                    utf32str!("DUAL AUDIO"),
                ],
            )
            .add(
                ElementCategory::AudioTerm,
                OPTIONS_UNIDENTIFIABLE,
                [utf32str!("OPUS")],
            ) // e.g. "Opus.COLORs"
            .add(
                ElementCategory::DeviceCompatibility,
                OPTIONS_DEFAULT,
                [
                    utf32str!("IPAD3"),
                    utf32str!("IPHONE5"),
                    utf32str!("IPOD"),
                    utf32str!("PS3"),
                    utf32str!("XBOX"),
                    utf32str!("XBOX360"),
                ],
            )
            .add(
                ElementCategory::DeviceCompatibility,
                OPTIONS_UNIDENTIFIABLE,
                [utf32str!("ANDROID")],
            )
            .add(
                ElementCategory::EpisodePrefix,
                OPTIONS_DEFAULT,
                [
                    utf32str!("EP"),
                    utf32str!("EP."),
                    utf32str!("EPS"),
                    utf32str!("EPS."),
                    utf32str!("EPISODE"),
                    utf32str!("EPISODE."),
                    utf32str!("EPISODES"),
                    utf32str!("CAPITULO"),
                    utf32str!("EPISODIO"),
                    utf32str!("EPIS\u{00F3}DIO"),
                    utf32str!("FOLGE"),
                ],
            )
            .add(
                ElementCategory::EpisodePrefix,
                OPTIONS_INVALID,
                [utf32str!("E"), utf32str!("第")],
            ) // single-letter episode keywords are not valid tokens
            .add(
                ElementCategory::FileExtension,
                OPTIONS_DEFAULT,
                [
                    utf32str!("3GP"),
                    utf32str!("AVI"),
                    utf32str!("DIVX"),
                    utf32str!("FLV"),
                    utf32str!("M2TS"),
                    utf32str!("MKV"),
                    utf32str!("MOV"),
                    utf32str!("MP4"),
                    utf32str!("MPG"),
                    utf32str!("OGM"),
                    utf32str!("RM"),
                    utf32str!("RMVB"),
                    utf32str!("TS"),
                    utf32str!("WEBM"),
                    utf32str!("WMV"),
                ],
            )
            .add(
                ElementCategory::FileExtension,
                OPTIONS_INVALID,
                [
                    utf32str!("AAC"),
                    utf32str!("AIFF"),
                    utf32str!("FLAC"),
                    utf32str!("M4A"),
                    utf32str!("MP3"),
                    utf32str!("MKA"),
                    utf32str!("OGG"),
                    utf32str!("WAV"),
                    utf32str!("WMA"),
                    utf32str!("7Z"),
                    utf32str!("RAR"),
                    utf32str!("ZIP"),
                    utf32str!("ASS"),
                    utf32str!("SRT"),
                ],
            )
            .add(
                ElementCategory::Language,
                OPTIONS_DEFAULT,
                [
                    utf32str!("ENG"),
                    utf32str!("ENGLISH"),
                    utf32str!("ESPANOL"),
                    utf32str!("JAP"),
                    utf32str!("PT-BR"),
                    utf32str!("SPANISH"),
                    utf32str!("VOSTFR"),
                ],
            )
            .add(
                ElementCategory::Language,
                OPTIONS_UNIDENTIFIABLE,
                [utf32str!("ESP"), utf32str!("ITA")],
            ) // e.g. "Tokyo ESP", "Bokura ga Ita"
            .add(
                ElementCategory::Other,
                OPTIONS_DEFAULT,
                [
                    utf32str!("REMASTER"),
                    utf32str!("REMASTERED"),
                    utf32str!("UNCENSORED"),
                    utf32str!("UNCUT"),
                    utf32str!("TS"),
                    utf32str!("VFR"),
                    utf32str!("WIDESCREEN"),
                    utf32str!("WS"),
                ],
            )
            .add(
                ElementCategory::ReleaseGroup,
                OPTIONS_DEFAULT,
                [utf32str!("THORA")],
            )
            .add(
                ElementCategory::ReleaseInformation,
                OPTIONS_DEFAULT,
                [
                    utf32str!("BATCH"),
                    utf32str!("COMPLETE"),
                    utf32str!("PATCH"),
                    utf32str!("REMUX"),
                ],
            )
            .add(
                ElementCategory::ReleaseInformation,
                OPTIONS_UNIDENTIFIABLE,
                [utf32str!("END"), utf32str!("FINAL")],
            ) // e.g. "The End of Evangelion", "Final Approach"
            .add(
                ElementCategory::ReleaseVersion,
                OPTIONS_DEFAULT,
                [
                    utf32str!("V0"),
                    utf32str!("V1"),
                    utf32str!("V2"),
                    utf32str!("V3"),
                    utf32str!("V4"),
                ],
            )
            .add(
                ElementCategory::Source,
                OPTIONS_DEFAULT,
                [
                    utf32str!("BD"),
                    utf32str!("BDRIP"),
                    utf32str!("BLURAY"),
                    utf32str!("BLU-RAY"),
                    utf32str!("DVD"),
                    utf32str!("DVD5"),
                    utf32str!("DVD9"),
                    utf32str!("DVD-R2J"),
                    utf32str!("DVDRIP"),
                    utf32str!("DVD-RIP"),
                    utf32str!("R2DVD"),
                    utf32str!("R2J"),
                    utf32str!("R2JDVD"),
                    utf32str!("R2JDVDRIP"),
                    utf32str!("HDTV"),
                    utf32str!("HDTVRIP"),
                    utf32str!("TVRIP"),
                    utf32str!("TV-RIP"),
                    utf32str!("WEBCAST"),
                    utf32str!("WEBRIP"),
                ],
            )
            .add(
                ElementCategory::Subtitles,
                OPTIONS_DEFAULT,
                [
                    utf32str!("ASS"),
                    utf32str!("BIG5"),
                    utf32str!("DUB"),
                    utf32str!("DUBBED"),
                    utf32str!("HARDSUB"),
                    utf32str!("HARDSUBS"),
                    utf32str!("RAW"),
                    utf32str!("SOFTSUB"),
                    utf32str!("SOFTSUBS"),
                    utf32str!("SUB"),
                    utf32str!("SUBBED"),
                    utf32str!("SUBTITLED"),
                    utf32str!("MULTISUB"),
                    utf32str!("MULTI SUB"),
                ],
            )
            .add(
                ElementCategory::VideoTerm,
                OPTIONS_DEFAULT,
                [
                    // Frame rate
                    utf32str!("23.976FPS"),
                    utf32str!("24FPS"),
                    utf32str!("29.97FPS"),
                    utf32str!("30FPS"),
                    utf32str!("60FPS"),
                    utf32str!("120FPS"),
                    // Video codec
                    utf32str!("8BIT"),
                    utf32str!("8-BIT"),
                    utf32str!("10BIT"),
                    utf32str!("10BITS"),
                    utf32str!("10-BIT"),
                    utf32str!("10-BITS"),
                    utf32str!("HI10"),
                    utf32str!("HI10P"),
                    utf32str!("HI444"),
                    utf32str!("HI444P"),
                    utf32str!("HI444PP"),
                    utf32str!("HDR"),
                    utf32str!("DV"),
                    utf32str!("DOLBY VISION"),
                    utf32str!("H264"),
                    utf32str!("H265"),
                    utf32str!("H.264"),
                    utf32str!("H.265"),
                    utf32str!("X264"),
                    utf32str!("X265"),
                    utf32str!("X.264"),
                    utf32str!("AVC"),
                    utf32str!("HEVC"),
                    utf32str!("HEVC2"),
                    utf32str!("DIVX"),
                    utf32str!("DIVX5"),
                    utf32str!("DIVX6"),
                    utf32str!("XVID"),
                    utf32str!("AV1"),
                    // Video format
                    utf32str!("AVI"),
                    utf32str!("RMVB"),
                    utf32str!("WMV"),
                    utf32str!("WMV3"),
                    utf32str!("WMV9"),
                    // Video quality
                    utf32str!("HQ"),
                    utf32str!("LQ"),
                    // Video resolution
                    utf32str!("4K"),
                    utf32str!("HD"),
                    utf32str!("SD"),
                ],
            )
            .add(
                ElementCategory::VolumePrefix,
                OPTIONS_DEFAULT,
                [utf32str!("VOL"), utf32str!("VOL."), utf32str!("VOLUME")],
            );
            this
        })
    }

    pub fn add<'a>(
        &'a mut self,
        cat: ElementCategory,
        options: KeywordOptions,
        keywords: impl IntoIterator<Item = &'a Utf32Str>,
    ) -> &mut Self {
        let container = self.get_container_mut(cat);
        for keyword in keywords {
            if keyword.is_empty() || container.contains_key(keyword) {
                continue;
            }

            container.insert(
                keyword.to_owned(),
                Keyword {
                    category: cat,
                    options,
                },
            );
        }
        self
    }

    pub fn find(&self, cat: ElementCategory, keyword: &Utf32Str) -> Option<&Keyword> {
        self.get_container(cat)
            .get(keyword)
            .filter(|x| x.category == cat)
    }

    pub fn find_opt(
        &self,
        cat: ElementCategory,
        keyword: &Utf32Str,
        option: &KeywordOptions,
    ) -> Option<&Keyword> {
        self.get_container(cat).get(keyword)
    }

    pub fn peek(
        &self,
        filename: &Utf32Str,
        range: &TokenRange,
        elements: &mut Elements,
        preidentified_tokens: &mut Vec<TokenRange>,
    ) {
        static ENTRIES: &[(ElementCategory, &[&Utf32Str])] = &[
            (ElementCategory::AudioTerm, &[utf32str!("Dual Audio")]),
            (
                ElementCategory::VideoTerm,
                &[
                    utf32str!("H264"),
                    utf32str!("H.264"),
                    utf32str!("h264"),
                    utf32str!("h.264"),
                ],
            ),
            (
                ElementCategory::VideoResolution,
                &[
                    utf32str!("480p"),
                    utf32str!("720p"),
                    utf32str!("1080p"),
                    utf32str!("2160p"),
                ],
            ),
            (ElementCategory::Source, &[utf32str!("Blu-Ray")]),
        ];

        let filename = &filename.as_slice()[range.as_range()];

        for &(cat, keywords) in ENTRIES {
            for &keyword in keywords {
                let Some(res) = search_slice(filename, keyword.as_slice()) else {
                    continue;
                };
                elements.insert(cat, keyword.to_owned());
                preidentified_tokens.push(res);
            }
        }
    }

    fn get_container(&self, cat: ElementCategory) -> &HashMap<Utf32String, Keyword> {
        if cat == ElementCategory::FileExtension {
            &self.ext
        } else {
            &self.key
        }
    }

    fn get_container_mut(&mut self, cat: ElementCategory) -> &mut HashMap<Utf32String, Keyword> {
        if cat == ElementCategory::FileExtension {
            &mut self.ext
        } else {
            &mut self.key
        }
    }
}
