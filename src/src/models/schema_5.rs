/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2021.09+b2
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema5 {
    /// 태그의 ID입니다.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "isMeta", skip_serializing_if = "Option::is_none")]
    pub is_meta: Option<bool>,
    /// 백준에서 사용되는 이 태그의 ID입니다.
    #[serde(rename = "bojTagId", skip_serializing_if = "Option::is_none")]
    pub boj_tag_id: Option<i64>,
    /// 이 태그를 포함하는 문제의 수입니다.
    #[serde(rename = "problemCount", skip_serializing_if = "Option::is_none")]
    pub problem_count: Option<i64>,
    /// 언어별 태그의 이름 목록입니다.
    #[serde(rename = "displayNames", skip_serializing_if = "Option::is_none")]
    pub display_names: Option<Vec<crate::models::ProblemTagDisplayNames>>,
    /// 태그의 별칭입니다.
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<crate::models::ProblemTagAliases>>,
}

impl Schema5 {
    pub fn new() -> Schema5 {
        Schema5 {
            key: None,
            is_meta: None,
            boj_tag_id: None,
            problem_count: None,
            display_names: None,
            aliases: None,
        }
    }
}


