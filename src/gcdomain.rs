use serde::{de::Visitor, Deserialize, Deserializer, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ActivityType {
    typeId: u16,
    typeKey: String,
    parentTypeId: Option<u16>,
    isHidden: bool,
    restricted: bool,
    trimmable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EventType {
    typeId: u16,
    typeKey: String,
    sortOrder: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Activity {
    #[serde(rename = "running")]
    Running {
        activityId: u64,
        activityName: String,
        startTimeLocal: String,
        startTimeGMT: String,
        activityType: ActivityType,
        eventType: EventType,
        distance: f64,
        duration: f64,
        elapsedDuration: f64,
        movingDuration: f64,
        elevationGain: f64,
        elevationLoss: f64,
        averageSpeed: f64,
        maxSpeed: f64,
        startLatitude: f64,
        startLongitude: f64,
        hasPolyline: bool,
        hasImages: bool,
        ownerId: i64,
        ownerDisplayName: String,
        ownerFullName: String,
        ownerProfileImageUrlSmall: String,
        ownerProfileImageUrlMedium: String,
        ownerProfileImageUrlLarge: String,
        calories: f64,
        bmrCalories: f64,
        averageHR: f64,
        maxHR: f64,
        steps: u16,
        userPro: bool,
        hasVideo: bool,
        timeZoneId: i32,
        beginTimestamp: i64,
        sportTypeId: i32,
        aerobicTrainingEffect: f64,
        anaerobicTrainingEffect: f64,
        avgVerticalOscillation: Option<f64>,
        avgGroundContactTime: Option<f64>,
        avgStrideLength: Option<f64>,
        vO2MaxValue: Option<f64>,
        avgVerticalRatio: Option<f64>,
        avgGroundContactBalance: Option<f64>,
        deviceId: i64,
        minTemperature: Option<f64>,
        maxTemperature: Option<f64>,
        minElevation: Option<f64>,
        maxElevation: Option<f64>,
        maxDoubleCadence: Option<f64>,
        maxVerticalSpeed: Option<f64>,
        manufacturer: Option<String>,
        locationName: Option<String>,
        lapCount: i32,
        endLatitude: f64,
        endLongitude: f64,
        waterEstimated: f64,
        trainingEffectLabel: Option<String>,
        activityTrainingLoad: f64,
        minActivityLapDuration: f64,
        aerobicTrainingEffectMessage: Option<String>,
        anaerobicTrainingEffectMessage: Option<String>,
        hasSplits: Option<bool>,
        moderateIntensityMinutes: i32,
        vigorousIntensityMinutes: i32,
        pr: bool,
        autoCalcCalories: bool,
        elevationCorrected: Option<bool>,
        atpActivity: Option<bool>,
        favorite: bool,
        decoDive: Option<bool>,
        parent: bool,
        purposeful: bool,
        manualActivity: bool,
    },
    // #[serde(rename = "running")]
    Others {
        activityId: u64,
        activityName: String,
    },
    // #[serde(rename = "hiking")]
    // Hiking {
    //     #[serde(rename = "activityId")]
    //     id: u64,
    //     #[serde(rename = "activityName")]
    //     name: String,
    //     distance: f64,
    //     duration: f64,
    //
    // },
    // #[serde(rename = "cycling")]
    // Cycling {
    //     #[serde(rename = "activityId")]
    //     id: u64,
    //     #[serde(rename = "activityName")]
    //     name: String,
    //     distance: f64,
    //     duration: f64,
    //
    // },
    // #[serde(rename = "bouldering")]
    // Bouldering {
    //     #[serde(rename = "activityId")]
    //     id: u64,
    //     #[serde(rename = "activityName")]
    //     name: String,
    //     duration: f64,
    // },
    // #[serde(rename = "StrengthTraining")]
    // Strength {
    //     #[serde(rename = "activityId")]
    //     id: u64,
    //     #[serde(rename = "activityName")]
    //     name: String,
    //     duration: f64,
    // },
    // #[serde(rename = "backcountry_skiing")]
    // BackcountrySkiing {
    //     #[serde(rename = "activityId")]
    //     id: u64,
    //     #[serde(rename = "activityName")]
    //     name: String,
    //     duration: f64,
    // },
    // #[serde(rename = "backcountry_skiing_snowboarding_ws")]
    // BackcountrySkiingSnowboardingWs {
    //     #[serde(rename = "activityId")]
    //     id: u64,
    //     #[serde(rename = "activityName")]
    //     name: String,
    //     duration: f64,
    // },
    // #[serde(rename = "cross_country_skiing_ws")]
    // CrossCountrySkiingWs {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "TreadmillRunning")]
    // TreadmillRunning {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "TrailRunning")]
    // TrailRunning {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "StrengthTraining")]
    // StrengthTraining {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "StandUpPaddleboardingV2")]
    // StandUpPaddleboardingV2 {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "StairClimbing")]
    // StairClimbing {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "RowingV2")]
    // RowingV2 {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "ResortSkiingSnowboardingWs")]
    // ResortSkiingSnowboardingWs {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "OpenWaterSwimming")]
    // OpenWaterSwimming {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "ObstacleRun")]
    // ObstacleRun {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "MultiSport")]
    // MultiSport {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "mountaineering")]
    // Mountaineering {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "lap_swimming")]
    // LapSwimming {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "indoor_running")]
    // IndoorRunning {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "indoor_rowing")]
    // IndoorRowing {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "indoor_cycling")]
    // IndoorCycling {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "indoor_climbing")]
    // IndoorClimbing {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
    // #[serde(rename = "IndoorCardio")]
    // IndoorCardio {
    //     #[serde(rename = "activityName")]
    //     name: String,
    // },
}

pub struct AA {
    pub field1: i32,
    pub field2: String,
}
pub struct AB {
    pub field3: bool,
}
pub enum MyEnum {
    A(AA),
    B(AB),
    // ... other variants
}

