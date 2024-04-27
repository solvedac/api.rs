/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3ce78c7
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */

/// CoinShopProduct : 코인샵에서 판매하고 있는 상품입니다.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoinShopProduct {
    /// 재고(Stock Keeping Unit) ID입니다.
    #[serde(rename = "skuId")]
    pub sku_id: i32,
    #[serde(rename = "item")]
    pub item: Box<crate::models::Item>,
    /// 1회 구매 시 획득하는 아이템 개수입니다.
    #[serde(rename = "units")]
    pub units: i32,
    /// 가격입니다. 코인의 경우 나누기 100을 해야 표시 가격이 됩니다.
    #[serde(rename = "price")]
    pub price: i32,
    /// 가격 단위입니다.
    #[serde(rename = "priceUnit")]
    pub price_unit: PriceUnit,
    /// 아이템 사용 시간 제한 여부입니다.
    #[serde(rename = "itemUseTimeLimited")]
    pub item_use_time_limited: bool,
    /// 아이템 구매 기간 제한 여부입니다.
    #[serde(rename = "itemSellTimeLimited")]
    pub item_sell_time_limited: bool,
}

impl CoinShopProduct {
    /// 코인샵에서 판매하고 있는 상품입니다.
    pub fn new(sku_id: i32, item: crate::models::Item, units: i32, price: i32, price_unit: PriceUnit, item_use_time_limited: bool, item_sell_time_limited: bool) -> CoinShopProduct {
        CoinShopProduct {
            sku_id,
            item: Box::new(item),
            units,
            price,
            price_unit,
            item_use_time_limited,
            item_sell_time_limited,
        }
    }
}

/// 가격 단위입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PriceUnit {
    #[serde(rename = "coins")]
    Coins,
    #[serde(rename = "stardusts")]
    Stardusts,
}

