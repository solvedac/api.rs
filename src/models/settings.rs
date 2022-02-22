/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  **주의**: account 하위 루트를 탐색할 경우, 현재 로그인된 solvedacToken이 노출·오용될 수 있으니 주의하십시오.  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2022.02+b1
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// Settings : 사용자의 solved.ac 설정 정보입니다.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    /// 사이트 디자인 테마입니다.
    #[serde(rename = "screenTheme", skip_serializing_if = "Option::is_none")]
    pub screen_theme: Option<ScreenTheme>,
    #[serde(rename = "tagDisplayLanguage", skip_serializing_if = "Option::is_none")]
    pub tag_display_language: Option<Box<crate::models::Language>>,
    #[serde(rename = "iconSchemeSolved", skip_serializing_if = "Option::is_none")]
    pub icon_scheme_solved: Option<Box<crate::models::IconScheme>>,
    #[serde(rename = "iconSchemeNotSolved", skip_serializing_if = "Option::is_none")]
    pub icon_scheme_not_solved: Option<Box<crate::models::IconScheme>>,
    /// 문제 목록의 기본 정렬 순서입니다.
    #[serde(rename = "problemSortBy", skip_serializing_if = "Option::is_none")]
    pub problem_sort_by: Option<ProblemSortBy>,
    /// 트윗에 핸들을 포함하는지 여부입니다.
    #[serde(rename = "twitterPostHandle", skip_serializing_if = "Option::is_none")]
    pub twitter_post_handle: Option<bool>,
    /// CLASS가 올랐을 때 트윗을 보내는지 여부입니다.
    #[serde(rename = "twitterPostOnClassIncrease", skip_serializing_if = "Option::is_none")]
    pub twitter_post_on_class_increase: Option<bool>,
    /// 문제를 처음 해결했을 때 트윗을 보내는지 여부입니다.
    #[serde(rename = "twitterPostOnProblemSolve", skip_serializing_if = "Option::is_none")]
    pub twitter_post_on_problem_solve: Option<bool>,
    /// AC 레이팅이 올랐을 때 트윗을 보내는지 여부입니다.
    #[serde(rename = "twitterPostOnRatingIncrease", skip_serializing_if = "Option::is_none")]
    pub twitter_post_on_rating_increase: Option<bool>,
    /// 티어가 올랐을 때 트윗을 보내는지 여부입니다.
    #[serde(rename = "twitterPostOnTierIncrease", skip_serializing_if = "Option::is_none")]
    pub twitter_post_on_tier_increase: Option<bool>,
}

impl Settings {
    /// 사용자의 solved.ac 설정 정보입니다.
    pub fn new() -> Settings {
        Settings {
            screen_theme: None,
            tag_display_language: None,
            icon_scheme_solved: None,
            icon_scheme_not_solved: None,
            problem_sort_by: None,
            twitter_post_handle: None,
            twitter_post_on_class_increase: None,
            twitter_post_on_problem_solve: None,
            twitter_post_on_rating_increase: None,
            twitter_post_on_tier_increase: None,
        }
    }
}

/// 사이트 디자인 테마입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScreenTheme {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "black")]
    Black,
}
/// 문제 목록의 기본 정렬 순서입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProblemSortBy {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "level")]
    Level,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "solved")]
    Solved,
    #[serde(rename = "average_try")]
    AverageTry,
    #[serde(rename = "random")]
    Random,
}

