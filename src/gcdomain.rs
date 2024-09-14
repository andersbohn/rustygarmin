use serde::{de::Visitor, Deserialize, Deserializer,Serialize};


#[derive(Serialize,Deserialize)]
#[serde(untagged)]
pub enum Activity {
    #[serde(rename = "running")]
    Running {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        distance: f64,
        duration: f64,

    },
    #[serde(rename = "hiking")]
    Hiking {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        distance: f64,
        duration: f64,

    },
    #[serde(rename = "cycling")]
    Cycling {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        distance: f64,
        duration: f64,

    },
    #[serde(rename = "bouldering")]
    Bouldering {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        duration: f64,
    },#[serde(rename = "StrengthTraining")]
    Strength {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        duration: f64,
    },
    #[serde(rename = "backcountry_skiing")]
    BackcountrySkiing {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        duration: f64,
    },
    #[serde(rename = "backcountry_skiing_snowboarding_ws")]
    BackcountrySkiingSnowboardingWs {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        duration: f64,
    },
    #[serde(rename = "cross_country_skiing_ws")]
    CrossCountrySkiingWs {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "TreadmillRunning")]
    TreadmillRunning {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "TrailRunning")]
    TrailRunning {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "StrengthTraining")]
    StrengthTraining {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "StandUpPaddleboardingV2")]
    StandUpPaddleboardingV2 {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "StairClimbing")]
    StairClimbing {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "RowingV2")]
    RowingV2 {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "ResortSkiingSnowboardingWs")]
    ResortSkiingSnowboardingWs {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "OpenWaterSwimming")]
    OpenWaterSwimming {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "ObstacleRun")]
    ObstacleRun {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "MultiSport")]
    MultiSport {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "mountaineering")]
    Mountaineering {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "lap_swimming")]
    LapSwimming {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "indoor_running")]
    IndoorRunning {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "indoor_rowing")]
    IndoorRowing {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "indoor_cycling")]
    IndoorCycling {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "indoor_climbing")]
    IndoorClimbing {
        #[serde(rename = "activityName")]
        name: String,
    },
    #[serde(rename = "IndoorCardio")]
    IndoorCardio {
        #[serde(rename = "activityName")]
        name: String,
    },
    Others,
}

