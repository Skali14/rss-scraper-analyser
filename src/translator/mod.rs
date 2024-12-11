use reqwest;
use serde_json::json;
use config_reader;
use serde::{Deserialize, Serialize};

pub fn translate(texts: Vec<String>, from_lang: FromLanguage, to_lang: ToLanguage) -> Vec<String> {
    /*let item = json!({
        "text": texts,
        "source_lang": from_lang.to_string(),
        "target_lang": to_lang.to_string()
    });*/

    let item = json!({
        "text": vec!["Hallo Welt!"],
        "source_lang": from_lang.to_string(),
        "target_lang": to_lang.to_string()
    });

    let client = reqwest::blocking::Client::new();

    let body = client.post("https://api-free.deepl.com/v2/translate")
        .header("Authorization", format!("DeepL-Auth-Key {}", config_reader::read_value("DEEPLAPIKEY")))
        .header("Host", "api-free.deepl.com")
        .header("User-Agent", "testApp/1.2.3")
        .header("Content-Type", "application/json")
        .body(item.to_string());

    let response: Response = serde_json::from_str(body.send().unwrap().text().unwrap().as_str()).unwrap();

    response.translations.iter().map(|x| {x.text.to_owned()}).collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    translations: Vec<Translation>
}

#[derive(Serialize, Deserialize, Debug)]
struct Translation {
    text: String
}

pub enum FromLanguage {
    BG, CS, DA, DE, EL, EN, ES, ET, FI, FR, HU, ID, IT, JA, KO, LT, LV, NB, NL, PL, PT, RO, RU, SK, SL, SV, TR, UK, ZH
}

pub enum ToLanguage {
    AR, BG, CS, DA, DE, EL, ENGB, ENUS, ES, ET, FI, FR, HU, ID, IT, JA, KO, LT, LV, NB, NL, PL, PTBR, PTPT, RO, RU, SK, SL, SV, TR, UK, ZH, ZHHANS, ZHHANT
}

impl ToString for FromLanguage {
    fn to_string(&self) -> String {
        match self {
            FromLanguage::BG => String::from("BG"),
            FromLanguage::CS => String::from("CS"),
            FromLanguage::DA => String::from("DA"),
            FromLanguage::DE => String::from("DE"),
            FromLanguage::EL => String::from("EL"),
            FromLanguage::EN => String::from("EN"),
            FromLanguage::ES => String::from("ES"),
            FromLanguage::ET => String::from("ET"),
            FromLanguage::FI => String::from("FI"),
            FromLanguage::FR => String::from("FR"),
            FromLanguage::HU => String::from("HU"),
            FromLanguage::ID => String::from("ID"),
            FromLanguage::IT => String::from("IT"),
            FromLanguage::JA => String::from("JA"),
            FromLanguage::KO => String::from("KO"),
            FromLanguage::LT => String::from("LT"),
            FromLanguage::LV => String::from("LV"),
            FromLanguage::NB => String::from("NB"),
            FromLanguage::NL => String::from("NL"),
            FromLanguage::PL => String::from("PL"),
            FromLanguage::PT => String::from("PT"),
            FromLanguage::RO => String::from("RO"),
            FromLanguage::RU => String::from("RU"),
            FromLanguage::SK => String::from("SK"),
            FromLanguage::SL => String::from("SL"),
            FromLanguage::SV => String::from("SV"),
            FromLanguage::TR => String::from("TR"),
            FromLanguage::UK => String::from("UK"),
            FromLanguage::ZH => String::from("ZH")
        }
    }
}

impl ToString for ToLanguage {
    fn to_string(&self) -> String {
        match self {
            ToLanguage::AR => String::from("AR"),
            ToLanguage::BG => String::from("BG"),
            ToLanguage::CS => String::from("CS"),
            ToLanguage::DA => String::from("DA"),
            ToLanguage::DE => String::from("DE"),
            ToLanguage::EL => String::from("EL"),
            ToLanguage::ENGB => String::from("EN-GB"),
            ToLanguage::ENUS => String::from("EN-US"),
            ToLanguage::ES => String::from("ES"),
            ToLanguage::ET => String::from("ET"),
            ToLanguage::FI => String::from("FI"),
            ToLanguage::FR => String::from("FR"),
            ToLanguage::HU => String::from("HU"),
            ToLanguage::ID => String::from("ID"),
            ToLanguage::IT => String::from("IT"),
            ToLanguage::JA => String::from("JA"),
            ToLanguage::KO => String::from("KO"),
            ToLanguage::LT => String::from("LT"),
            ToLanguage::LV => String::from("LV"),
            ToLanguage::NB => String::from("NB"),
            ToLanguage::NL => String::from("NL"),
            ToLanguage::PL => String::from("PL"),
            ToLanguage::PTBR => String::from("PT-BR"),
            ToLanguage::PTPT => String::from("PT-PT"),
            ToLanguage::RO => String::from("RO"),
            ToLanguage::RU => String::from("RU"),
            ToLanguage::SK => String::from("SK"),
            ToLanguage::SL => String::from("SL"),
            ToLanguage::SV => String::from("SV"),
            ToLanguage::TR => String::from("TR"),
            ToLanguage::UK => String::from("UK"),
            ToLanguage::ZH => String::from("ZH"),
            ToLanguage::ZHHANS => String::from("ZH-HANS"),
            ToLanguage::ZHHANT => String::from("ZH-HANT")
        }
    }
}

