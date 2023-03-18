/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51))  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2022.02+b1
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// User : 사용자 정보입니다. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// 사용자명입니다.
    #[serde(rename = "handle")]
    pub handle: String,
    /// 사용자의 자기소개입니다.
    #[serde(rename = "bio")]
    pub bio: String,
    /// 사용자가 지금 사용 중인 뱃지의 아이디입니다.
    #[serde(rename = "badgeId", skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    /// 사용자가 지금 사용 중인 배경의 아이디입니다.
    #[serde(rename = "backgroundId")]
    pub background_id: String,
    /// 사용자의 프로필 사진으로 가는 하이퍼링크입니다.
    #[serde(rename = "profileImageUrl")]
    pub profile_image_url: Option<String>,
    /// 사용자가 푼 문제 수입니다.
    #[serde(rename = "solvedCount")]
    pub solved_count: i64,
    /// 사용자가 난이도 기여를 한 횟수입니다.
    #[serde(rename = "voteCount")]
    pub vote_count: i64,
    /// 사용자가 여태까지 획득한 경험치량입니다.
    #[serde(rename = "exp")]
    pub exp: i64,
    /// Bronze V를 1, Bronze IV를 2, ..., Ruby I을 30, Master를 31로 표현하는 사용자 티어입니다. 자세한 값 정보는 표1. 수치 - 이름 표를 펼쳐 참고하십시오.  <details>   <summary>     표1. 수치 - 이름 표   </summary>    | 수치 | 이름         |   | ---: | ------------ |   |    1 | Bronze V     |   |    2 | Bronze IV    |   |    3 | Bronze III   |   |    4 | Bronze II    |   |    5 | Bronze I     |   |    6 | Silver V     |   |    7 | Silver IV    |   |    8 | Silver III   |   |    9 | Silver II    |   |   10 | Silver I     |   |   11 | Gold V       |   |   12 | Gold IV      |   |   13 | Gold III     |   |   14 | Gold II      |   |   15 | Gold I       |   |   16 | Platinum V   |   |   17 | Platinum IV  |   |   18 | Platinum III |   |   19 | Platinum II  |   |   20 | Platinum I   |   |   21 | Diamond V    |   |   22 | Diamond IV   |   |   23 | Diamond III  |   |   24 | Diamond II   |   |   25 | Diamond I    |   |   26 | Ruby V       |   |   27 | Ruby IV      |   |   28 | Ruby III     |   |   29 | Ruby II      |   |   30 | Ruby I       |   |   31 | Master       |  </details> 
    #[serde(rename = "tier")]
    pub tier: i64,
    /// 사용자의 레이팅입니다.
    #[serde(rename = "rating")]
    pub rating: i64,
    /// 푼 문제의 난이도 합으로 계산한 사용자의 레이팅입니다.
    #[serde(rename = "ratingByProblemsSum")]
    pub rating_by_problems_sum: i64,
    /// 취득한 클래스에 따른 사용자의 레이팅입니다.
    #[serde(rename = "ratingByClass")]
    pub rating_by_class: i64,
    /// 푼 문제 수로 계산한 사용자의 레이팅입니다.
    #[serde(rename = "ratingBySolvedCount")]
    pub rating_by_solved_count: i64,
    /// 문제 난이도에 기여한 횟수로 계산한 사용자의 레이팅입니다.
    #[serde(rename = "ratingByVoteCount")]
    pub rating_by_vote_count: i64,
    /// 사용자가 취득한 Class입니다.
    #[serde(rename = "class")]
    pub class: i64,
    #[serde(rename = "classDecoration")]
    pub class_decoration: crate::models::ClassDecoration,
    /// 사용자의 라이벌 수입니다.
    #[serde(rename = "rivalCount")]
    pub rival_count: i64,
    /// 사용자의 역라이벌 수입니다.
    #[serde(rename = "reverseRivalCount")]
    pub reverse_rival_count: i64,
    /// 최대 연속 문제 풀이일 수입니다.
    #[serde(rename = "maxStreak")]
    pub max_streak: i64,
    /// 사용자가 가지고 있는 코인의 수입니다.
    #[serde(rename = "coins")]
    pub coins: i64,
    /// 사용자가 가지고 있는 별가루의 수입니다.
    #[serde(rename = "stardusts")]
    pub stardusts: i64,
    /// 사용자가 가입한 날짜입니다.
    #[serde(rename = "joinedAt")]
    pub joined_at: String,
    /// 사용자의 정지 종료 날짜입니다.
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    /// 사용자의 PRO 종료 날짜입니다.
    #[serde(rename = "proUntil")]
    pub pro_until: String,
    /// 사용자의 순위입니다.
    #[serde(rename = "rank")]
    pub rank: i64,
}

impl User {
    /// 사용자 정보입니다. 
    pub fn new(handle: String, bio: String, background_id: String, profile_image_url: Option<String>, solved_count: i64, vote_count: i64, exp: i64, tier: i64, rating: i64, rating_by_problems_sum: i64, rating_by_class: i64, rating_by_solved_count: i64, rating_by_vote_count: i64, class: i64, class_decoration: crate::models::ClassDecoration, rival_count: i64, reverse_rival_count: i64, max_streak: i64, coins: i64, stardusts: i64, joined_at: String, banned_until: String, pro_until: String, rank: i64) -> User {
        User {
            handle,
            bio,
            badge_id: None,
            background_id,
            profile_image_url,
            solved_count,
            vote_count,
            exp,
            tier,
            rating,
            rating_by_problems_sum,
            rating_by_class,
            rating_by_solved_count,
            rating_by_vote_count,
            class,
            class_decoration,
            rival_count,
            reverse_rival_count,
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


