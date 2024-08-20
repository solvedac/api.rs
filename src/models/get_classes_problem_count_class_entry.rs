/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](/unofficial-documentation/assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3.2024.03+b1
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetClassesProblemCountClassEntry {
    /// 클래스 숫자입니다.
    #[serde(rename = "class")]
    pub class: Class,
    /// 총 문제 수입니다.
    #[serde(rename = "total")]
    pub total: i32,
    /// 에센셜 문제 수입니다.
    #[serde(rename = "essentials")]
    pub essentials: i32,
    /// 취득에 필요한 문제 수입니다.
    #[serde(rename = "criteria")]
    pub criteria: i32,
}

impl GetClassesProblemCountClassEntry {
    pub fn new(class: Class, total: i32, essentials: i32, criteria: i32) -> GetClassesProblemCountClassEntry {
        GetClassesProblemCountClassEntry {
            class,
            total,
            essentials,
            criteria,
        }
    }
}

/// 클래스 숫자입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Class {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
    #[serde(rename = "4")]
    _4,
    #[serde(rename = "5")]
    _5,
    #[serde(rename = "6")]
    _6,
    #[serde(rename = "7")]
    _7,
    #[serde(rename = "8")]
    _8,
    #[serde(rename = "9")]
    _9,
    #[serde(rename = "10")]
    _10,
}

