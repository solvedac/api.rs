/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3ce78c7
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyCredentialsUserWithSettings {
    /// 설정입니다.
    #[serde(rename = "settings")]
    pub settings: Box<crate::models::VerifyCredentialsUserSettings>,
    /// 사용 중인 이메일 주소입니다.
    #[serde(rename = "email")]
    pub email: String,
    /// 사용자명입니다.
    #[serde(rename = "handle")]
    pub handle: String,
    /// 자기소개입니다.
    #[serde(rename = "bio")]
    pub bio: String,
    /// 장착 중인 뱃지 ID입니다.
    #[serde(rename = "badgeId", skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    /// 장착 중인 배경 ID입니다.
    #[serde(rename = "backgroundId")]
    pub background_id: String,
    /// 프로필 사진으로 가는 하이퍼링크입니다.
    #[serde(rename = "profileImageUrl", skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,
    /// 푼 문제 수입니다.
    #[serde(rename = "solvedCount")]
    pub solved_count: i32,
    /// 난이도 기여 수입니다.
    #[serde(rename = "voteCount")]
    pub vote_count: i32,
    /// 취득한 CLASS입니다. 취득한 CLASS가 없다면 0으로 표현합니다.
    #[serde(rename = "class")]
    pub class: Class,
    /// CLASS 완성도에 따른 장식입니다. <solved-icon type=\"class:1\"></solved-icon> 장식 없음 (`\"none\"`), <solved-icon type=\"class:1s\"></solved-icon> 은장<sup>+</sup> (`\"silver\"`), <solved-icon type=\"class:1g\"></solved-icon> 금장<sup>++</sup> (`\"gold\"`)이 있습니다.
    #[serde(rename = "classDecoration")]
    pub class_decoration: ClassDecoration,
    /// 라이벌 수입니다.
    #[serde(rename = "rivalCount")]
    pub rival_count: i32,
    /// 역라이벌 수입니다.
    #[serde(rename = "reverseRivalCount")]
    pub reverse_rival_count: i32,
    #[serde(rename = "tier")]
    pub tier: Box<crate::models::SolveTier>,
    /// 문제풀이 레이팅입니다
    #[serde(rename = "rating")]
    pub rating: i32,
    /// 상위 100 문제 난이도 합으로 얻은 레이팅입니다.
    #[serde(rename = "ratingByProblemsSum")]
    pub rating_by_problems_sum: i32,
    /// 취득한 CLASS에 따라 얻은 레이팅입니다.
    #[serde(rename = "ratingByClass")]
    pub rating_by_class: i32,
    /// 푼 문제 수로 얻은 레이팅입니다.
    #[serde(rename = "ratingBySolvedCount")]
    pub rating_by_solved_count: i32,
    /// 문제 난이도에 기여한 횟수로 받은 레이팅입니다.
    #[serde(rename = "ratingByVoteCount")]
    pub rating_by_vote_count: i32,
    /// 현재 아레나 티어입니다.
    #[serde(rename = "arenaTier")]
    pub arena_tier: Box<crate::models::ArenaTier>,
    /// 현재 아레나 레이팅입니다.
    #[serde(rename = "arenaRating")]
    pub arena_rating: i32,
    /// 역대 받은 아레나 티어 중 최고점일 때의 아레나 티어입니다.
    #[serde(rename = "arenaMaxTier")]
    pub arena_max_tier: Box<crate::models::ArenaTier>,
    /// 역대 받은 아레나 레이팅 중 최고점일 때의 아레나 레이팅입니다.
    #[serde(rename = "arenaMaxRating")]
    pub arena_max_rating: i32,
    /// 참여한 아레나 라운드 수입니다.
    #[serde(rename = "arenaCompetedRoundCount")]
    pub arena_competed_round_count: i32,
    /// 유지한 최대 스트릭의 길이입니다. (일 단위)
    #[serde(rename = "maxStreak")]
    pub max_streak: i32,
    /// 보유 중인 코인에 100을 곱한 값입니다. 만약, 실제로 보유한 코인이 0.15라면 15로 기록됩니다.
    #[serde(rename = "coins")]
    pub coins: i32,
    /// 보유 중인 별가루 양입니다.
    #[serde(rename = "stardusts")]
    pub stardusts: i32,
    /// 가입 시각입니다. 일부 계정에 대해 2021년 6월 19일 0시 (UTC)로 백필된 흔적이 있습니다.
    #[serde(rename = "joinedAt")]
    pub joined_at: String,
    /// 정지 종료 시각입니다. 정지 이력이 없을 경우 Unix Timestamp 0 값을 포매팅한 문자열입니다.
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    /// [솔브드 서포터](https://solved.ac/support) 종료 시각입니다. 활성화 이력이 없을 경우 Unix Timestamp 0 값을 포매팅한 문자열입니다.
    #[serde(rename = "proUntil")]
    pub pro_until: String,
    /// 순위입니다. 이 값은 요청하는 엔드포인트의 정렬 기준에 따라 다를 수 있습니다. 예) /ranking/_* 엔드포인트에서는 해당 랭킹의 순위, /show 엔드포인트에서는 문제풀이 레이팅 순위
    #[serde(rename = "rank")]
    pub rank: i32,
}

impl VerifyCredentialsUserWithSettings {
    pub fn new(settings: crate::models::VerifyCredentialsUserSettings, email: String, handle: String, bio: String, background_id: String, solved_count: i32, vote_count: i32, class: Class, class_decoration: ClassDecoration, rival_count: i32, reverse_rival_count: i32, tier: crate::models::SolveTier, rating: i32, rating_by_problems_sum: i32, rating_by_class: i32, rating_by_solved_count: i32, rating_by_vote_count: i32, arena_tier: crate::models::ArenaTier, arena_rating: i32, arena_max_tier: crate::models::ArenaTier, arena_max_rating: i32, arena_competed_round_count: i32, max_streak: i32, coins: i32, stardusts: i32, joined_at: String, banned_until: String, pro_until: String, rank: i32) -> VerifyCredentialsUserWithSettings {
        VerifyCredentialsUserWithSettings {
            settings: Box::new(settings),
            email,
            handle,
            bio,
            badge_id: None,
            background_id,
            profile_image_url: None,
            solved_count,
            vote_count,
            class,
            class_decoration,
            rival_count,
            reverse_rival_count,
            tier: Box::new(tier),
            rating,
            rating_by_problems_sum,
            rating_by_class,
            rating_by_solved_count,
            rating_by_vote_count,
            arena_tier: Box::new(arena_tier),
            arena_rating,
            arena_max_tier: Box::new(arena_max_tier),
            arena_max_rating,
            arena_competed_round_count,
            max_streak,
            coins,
            stardusts,
            joined_at,
            banned_until,
            pro_until,
            rank,
        }
    }
}

/// 취득한 CLASS입니다. 취득한 CLASS가 없다면 0으로 표현합니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Class {
    #[serde(rename = "0")]
    _0,
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
/// CLASS 완성도에 따른 장식입니다. <solved-icon type=\"class:1\"></solved-icon> 장식 없음 (`\"none\"`), <solved-icon type=\"class:1s\"></solved-icon> 은장<sup>+</sup> (`\"silver\"`), <solved-icon type=\"class:1g\"></solved-icon> 금장<sup>++</sup> (`\"gold\"`)이 있습니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClassDecoration {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "silver")]
    Silver,
    #[serde(rename = "gold")]
    Gold,
}

