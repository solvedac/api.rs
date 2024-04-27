/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3.2024.03+b1
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */

/// SiteStatistics : solved.ac 사이트의 통계 정보입니다.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteStatistics {
    /// 여태까지 색인된 백준 문제 수입니다.
    #[serde(rename = "problemCount")]
    pub problem_count: i32,
    /// 여태까지 난이도가 기여된 백준 문제 수입니다.
    #[serde(rename = "problemVotedCount")]
    pub problem_voted_count: i32,
    /// 여태까지 등록한 사용자 수입니다.
    #[serde(rename = "userCount")]
    pub user_count: i32,
    /// 여태까지 난이도에 기여한 사용자 수입니다
    #[serde(rename = "contributorCount")]
    pub contributor_count: i32,
    /// 여태까지 이루어진 난이도 기여의 수입니다.
    #[serde(rename = "contributionCount")]
    pub contribution_count: i32,
}

impl SiteStatistics {
    /// solved.ac 사이트의 통계 정보입니다.
    pub fn new(problem_count: i32, problem_voted_count: i32, user_count: i32, contributor_count: i32, contribution_count: i32) -> SiteStatistics {
        SiteStatistics {
            problem_count,
            problem_voted_count,
            user_count,
            contributor_count,
            contribution_count,
        }
    }
}


