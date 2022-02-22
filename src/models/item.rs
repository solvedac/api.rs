/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  **주의**: account 하위 루트를 탐색할 경우, 현재 로그인된 solvedacToken이 노출·오용될 수 있으니 주의하십시오.  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2022.02+b1
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// Item : 사용자가 사용할 수 있는 아이템입니다. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// 아이템의 ID입니다.
    #[serde(rename = "itemId")]
    pub item_id: String,
    /// 아이템 사진으로 가는 하이퍼링크입니다.
    #[serde(rename = "itemImageUrl")]
    pub item_image_url: String,
    /// 최대 소유 가능 개수입니다. 호출자에 따라 달라질 수 있습니다.
    #[serde(rename = "inventoryMaxUnits")]
    pub inventory_max_units: i32,
    /// 아이템 사용 가능 여부입니다.
    #[serde(rename = "usable")]
    pub usable: bool,
    /// 아이템의 이름입니다.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 아이템의 설명입니다.
    #[serde(rename = "displayDescription")]
    pub display_description: String,
}

impl Item {
    /// 사용자가 사용할 수 있는 아이템입니다. 
    pub fn new(item_id: String, item_image_url: String, inventory_max_units: i32, usable: bool, display_name: String, display_description: String) -> Item {
        Item {
            item_id,
            item_image_url,
            inventory_max_units,
            usable,
            display_name,
            display_description,
        }
    }
}


