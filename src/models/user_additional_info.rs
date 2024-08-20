/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](/unofficial-documentation/assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3.2024.03+b1
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */

/// UserAdditionalInfo : solved.ac 사용자 부가 정보입니다.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAdditionalInfo {
    /// 사용자의 국가/지역 코드입니다.
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// 사용자의 성별입니다. - 0: 선택 안 함 - 1: 남성 - 2: 여성 - 9: 기타
    #[serde(rename = "gender")]
    pub gender: i32,
    /// 사용자를 영어로 표기할 때 사용하는 대명사입니다.
    #[serde(rename = "pronouns")]
    pub pronouns: String,
    /// 사용자의 생년입니다.
    #[serde(rename = "birthYear")]
    pub birth_year: i32,
    /// 사용자의 생월입니다.
    #[serde(rename = "birthMonth")]
    pub birth_month: i32,
    /// 사용자의 생일입니다.
    #[serde(rename = "birthDay")]
    pub birth_day: i32,
    /// 사용자의 영어 이름입니다.
    #[serde(rename = "name")]
    pub name: String,
    /// 사용자의 모국어 이름입니다.
    #[serde(rename = "nameNative")]
    pub name_native: String,
}

impl UserAdditionalInfo {
    /// solved.ac 사용자 부가 정보입니다.
    pub fn new(country_code: String, gender: i32, pronouns: String, birth_year: i32, birth_month: i32, birth_day: i32, name: String, name_native: String) -> UserAdditionalInfo {
        UserAdditionalInfo {
            country_code,
            gender,
            pronouns,
            birth_year,
            birth_month,
            birth_day,
            name,
            name_native,
        }
    }
}

