use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameConfiguration {
    pub general: General,
    pub loading: Loading,
    pub fonts: Fonts,
    pub archive: Archive,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct General {
    pub sEssentialFileCacheList: String,
    pub sUnessentialFileCacheList: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Loading {
    pub sWelcomeScreen1: String,
    pub sWelcomeScreen2: String,
    pub sWelcomeScreen3: String,
    pub sWelcomeScreen4: String,
    pub sMainMenuBackground: String,
    pub sTitleMusic: String,
    pub sInitialSound: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Fonts {
    pub sFontFile_1: String,
    pub sFontFile_2: String,
    pub sFontFile_3: String,
    pub sFontFile_4: String,
    pub sFontFile_5: String,
    pub sFontFile_6: String,
    pub sFontFile_7: String,
    pub sFontFile_8: String,
    pub sFontFile_9: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Archive {
    /// Ordered list of .bsa archives to load
    pub SArchiveList: String,
}

pub fn load_config() -> GameConfiguration {
    // TODO: Properly load configuration from Documents/My Games
    let file: String =
        std::fs::read_to_string("Fallout.ini").expect("Failed to read Fallout.ini configuration");

    serde_ini::from_str(&file).expect("Failed to load config")
}
