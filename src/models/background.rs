/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51))  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2022.02+b1
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// Background : 사용자가 사용할 수 있는 배경입니다. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Background {
    /// 배경의 ID입니다.
    #[serde(rename = "backgroundId")]
    pub background_id: String,
    /// 배경 사진으로 가는 하이퍼링크입니다.
    #[serde(rename = "backgroundImageUrl")]
    pub background_image_url: String,
    /// 배경 사진이 없을 때 대체로 사용할 사진으로 가는 하이퍼링크입니다.
    #[serde(rename = "fallbackBackgroundImageUrl", skip_serializing_if = "Option::is_none")]
    pub fallback_background_image_url: Option<String>,
    /// 배경 비디오로 가는 하이퍼링크입니다.
    #[serde(rename = "backgroundVideoUrl", skip_serializing_if = "Option::is_none")]
    pub background_video_url: Option<String>,
    /// 해당 배경을 획득한 사용자의 수입니다.
    #[serde(rename = "unlockedUserCount")]
    pub unlocked_user_count: i64,
    /// 배경의 이름입니다.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 배경의 설명입니다.
    #[serde(rename = "displayDescription")]
    pub display_description: String,
    /// 해당 배경을 얻을 수 있는 조건입니다.
    #[serde(rename = "conditions")]
    pub conditions: String,
    /// 해당 배경을 얻을 수 있는 조건이 숨겨져 있는지 여부입니다.
    #[serde(rename = "hiddenConditions")]
    pub hidden_conditions: bool,
    /// 해당 배경이 일러스트인지 여부입니다.
    #[serde(rename = "isIllust")]
    pub is_illust: bool,
    /// 해당 배경을 만든 사람들의 정보입니다.
    #[serde(rename = "authors")]
    pub authors: Vec<crate::models::BackgroundAuthors>,
}

impl Background {
    /// 사용자가 사용할 수 있는 배경입니다. 
    pub fn new(background_id: String, background_image_url: String, unlocked_user_count: i64, display_name: String, display_description: String, conditions: String, hidden_conditions: bool, is_illust: bool, authors: Vec<crate::models::BackgroundAuthors>) -> Background {
        Background {
            background_id,
            background_image_url,
            fallback_background_image_url: None,
            background_video_url: None,
            unlocked_user_count,
            display_name,
            display_description,
            conditions,
            hidden_conditions,
            is_illust,
            authors,
        }
    }
}


