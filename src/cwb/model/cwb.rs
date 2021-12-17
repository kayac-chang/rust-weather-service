pub mod weather_data {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum ParameterName {
        #[serde(alias = "CITY")]
        City, // 縣市

        #[serde(alias = "CITY_SN")]
        CityID, // 縣市編號

        #[serde(alias = "TOWN")]
        Town, // 鄉鎮

        #[serde(alias = "TOWN_SN")]
        TownID, // 鄉鎮編號
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum WeatherElementName {
        /// 高度，單位 公尺
        #[serde(alias = "ELEV")]
        Elevation,

        /// 風向，單位 度，一般風向 0 表示無風
        #[serde(alias = "WDIR")]
        WindDirection,

        /// 風速，單位 公尺/秒
        #[serde(alias = "WDSD")]
        WindSpeed,

        /// 溫度，單位 攝氏
        #[serde(alias = "TEMP")]
        Temperature,

        /// 相對濕度，單位 百分比率，此處以實數 0-1.0 記錄
        #[serde(alias = "HUMD")]
        Humidity,

        /// 測站氣壓，單位 百帕
        #[serde(alias = "PRES")]
        Pressure,

        /// 日累積雨量，單位 毫米
        #[serde(alias = "H_24R")]
        PrecipitationPerDay,

        /// 小時最大陣風風速，單位 公尺/秒
        #[serde(alias = "H_FX")]
        MaxWindGustPerHourSpeed,

        /// 小時最大陣風風向，單位 度
        #[serde(alias = "H_XD")]
        MaxWindGustPerHourDirection,

        /// 小時最大陣風時間，yyyy-MM-ddThh:mm:ss+08:00
        #[serde(alias = "H_FXT")]
        MaxWindGustPerHourOccurTime,

        /// 本日最高溫，單位 攝氏
        #[serde(alias = "D_TX")]
        MaxTemperaturePerDay,

        /// 本日最高溫發生時間，hhmm (小時分鐘)
        #[serde(alias = "D_TXT")]
        MaxTemperaturePerDayOccurTime,

        /// 本日最低溫，單位 攝氏
        #[serde(alias = "D_TN")]
        MinTemperaturePerDay,

        /// 本日最低溫發生時間，hhmm (小時分鐘)
        #[serde(alias = "D_TNT")]
        MinTemperaturePerDayOccurTime,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WeatherElement {
        #[serde(alias = "elementName")]
        pub name: WeatherElementName,

        #[serde(alias = "elementValue")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Parameter {
        #[serde(alias = "parameterName")]
        pub name: ParameterName,

        #[serde(alias = "parameterValue")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Record {
        pub lat: String, // 座標 緯度
        pub lon: String, // 座標 經度

        #[serde(alias = "locationName")]
        pub name: String, // 測站名稱

        #[serde(alias = "parameter")]
        pub parameters: Vec<Parameter>,

        #[serde(alias = "weatherElement")]
        pub weather_elements: Vec<WeatherElement>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RecordGroup {
        #[serde(alias = "location")]
        pub locations: Vec<Record>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Data {
        pub records: RecordGroup,
    }
}

pub mod forecast {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq)]
    pub enum ForecastType {
        /// 鄉鎮天氣預報-宜蘭縣未來2天天氣預報 - F-D0047-001
        YilanCountyIn2Day,
        /// 鄉鎮天氣預報-宜蘭縣未來1週天氣預報 - F-D0047-003
        YilanCountyInWeek,

        /// 鄉鎮天氣預報-桃園市未來2天天氣預報 - F-D0047-005
        TaoyuanCityIn2Day,
        /// 鄉鎮天氣預報-桃園市未來1週天氣預報 - F-D0047-007
        TaoyuanCityInWeek,

        /// 鄉鎮天氣預報-新竹縣未來2天天氣預報 - F-D0047-009
        HsinchuCountyIn2Day,
        /// 鄉鎮天氣預報-新竹縣未來1週天氣預報 - F-D0047-011
        HsinchuCountyInWeek,

        /// 鄉鎮天氣預報-苗栗縣未來2天天氣預報 - F-D0047-013
        MiaoliCountyIn2Day,
        /// 鄉鎮天氣預報-苗栗縣未來1週天氣預報 - F-D0047-015
        MiaoliCountyInWeek,

        /// 鄉鎮天氣預報-彰化縣未來2天天氣預報 - F-D0047-017
        ChanghuaCountyIn2Day,
        /// 鄉鎮天氣預報-彰化縣未來1週天氣預報 - F-D0047-019
        ChanghuaCountyInWeek,

        /// 鄉鎮天氣預報-南投縣未來2天天氣預報 - F-D0047-021
        NantouCountyIn2Day,
        /// 鄉鎮天氣預報-南投縣未來1週天氣預報 - F-D0047-023
        NantouCountyInWeek,

        /// 鄉鎮天氣預報-雲林縣未來2天天氣預報 - F-D0047-025
        YunlinCountyIn2Day,
        /// 鄉鎮天氣預報-雲林縣未來1週天氣預報 - F-D0047-027
        YunlinCountyInWeek,

        /// 鄉鎮天氣預報-嘉義縣未來2天天氣預報 - F-D0047-029
        ChiayiCountyIn2Day,
        /// 鄉鎮天氣預報-嘉義縣未來1週天氣預報 - F-D0047-031
        ChiayiCountyInWeek,

        /// 鄉鎮天氣預報-屏東縣未來2天天氣預報 - F-D0047-033
        PingtungCountyIn2Day,
        /// 鄉鎮天氣預報-屏東縣未來1週天氣預報 - F-D0047-035
        PingtungCountyInWeek,

        /// 鄉鎮天氣預報-臺東縣未來2天天氣預報 - F-D0047-037
        TaitungCountyIn2Day,
        /// 鄉鎮天氣預報-臺東縣未來1週天氣預報 - F-D0047-039
        TaitungCountyInWeek,

        /// 鄉鎮天氣預報-花蓮縣未來2天天氣預報 - F-D0047-041
        HualienCountyIn2Day,
        /// 鄉鎮天氣預報-花蓮縣未來1週天氣預報 - F-D0047-043
        HualienCountyInWeek,

        /// 鄉鎮天氣預報-澎湖縣未來2天天氣預報 - F-D0047-045
        PenghuCountyIn2Day,
        /// 鄉鎮天氣預報-澎湖縣未來1週天氣預報 - F-D0047-047
        PenghuCountyInWeek,

        /// 鄉鎮天氣預報-基隆縣未來2天天氣預報 - F-D0047-049
        KeelungCountyIn2Day,
        /// 鄉鎮天氣預報-基隆縣未來1週天氣預報 - F-D0047-051
        KeelungCountyInWeek,

        /// 鄉鎮天氣預報-新竹市未來2天天氣預報 - F-D0047-053
        HsinchuCityIn2Day,
        /// 鄉鎮天氣預報-新竹市未來1週天氣預報 - F-D0047-055
        HsinchuCityInWeek,

        /// 鄉鎮天氣預報-嘉義市未來2天天氣預報 - F-D0047-057
        ChiayiCityIn2Day,
        /// 鄉鎮天氣預報-嘉義市未來1週天氣預報 - F-D0047-059
        ChiayiCityInWeek,

        /// 鄉鎮天氣預報-臺北市未來2天天氣預報 - F-D0047-061
        TaipeiCityIn2Day,
        /// 鄉鎮天氣預報-臺北市未來1週天氣預報 - F-D0047-063
        TaipeiCityInWeek,

        /// 鄉鎮天氣預報-高雄市未來2天天氣預報 - F-D0047-065
        KaohsiungCityIn2Day,
        /// 鄉鎮天氣預報-高雄市未來1週天氣預報 - F-D0047-067
        KaohsiungCityInWeek,

        /// 鄉鎮天氣預報-新北市未來2天天氣預報 - F-D0047-069
        NewTaipeiCityIn2Day,
        /// 鄉鎮天氣預報-新北市未來1週天氣預報 - F-D0047-071
        NewTaipeiCityInWeek,

        /// 鄉鎮天氣預報-臺中市未來2天天氣預報 - F-D0047-073
        TaichungCityIn2Day,
        /// 鄉鎮天氣預報-臺中市未來1週天氣預報 - F-D0047-075
        TaichungCityInWeek,

        /// 鄉鎮天氣預報-臺南市未來2天天氣預報 - F-D0047-077
        TainanCityIn2Day,
        /// 鄉鎮天氣預報-臺南市未來1週天氣預報 - F-D0047-079
        TainanCityInWeek,

        /// 鄉鎮天氣預報-連江市未來2天天氣預報 - F-D0047-081
        LienchiangCountyIn2Day,
        /// 鄉鎮天氣預報-連江縣未來1週天氣預報 - F-D0047-083
        LienchiangCountyInWeek,

        /// 鄉鎮天氣預報-金門縣未來2天天氣預報 - F-D0047-085
        KinmenCountyIn2Day,
        /// 鄉鎮天氣預報-金門縣未來1週天氣預報 - F-D0047-087
        KinmenCountyInWeek,

        /// 鄉鎮天氣預報-臺灣未來 2 天天氣預報 - F-D0047-089
        TaiwanIn2Day,
        /// 鄉鎮天氣預報-臺灣未來1週天氣預報 - F-D0047-091
        TaiwanInWeek,
    }

    #[derive(Debug)]
    pub enum CustomError {
        ParseError,
    }

    impl std::error::Error for CustomError {}

    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                CustomError::ParseError => write!(f, "parse error"),
            }
        }
    }

    impl std::str::FromStr for ForecastType {
        type Err = CustomError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "F-D0047-001" => Ok(ForecastType::YilanCountyIn2Day),
                "F-D0047-003" => Ok(ForecastType::YilanCountyInWeek),
                "F-D0047-005" => Ok(ForecastType::TaoyuanCityIn2Day),
                "F-D0047-007" => Ok(ForecastType::TaoyuanCityInWeek),
                "F-D0047-009" => Ok(ForecastType::HsinchuCountyIn2Day),
                "F-D0047-011" => Ok(ForecastType::HsinchuCountyInWeek),
                "F-D0047-013" => Ok(ForecastType::MiaoliCountyIn2Day),
                "F-D0047-015" => Ok(ForecastType::MiaoliCountyInWeek),
                "F-D0047-017" => Ok(ForecastType::ChanghuaCountyIn2Day),
                "F-D0047-019" => Ok(ForecastType::ChanghuaCountyInWeek),
                "F-D0047-021" => Ok(ForecastType::NantouCountyIn2Day),
                "F-D0047-023" => Ok(ForecastType::NantouCountyInWeek),
                "F-D0047-025" => Ok(ForecastType::YunlinCountyIn2Day),
                "F-D0047-027" => Ok(ForecastType::YunlinCountyInWeek),
                "F-D0047-029" => Ok(ForecastType::ChiayiCountyIn2Day),
                "F-D0047-031" => Ok(ForecastType::ChiayiCountyInWeek),
                "F-D0047-033" => Ok(ForecastType::PingtungCountyIn2Day),
                "F-D0047-035" => Ok(ForecastType::PingtungCountyInWeek),
                "F-D0047-037" => Ok(ForecastType::TaitungCountyIn2Day),
                "F-D0047-039" => Ok(ForecastType::TaitungCountyInWeek),
                "F-D0047-041" => Ok(ForecastType::HualienCountyIn2Day),
                "F-D0047-043" => Ok(ForecastType::HualienCountyInWeek),
                "F-D0047-045" => Ok(ForecastType::PenghuCountyIn2Day),
                "F-D0047-047" => Ok(ForecastType::PenghuCountyInWeek),
                "F-D0047-049" => Ok(ForecastType::KeelungCountyIn2Day),
                "F-D0047-051" => Ok(ForecastType::KeelungCountyInWeek),
                "F-D0047-053" => Ok(ForecastType::HsinchuCityIn2Day),
                "F-D0047-055" => Ok(ForecastType::HsinchuCityInWeek),
                "F-D0047-057" => Ok(ForecastType::ChiayiCityIn2Day),
                "F-D0047-059" => Ok(ForecastType::ChiayiCityInWeek),
                "F-D0047-061" => Ok(ForecastType::TaipeiCityIn2Day),
                "F-D0047-063" => Ok(ForecastType::TaipeiCityInWeek),
                "F-D0047-065" => Ok(ForecastType::KaohsiungCityIn2Day),
                "F-D0047-067" => Ok(ForecastType::KaohsiungCityInWeek),
                "F-D0047-069" => Ok(ForecastType::NewTaipeiCityIn2Day),
                "F-D0047-071" => Ok(ForecastType::NewTaipeiCityInWeek),
                "F-D0047-073" => Ok(ForecastType::TaichungCityIn2Day),
                "F-D0047-075" => Ok(ForecastType::TaichungCityInWeek),
                "F-D0047-077" => Ok(ForecastType::TainanCityIn2Day),
                "F-D0047-079" => Ok(ForecastType::TainanCityInWeek),
                "F-D0047-081" => Ok(ForecastType::LienchiangCountyIn2Day),
                "F-D0047-083" => Ok(ForecastType::LienchiangCountyInWeek),
                "F-D0047-085" => Ok(ForecastType::KinmenCountyIn2Day),
                "F-D0047-087" => Ok(ForecastType::KinmenCountyInWeek),
                "F-D0047-089" => Ok(ForecastType::TaiwanIn2Day),
                "F-D0047-091" => Ok(ForecastType::TaiwanInWeek),
                _ => Err(CustomError::ParseError),
            }
        }
    }

    impl std::fmt::Display for ForecastType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let msg = match self {
                ForecastType::YilanCountyIn2Day => "F-D0047-001",
                ForecastType::YilanCountyInWeek => "F-D0047-003",
                ForecastType::TaoyuanCityIn2Day => "F-D0047-005",
                ForecastType::TaoyuanCityInWeek => "F-D0047-007",
                ForecastType::HsinchuCountyIn2Day => "F-D0047-009",
                ForecastType::HsinchuCountyInWeek => "F-D0047-011",
                ForecastType::MiaoliCountyIn2Day => "F-D0047-013",
                ForecastType::MiaoliCountyInWeek => "F-D0047-015",
                ForecastType::ChanghuaCountyIn2Day => "F-D0047-017",
                ForecastType::ChanghuaCountyInWeek => "F-D0047-019",
                ForecastType::NantouCountyIn2Day => "F-D0047-021",
                ForecastType::NantouCountyInWeek => "F-D0047-023",
                ForecastType::YunlinCountyIn2Day => "F-D0047-025",
                ForecastType::YunlinCountyInWeek => "F-D0047-027",
                ForecastType::ChiayiCountyIn2Day => "F-D0047-029",
                ForecastType::ChiayiCountyInWeek => "F-D0047-031",
                ForecastType::PingtungCountyIn2Day => "F-D0047-033",
                ForecastType::PingtungCountyInWeek => "F-D0047-035",
                ForecastType::TaitungCountyIn2Day => "F-D0047-037",
                ForecastType::TaitungCountyInWeek => "F-D0047-039",
                ForecastType::HualienCountyIn2Day => "F-D0047-041",
                ForecastType::HualienCountyInWeek => "F-D0047-043",
                ForecastType::PenghuCountyIn2Day => "F-D0047-045",
                ForecastType::PenghuCountyInWeek => "F-D0047-047",
                ForecastType::KeelungCountyIn2Day => "F-D0047-049",
                ForecastType::KeelungCountyInWeek => "F-D0047-051",
                ForecastType::HsinchuCityIn2Day => "F-D0047-053",
                ForecastType::HsinchuCityInWeek => "F-D0047-055",
                ForecastType::ChiayiCityIn2Day => "F-D0047-057",
                ForecastType::ChiayiCityInWeek => "F-D0047-059",
                ForecastType::TaipeiCityIn2Day => "F-D0047-061",
                ForecastType::TaipeiCityInWeek => "F-D0047-063",
                ForecastType::KaohsiungCityIn2Day => "F-D0047-065",
                ForecastType::KaohsiungCityInWeek => "F-D0047-067",
                ForecastType::NewTaipeiCityIn2Day => "F-D0047-069",
                ForecastType::NewTaipeiCityInWeek => "F-D0047-071",
                ForecastType::TaichungCityIn2Day => "F-D0047-073",
                ForecastType::TaichungCityInWeek => "F-D0047-075",
                ForecastType::TainanCityIn2Day => "F-D0047-077",
                ForecastType::TainanCityInWeek => "F-D0047-079",
                ForecastType::LienchiangCountyIn2Day => "F-D0047-081",
                ForecastType::LienchiangCountyInWeek => "F-D0047-083",
                ForecastType::KinmenCountyIn2Day => "F-D0047-085",
                ForecastType::KinmenCountyInWeek => "F-D0047-087",
                ForecastType::TaiwanIn2Day => "F-D0047-089",
                ForecastType::TaiwanInWeek => "F-D0047-091",
            };

            write!(f, "{}", msg)
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum WeatherElementName {
        /// 12小時降雨機率
        #[serde(alias = "PoP12h")]
        ProbabilityOfPrecipitationIn12Hours,

        /// 天氣現象
        #[serde(alias = "Wx")]
        WeatherPhenomenon,

        /// 體感溫度
        #[serde(alias = "AT")]
        ApparentTemperature,

        /// 最高體感溫度
        #[serde(alias = "MaxAT")]
        MaxApparentTemperature,

        /// 最低體感溫度
        #[serde(alias = "MinAT")]
        MinApparentTemperature,

        /// 溫度
        #[serde(alias = "T")]
        Temperature,

        /// 最低溫度
        #[serde(alias = "MinT")]
        MinTemperature,

        /// 最高溫度
        #[serde(alias = "MaxT")]
        MaxTemperature,

        /// 相對濕度
        #[serde(alias = "RH")]
        RelativeHumidity,

        /// 舒適度指數
        #[serde(alias = "CI")]
        ComfortIndex,

        /// 最小舒適度指數
        #[serde(alias = "MinCI")]
        MinComfortIndex,

        /// 最大舒適度指數
        #[serde(alias = "MaxCI")]
        MaxComfortIndex,

        /// 天氣預報綜合描述
        #[serde(alias = "WeatherDescription")]
        WeatherDescription,

        /// 6小時降雨機率
        #[serde(alias = "PoP6h")]
        ProbabilityOfPrecipitationIn6Hours,

        /// 風速
        #[serde(alias = "WS")]
        EstimateWindSpeed,

        /// 風向
        #[serde(alias = "WD")]
        EstimateWindDirection,

        /// 露點溫度
        #[serde(alias = "Td")]
        DewPointTemperature,

        /// 紫外線指數
        #[serde(alias = "UVI")]
        UVI,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WeatherElementValue {
        pub value: String,
        pub measures: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Time {
        #[serde(alias = "elementValue")]
        pub value: Vec<WeatherElementValue>,

        #[serde(alias = "endTime")]
        pub end_time: String,

        #[serde(alias = "startTime")]
        pub start_time: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WeatherElement {
        #[serde(alias = "elementName")]
        pub name: WeatherElementName,

        #[serde(alias = "time")]
        pub time: Vec<Time>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Location {
        #[serde(alias = "locationName")]
        pub name: String,

        #[serde(alias = "weatherElement")]
        pub weather_elements: Vec<WeatherElement>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Wrapper {
        #[serde(alias = "location")]
        pub location: Vec<Location>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Records {
        #[serde(alias = "locations")]
        pub locations: Vec<Wrapper>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        #[serde(alias = "records")]
        pub records: Records,
    }
}
