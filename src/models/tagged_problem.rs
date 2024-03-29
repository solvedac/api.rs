/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51))  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2022.02+b1
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// TaggedProblem : 문제 정보입니다. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedProblem {
    /// 문제 ID입니다.
    #[serde(rename = "problemId", skip_serializing_if = "Option::is_none")]
    pub problem_id: Option<i64>,
    /// 한국어 문제 제목입니다. HTML 엔티티나 LaTeX 수식을 포함할 수 있습니다.
    #[serde(rename = "titleKo", skip_serializing_if = "Option::is_none")]
    pub title_ko: Option<String>,
    /// 채점 가능 여부입니다.
    #[serde(rename = "isSolvable", skip_serializing_if = "Option::is_none")]
    pub is_solvable: Option<bool>,
    /// 부분 점수 혹은 서브태스크 문제 여부입니다.
    #[serde(rename = "isPartial", skip_serializing_if = "Option::is_none")]
    pub is_partial: Option<bool>,
    /// 맞은 사람 수입니다.
    #[serde(rename = "acceptedUserCount", skip_serializing_if = "Option::is_none")]
    pub accepted_user_count: Option<i64>,
    /// Unrated를 0, Bronze V를 1, ... Ruby II를 29, Ruby I을 30으로 표현하는 문제 레벨입니다. 자세한 값 정보는 표1. 수치 - 이름 표를 펼쳐 참고하십시오.  <details>   <summary>     표1. 수치 - 이름 표   </summary>    | 수치 | 이름         |   | ---: | ------------ |   |    0 | Unrated      |   |    1 | Bronze V     |   |    2 | Bronze IV    |   |    3 | Bronze III   |   |    4 | Bronze II    |   |    5 | Bronze I     |   |    6 | Silver V     |   |    7 | Silver IV    |   |    8 | Silver III   |   |    9 | Silver II    |   |   10 | Silver I     |   |   11 | Gold V       |   |   12 | Gold IV      |   |   13 | Gold III     |   |   14 | Gold II      |   |   15 | Gold I       |   |   16 | Platinum V   |   |   17 | Platinum IV  |   |   18 | Platinum III |   |   19 | Platinum II  |   |   20 | Platinum I   |   |   21 | Diamond V    |   |   22 | Diamond IV   |   |   23 | Diamond III  |   |   24 | Diamond II   |   |   25 | Diamond I    |   |   26 | Ruby V       |   |   27 | Ruby IV      |   |   28 | Ruby III     |   |   29 | Ruby II      |   |   30 | Ruby I       |  </details> 
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    /// 난이도 기여자의 수입니다.
    #[serde(rename = "votedUserCount", skip_serializing_if = "Option::is_none")]
    pub voted_user_count: Option<i64>,
    /// 난이도 기여 제한 여부입니다.
    #[serde(rename = "isLevelLocked", skip_serializing_if = "Option::is_none")]
    pub is_level_locked: Option<bool>,
    /// 평균 시도 횟수입니다.
    #[serde(rename = "averageTries", skip_serializing_if = "Option::is_none")]
    pub average_tries: Option<f32>,
    /// 태그 목록입니다.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ProblemTag>>,
}

impl TaggedProblem {
    /// 문제 정보입니다. 
    pub fn new() -> TaggedProblem {
        TaggedProblem {
            problem_id: None,
            title_ko: None,
            is_solvable: None,
            is_partial: None,
            accepted_user_count: None,
            level: None,
            voted_user_count: None,
            is_level_locked: None,
            average_tries: None,
            tags: None,
        }
    }
}


