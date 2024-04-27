/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3ce78c7
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationType : 단체 유형입니다.

/// 단체 유형입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrganizationType {
    #[serde(rename = "university")]
    University,
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "community")]
    Community,
    #[serde(rename = "high_school")]
    HighSchool,
    #[serde(rename = "middle_school")]
    MiddleSchool,
    #[serde(rename = "elementary_school")]
    ElementarySchool,
    #[serde(rename = "undefined")]
    Undefined,

}

impl ToString for OrganizationType {
    fn to_string(&self) -> String {
        match self {
            Self::University => String::from("university"),
            Self::Company => String::from("company"),
            Self::Community => String::from("community"),
            Self::HighSchool => String::from("high_school"),
            Self::MiddleSchool => String::from("middle_school"),
            Self::ElementarySchool => String::from("elementary_school"),
            Self::Undefined => String::from("undefined"),
        }
    }
}




