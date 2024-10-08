/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](/unofficial-documentation/assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3.2024.03+b1
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */

/// ArenaTier : <solved-icon type=\"arena-tier:0\"></solved-icon> <span class=\"arena-tier unrated\">Unrated</span>를 0, <solved-icon type=\"arena-tier:1\"></solved-icon> <span class=\"arena-tier c\">C</span>를 1, ..., <solved-icon type=\"arena-tier:12\"></solved-icon> <span class=\"arena-tier sss+\">SSS+</span>을 12, <solved-icon type=\"arena-tier:13\"></solved-icon> <span class=\"arena-tier x\">X</span>를 13으로 표현하는 아레나 티어입니다. 자세한 값 정보는 표1. 아레나 레이팅 표를 펼쳐 참고하십시오.  <details> <summary> 표1. 아레나 레이팅 표 </summary>  | 수치 | 이름                                                                                            |                                    요구 레이팅 | | ---: | :---------------------------------------------------------------------------------------------- | ---------------------------------------------: | |    0 | <solved-icon type=\"arena-tier:0\"></solved-icon> <span class=\"arena-tier unrated\">Unrated</span> | <span class=\"bold arena-tier unrated\">-</span> | |    1 |             <solved-icon type=\"arena-tier:1\"></solved-icon> <span class=\"arena-tier c\">C</span> |       <span class=\"bold arena-tier c\">1</span> | |    2 |           <solved-icon type=\"arena-tier:2\"></solved-icon> <span class=\"arena-tier c+\">C+</span> |    <span class=\"bold arena-tier c+\">400</span> | |    3 |             <solved-icon type=\"arena-tier:3\"></solved-icon> <span class=\"arena-tier b\">B</span> |     <span class=\"bold arena-tier b\">800</span> | |    4 |           <solved-icon type=\"arena-tier:4\"></solved-icon> <span class=\"arena-tier b+\">B+</span> |   <span class=\"bold arena-tier b+\">1000</span> | |    5 |             <solved-icon type=\"arena-tier:5\"></solved-icon> <span class=\"arena-tier a\">A</span> |    <span class=\"bold arena-tier a\">1200</span> | |    6 |           <solved-icon type=\"arena-tier:6\"></solved-icon> <span class=\"arena-tier a+\">A+</span> |   <span class=\"bold arena-tier a+\">1400</span> | |    7 |             <solved-icon type=\"arena-tier:7\"></solved-icon> <span class=\"arena-tier s\">S</span> |    <span class=\"bold arena-tier s\">1600</span> | |    8 |           <solved-icon type=\"arena-tier:8\"></solved-icon> <span class=\"arena-tier s+\">S+</span> |   <span class=\"bold arena-tier s+\">1800</span> | |    9 |           <solved-icon type=\"arena-tier:9\"></solved-icon> <span class=\"arena-tier ss\">SS</span> |   <span class=\"bold arena-tier ss\">2000</span> | |   10 |        <solved-icon type=\"arena-tier:10\"></solved-icon> <span class=\"arena-tier ss+\">SS+</span> |  <span class=\"bold arena-tier ss+\">2200</span> | |   11 |        <solved-icon type=\"arena-tier:11\"></solved-icon> <span class=\"arena-tier sss\">SSS</span> |  <span class=\"bold arena-tier sss\">2400</span> | |   12 |      <solved-icon type=\"arena-tier:12\"></solved-icon> <span class=\"arena-tier sss+\">SSS+</span> | <span class=\"bold arena-tier sss+\">2600</span> | |   13 |            <solved-icon type=\"arena-tier:13\"></solved-icon> <span class=\"arena-tier x\">X</span> |    <span class=\"bold arena-tier x\">3000</span> |  </details>

/// <solved-icon type=\"arena-tier:0\"></solved-icon> <span class=\"arena-tier unrated\">Unrated</span>를 0, <solved-icon type=\"arena-tier:1\"></solved-icon> <span class=\"arena-tier c\">C</span>를 1, ..., <solved-icon type=\"arena-tier:12\"></solved-icon> <span class=\"arena-tier sss+\">SSS+</span>을 12, <solved-icon type=\"arena-tier:13\"></solved-icon> <span class=\"arena-tier x\">X</span>를 13으로 표현하는 아레나 티어입니다. 자세한 값 정보는 표1. 아레나 레이팅 표를 펼쳐 참고하십시오.  <details> <summary> 표1. 아레나 레이팅 표 </summary>  | 수치 | 이름                                                                                            |                                    요구 레이팅 | | ---: | :---------------------------------------------------------------------------------------------- | ---------------------------------------------: | |    0 | <solved-icon type=\"arena-tier:0\"></solved-icon> <span class=\"arena-tier unrated\">Unrated</span> | <span class=\"bold arena-tier unrated\">-</span> | |    1 |             <solved-icon type=\"arena-tier:1\"></solved-icon> <span class=\"arena-tier c\">C</span> |       <span class=\"bold arena-tier c\">1</span> | |    2 |           <solved-icon type=\"arena-tier:2\"></solved-icon> <span class=\"arena-tier c+\">C+</span> |    <span class=\"bold arena-tier c+\">400</span> | |    3 |             <solved-icon type=\"arena-tier:3\"></solved-icon> <span class=\"arena-tier b\">B</span> |     <span class=\"bold arena-tier b\">800</span> | |    4 |           <solved-icon type=\"arena-tier:4\"></solved-icon> <span class=\"arena-tier b+\">B+</span> |   <span class=\"bold arena-tier b+\">1000</span> | |    5 |             <solved-icon type=\"arena-tier:5\"></solved-icon> <span class=\"arena-tier a\">A</span> |    <span class=\"bold arena-tier a\">1200</span> | |    6 |           <solved-icon type=\"arena-tier:6\"></solved-icon> <span class=\"arena-tier a+\">A+</span> |   <span class=\"bold arena-tier a+\">1400</span> | |    7 |             <solved-icon type=\"arena-tier:7\"></solved-icon> <span class=\"arena-tier s\">S</span> |    <span class=\"bold arena-tier s\">1600</span> | |    8 |           <solved-icon type=\"arena-tier:8\"></solved-icon> <span class=\"arena-tier s+\">S+</span> |   <span class=\"bold arena-tier s+\">1800</span> | |    9 |           <solved-icon type=\"arena-tier:9\"></solved-icon> <span class=\"arena-tier ss\">SS</span> |   <span class=\"bold arena-tier ss\">2000</span> | |   10 |        <solved-icon type=\"arena-tier:10\"></solved-icon> <span class=\"arena-tier ss+\">SS+</span> |  <span class=\"bold arena-tier ss+\">2200</span> | |   11 |        <solved-icon type=\"arena-tier:11\"></solved-icon> <span class=\"arena-tier sss\">SSS</span> |  <span class=\"bold arena-tier sss\">2400</span> | |   12 |      <solved-icon type=\"arena-tier:12\"></solved-icon> <span class=\"arena-tier sss+\">SSS+</span> | <span class=\"bold arena-tier sss+\">2600</span> | |   13 |            <solved-icon type=\"arena-tier:13\"></solved-icon> <span class=\"arena-tier x\">X</span> |    <span class=\"bold arena-tier x\">3000</span> |  </details>
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ArenaTier {
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
    #[serde(rename = "11")]
    _11,
    #[serde(rename = "12")]
    _12,
    #[serde(rename = "13")]
    _13,

}

impl ToString for ArenaTier {
    fn to_string(&self) -> String {
        match self {
            Self::_0 => String::from("0"),
            Self::_1 => String::from("1"),
            Self::_2 => String::from("2"),
            Self::_3 => String::from("3"),
            Self::_4 => String::from("4"),
            Self::_5 => String::from("5"),
            Self::_6 => String::from("6"),
            Self::_7 => String::from("7"),
            Self::_8 => String::from("8"),
            Self::_9 => String::from("9"),
            Self::_10 => String::from("10"),
            Self::_11 => String::from("11"),
            Self::_12 => String::from("12"),
            Self::_13 => String::from("13"),
        }
    }
}




