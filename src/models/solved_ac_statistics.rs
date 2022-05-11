/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2022.02+b1
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolvedAcStatistics {
    /// 여태까지 색인된 백준 문제 수입니다.
    #[serde(rename = "problemCount", skip_serializing_if = "Option::is_none")]
    pub problem_count: Option<i64>,
    /// 여태까지 난이도가 기여된 백준 문제 수입니다.
    #[serde(rename = "problemVotedCount", skip_serializing_if = "Option::is_none")]
    pub problem_voted_count: Option<i64>,
    /// 여태까지 등록한 사용자 수입니다.
    #[serde(rename = "userCount", skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i64>,
    /// 여태까지 난이도에 기여한 사용자 수입니다.
    #[serde(rename = "contributorCount", skip_serializing_if = "Option::is_none")]
    pub contributor_count: Option<i64>,
    /// 여태까지 이루어진 난이도 기여의 수입니다.
    #[serde(rename = "contributionCount", skip_serializing_if = "Option::is_none")]
    pub contribution_count: Option<i64>,
}

impl SolvedAcStatistics {
    pub fn new() -> SolvedAcStatistics {
        SolvedAcStatistics {
            problem_count: None,
            problem_voted_count: None,
            user_count: None,
            contributor_count: None,
            contribution_count: None,
        }
    }
}


