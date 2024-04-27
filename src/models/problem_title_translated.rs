/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3.2024.03+b1
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */

/// ProblemTitleTranslated : 언어별 문제 제목입니다.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemTitleTranslated {
    /// 문제 제목이 작성된 언어입니다.
    #[serde(rename = "language")]
    pub language: Box<crate::models::Language>,
    /// 문제 제목이 작성된 언어의 이름입니다.
    #[serde(rename = "languageDisplayName")]
    pub language_display_name: String,
    /// 문제 제목입니다.
    #[serde(rename = "title")]
    pub title: String,
    /// 원본과 동일한지 여부입니다.
    #[serde(rename = "isOriginal")]
    pub is_original: bool,
}

impl ProblemTitleTranslated {
    /// 언어별 문제 제목입니다.
    pub fn new(language: crate::models::Language, language_display_name: String, title: String, is_original: bool) -> ProblemTitleTranslated {
        ProblemTitleTranslated {
            language: Box::new(language),
            language_display_name,
            title,
            is_original,
        }
    }
}


