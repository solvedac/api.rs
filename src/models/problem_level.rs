/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다.  이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.   <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>   [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)   **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51)) <br> **참고**: 본 저장소를 내려받고, `pnpm dev`를 실행하시면 로컬 개발 서버를 프록시로 삼아 CORS를 무시할 수 있습니다.    ![@solvedac/unofficial-documentation banner](/unofficial-documentation/assets/solvedac-ud-compact.png)
 *
 * The version of the OpenAPI document: 3.2024.03+b1
 * Contact: me@ranolp.dev
 * Generated by: https://openapi-generator.tech
 */

/// ProblemLevel : <solved-icon type=\"solve-tier:0\"></solved-icon> <span class=\"solve-tier zero\">Unrated</span> / <solved-icon type=\"solve-tier:nr\"></solved-icon> <span class=\"solve-tier zero\">Not Ratable</span>를 0, <solved-icon type=\"solve-tier:1\"></solved-icon> <span class=\"solve-tier bronze5\">Bronze V</span>를 1, <solved-icon type=\"solve-tier:2\"></solved-icon> <span class=\"solve-tier bronze4\">Bronze IV</span>를 2, ..., <solved-icon type=\"solve-tier:29\"></solved-icon> <span class=\"solve-tier ruby2\">Ruby II</span>을 29, <solved-icon type=\"solve-tier:30\"></solved-icon> <span class=\"solve-tier ruby1\">Ruby I</span>를 30으로 표현하는 문제 난이도입니다. 자세한 값 정보는 표1. 문제 난이도 표를 펼쳐 참고하십시오.  <details> <summary> 표1. 문제 난이도 표 </summary>  | 수치 | 이름                                                                                                                                                                                             | | ---: | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | |    0 | <solved-icon type=\"solve-tier:0\"></solved-icon> <span class=\"solve-tier zero\">Unrated</span> / <solved-icon type=\"solve-tier:nr\"></solved-icon> <span class=\"solve-tier zero\">Not Ratable</span> | |    1 |                                                                                                 <solved-icon type=\"solve-tier:1\"></solved-icon> <span class=\"solve-tier bronze5\">Bronze V</span> | |    2 |                                                                                                <solved-icon type=\"solve-tier:2\"></solved-icon> <span class=\"solve-tier bronze4\">Bronze IV</span> | |    3 |                                                                                               <solved-icon type=\"solve-tier:3\"></solved-icon> <span class=\"solve-tier bronze3\">Bronze III</span> | |    4 |                                                                                                <solved-icon type=\"solve-tier:4\"></solved-icon> <span class=\"solve-tier bronze2\">Bronze II</span> | |    5 |                                                                                                 <solved-icon type=\"solve-tier:5\"></solved-icon> <span class=\"solve-tier bronze1\">Bronze I</span> | |    6 |                                                                                                 <solved-icon type=\"solve-tier:6\"></solved-icon> <span class=\"solve-tier silver5\">Silver V</span> | |    7 |                                                                                                <solved-icon type=\"solve-tier:7\"></solved-icon> <span class=\"solve-tier silver4\">Silver IV</span> | |    8 |                                                                                               <solved-icon type=\"solve-tier:8\"></solved-icon> <span class=\"solve-tier silver3\">Silver III</span> | |    9 |                                                                                                <solved-icon type=\"solve-tier:9\"></solved-icon> <span class=\"solve-tier silver2\">Silver II</span> | |   10 |                                                                                                <solved-icon type=\"solve-tier:10\"></solved-icon> <span class=\"solve-tier silver1\">Silver I</span> | |   11 |                                                                                                    <solved-icon type=\"solve-tier:11\"></solved-icon> <span class=\"solve-tier gold5\">Gold V</span> | |   12 |                                                                                                   <solved-icon type=\"solve-tier:12\"></solved-icon> <span class=\"solve-tier gold4\">Gold IV</span> | |   13 |                                                                                                  <solved-icon type=\"solve-tier:13\"></solved-icon> <span class=\"solve-tier gold3\">Gold III</span> | |   14 |                                                                                                   <solved-icon type=\"solve-tier:14\"></solved-icon> <span class=\"solve-tier gold2\">Gold II</span> | |   15 |                                                                                                    <solved-icon type=\"solve-tier:15\"></solved-icon> <span class=\"solve-tier gold1\">Gold I</span> | |   16 |                                                                                            <solved-icon type=\"solve-tier:16\"></solved-icon> <span class=\"solve-tier platinum5\">Platinum V</span> | |   17 |                                                                                           <solved-icon type=\"solve-tier:17\"></solved-icon> <span class=\"solve-tier platinum4\">Platinum IV</span> | |   18 |                                                                                          <solved-icon type=\"solve-tier:18\"></solved-icon> <span class=\"solve-tier platinum3\">Platinum III</span> | |   19 |                                                                                           <solved-icon type=\"solve-tier:19\"></solved-icon> <span class=\"solve-tier platinum2\">Platinum II</span> | |   20 |                                                                                            <solved-icon type=\"solve-tier:20\"></solved-icon> <span class=\"solve-tier platinum1\">Platinum I</span> | |   21 |                                                                                              <solved-icon type=\"solve-tier:21\"></solved-icon> <span class=\"solve-tier diamond5\">Diamond V</span> | |   22 |                                                                                             <solved-icon type=\"solve-tier:22\"></solved-icon> <span class=\"solve-tier diamond4\">Diamond IV</span> | |   23 |                                                                                            <solved-icon type=\"solve-tier:23\"></solved-icon> <span class=\"solve-tier diamond3\">Diamond III</span> | |   24 |                                                                                             <solved-icon type=\"solve-tier:24\"></solved-icon> <span class=\"solve-tier diamond2\">Diamond II</span> | |   25 |                                                                                              <solved-icon type=\"solve-tier:25\"></solved-icon> <span class=\"solve-tier diamond1\">Diamond I</span> | |   26 |                                                                                                    <solved-icon type=\"solve-tier:26\"></solved-icon> <span class=\"solve-tier ruby5\">Ruby V</span> | |   27 |                                                                                                   <solved-icon type=\"solve-tier:27\"></solved-icon> <span class=\"solve-tier ruby4\">Ruby IV</span> | |   28 |                                                                                                  <solved-icon type=\"solve-tier:28\"></solved-icon> <span class=\"solve-tier ruby3\">Ruby III</span> | |   29 |                                                                                                   <solved-icon type=\"solve-tier:29\"></solved-icon> <span class=\"solve-tier ruby2\">Ruby II</span> | |   30 |                                                                                                    <solved-icon type=\"solve-tier:30\"></solved-icon> <span class=\"solve-tier ruby1\">Ruby I</span> |  </details>

/// <solved-icon type=\"solve-tier:0\"></solved-icon> <span class=\"solve-tier zero\">Unrated</span> / <solved-icon type=\"solve-tier:nr\"></solved-icon> <span class=\"solve-tier zero\">Not Ratable</span>를 0, <solved-icon type=\"solve-tier:1\"></solved-icon> <span class=\"solve-tier bronze5\">Bronze V</span>를 1, <solved-icon type=\"solve-tier:2\"></solved-icon> <span class=\"solve-tier bronze4\">Bronze IV</span>를 2, ..., <solved-icon type=\"solve-tier:29\"></solved-icon> <span class=\"solve-tier ruby2\">Ruby II</span>을 29, <solved-icon type=\"solve-tier:30\"></solved-icon> <span class=\"solve-tier ruby1\">Ruby I</span>를 30으로 표현하는 문제 난이도입니다. 자세한 값 정보는 표1. 문제 난이도 표를 펼쳐 참고하십시오.  <details> <summary> 표1. 문제 난이도 표 </summary>  | 수치 | 이름                                                                                                                                                                                             | | ---: | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | |    0 | <solved-icon type=\"solve-tier:0\"></solved-icon> <span class=\"solve-tier zero\">Unrated</span> / <solved-icon type=\"solve-tier:nr\"></solved-icon> <span class=\"solve-tier zero\">Not Ratable</span> | |    1 |                                                                                                 <solved-icon type=\"solve-tier:1\"></solved-icon> <span class=\"solve-tier bronze5\">Bronze V</span> | |    2 |                                                                                                <solved-icon type=\"solve-tier:2\"></solved-icon> <span class=\"solve-tier bronze4\">Bronze IV</span> | |    3 |                                                                                               <solved-icon type=\"solve-tier:3\"></solved-icon> <span class=\"solve-tier bronze3\">Bronze III</span> | |    4 |                                                                                                <solved-icon type=\"solve-tier:4\"></solved-icon> <span class=\"solve-tier bronze2\">Bronze II</span> | |    5 |                                                                                                 <solved-icon type=\"solve-tier:5\"></solved-icon> <span class=\"solve-tier bronze1\">Bronze I</span> | |    6 |                                                                                                 <solved-icon type=\"solve-tier:6\"></solved-icon> <span class=\"solve-tier silver5\">Silver V</span> | |    7 |                                                                                                <solved-icon type=\"solve-tier:7\"></solved-icon> <span class=\"solve-tier silver4\">Silver IV</span> | |    8 |                                                                                               <solved-icon type=\"solve-tier:8\"></solved-icon> <span class=\"solve-tier silver3\">Silver III</span> | |    9 |                                                                                                <solved-icon type=\"solve-tier:9\"></solved-icon> <span class=\"solve-tier silver2\">Silver II</span> | |   10 |                                                                                                <solved-icon type=\"solve-tier:10\"></solved-icon> <span class=\"solve-tier silver1\">Silver I</span> | |   11 |                                                                                                    <solved-icon type=\"solve-tier:11\"></solved-icon> <span class=\"solve-tier gold5\">Gold V</span> | |   12 |                                                                                                   <solved-icon type=\"solve-tier:12\"></solved-icon> <span class=\"solve-tier gold4\">Gold IV</span> | |   13 |                                                                                                  <solved-icon type=\"solve-tier:13\"></solved-icon> <span class=\"solve-tier gold3\">Gold III</span> | |   14 |                                                                                                   <solved-icon type=\"solve-tier:14\"></solved-icon> <span class=\"solve-tier gold2\">Gold II</span> | |   15 |                                                                                                    <solved-icon type=\"solve-tier:15\"></solved-icon> <span class=\"solve-tier gold1\">Gold I</span> | |   16 |                                                                                            <solved-icon type=\"solve-tier:16\"></solved-icon> <span class=\"solve-tier platinum5\">Platinum V</span> | |   17 |                                                                                           <solved-icon type=\"solve-tier:17\"></solved-icon> <span class=\"solve-tier platinum4\">Platinum IV</span> | |   18 |                                                                                          <solved-icon type=\"solve-tier:18\"></solved-icon> <span class=\"solve-tier platinum3\">Platinum III</span> | |   19 |                                                                                           <solved-icon type=\"solve-tier:19\"></solved-icon> <span class=\"solve-tier platinum2\">Platinum II</span> | |   20 |                                                                                            <solved-icon type=\"solve-tier:20\"></solved-icon> <span class=\"solve-tier platinum1\">Platinum I</span> | |   21 |                                                                                              <solved-icon type=\"solve-tier:21\"></solved-icon> <span class=\"solve-tier diamond5\">Diamond V</span> | |   22 |                                                                                             <solved-icon type=\"solve-tier:22\"></solved-icon> <span class=\"solve-tier diamond4\">Diamond IV</span> | |   23 |                                                                                            <solved-icon type=\"solve-tier:23\"></solved-icon> <span class=\"solve-tier diamond3\">Diamond III</span> | |   24 |                                                                                             <solved-icon type=\"solve-tier:24\"></solved-icon> <span class=\"solve-tier diamond2\">Diamond II</span> | |   25 |                                                                                              <solved-icon type=\"solve-tier:25\"></solved-icon> <span class=\"solve-tier diamond1\">Diamond I</span> | |   26 |                                                                                                    <solved-icon type=\"solve-tier:26\"></solved-icon> <span class=\"solve-tier ruby5\">Ruby V</span> | |   27 |                                                                                                   <solved-icon type=\"solve-tier:27\"></solved-icon> <span class=\"solve-tier ruby4\">Ruby IV</span> | |   28 |                                                                                                  <solved-icon type=\"solve-tier:28\"></solved-icon> <span class=\"solve-tier ruby3\">Ruby III</span> | |   29 |                                                                                                   <solved-icon type=\"solve-tier:29\"></solved-icon> <span class=\"solve-tier ruby2\">Ruby II</span> | |   30 |                                                                                                    <solved-icon type=\"solve-tier:30\"></solved-icon> <span class=\"solve-tier ruby1\">Ruby I</span> |  </details>
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProblemLevel {
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
    #[serde(rename = "14")]
    _14,
    #[serde(rename = "15")]
    _15,
    #[serde(rename = "16")]
    _16,
    #[serde(rename = "17")]
    _17,
    #[serde(rename = "18")]
    _18,
    #[serde(rename = "19")]
    _19,
    #[serde(rename = "20")]
    _20,
    #[serde(rename = "21")]
    _21,
    #[serde(rename = "22")]
    _22,
    #[serde(rename = "23")]
    _23,
    #[serde(rename = "24")]
    _24,
    #[serde(rename = "25")]
    _25,
    #[serde(rename = "26")]
    _26,
    #[serde(rename = "27")]
    _27,
    #[serde(rename = "28")]
    _28,
    #[serde(rename = "29")]
    _29,
    #[serde(rename = "30")]
    _30,

}

impl ToString for ProblemLevel {
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
            Self::_14 => String::from("14"),
            Self::_15 => String::from("15"),
            Self::_16 => String::from("16"),
            Self::_17 => String::from("17"),
            Self::_18 => String::from("18"),
            Self::_19 => String::from("19"),
            Self::_20 => String::from("20"),
            Self::_21 => String::from("21"),
            Self::_22 => String::from("22"),
            Self::_23 => String::from("23"),
            Self::_24 => String::from("24"),
            Self::_25 => String::from("25"),
            Self::_26 => String::from("26"),
            Self::_27 => String::from("27"),
            Self::_28 => String::from("28"),
            Self::_29 => String::from("29"),
            Self::_30 => String::from("30"),
        }
    }
}




