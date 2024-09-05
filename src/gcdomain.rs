use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ActivityType {
    typeId: i32,
    typeKey: String,
    parentTypeId: i32,
    isHidden: bool,
    restricted: bool,
    trimmable: bool
}
#[derive(Serialize, Deserialize)]
pub struct EventType {
    typeId: i32,
    typeKey: String,
    sortOrder: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    activityId: i64,
    activityName: Option<String>,
    startTimeLocal: String,
    startTimeGMT: String,
    activityType: ActivityType,
    eventType: EventType,
    distance: Option<f64>,
    duration: f64,
    elapsedDuration: Option<f64>,
    movingDuration: Option<f64>,
    elevationGain: Option<f64>,
    elevationLoss: Option<f64>,
    averageSpeed: Option<f64>,
    maxSpeed: Option<f64>,
    startLatitude: Option<f64>,
    startLongitude: Option<f64>,
    hasPolyline: bool,
    hasImages: bool,
    ownerId: i64,
    ownerDisplayName: String,
    ownerFullName: String,
    ownerProfileImageUrlSmall: String,
    ownerProfileImageUrlMedium: String,
    ownerProfileImageUrlLarge: String,
    calories: f64,
    bmrCalories: Option<f64>,
    averageHR: Option<f64>,
    maxHR: Option<f64>,
    userPro: bool,
    hasVideo: bool,
    timeZoneId: i32,
    beginTimestamp: Option<i64>,
    sportTypeId: Option<i32>,
    aerobicTrainingEffect: Option<f64>,
    anaerobicTrainingEffect: Option<f64>,
    deviceId: Option<i64>,
    minTemperature: Option<f64>,
    maxTemperature: Option<f64>,
    minElevation: Option<f64>,
    maxElevation: Option<f64>,
    maxVerticalSpeed: Option<f64>,
    manufacturer: Option<String>,
    locationName: Option<String>,
    lapCount: Option<i32>,
    endLatitude: Option<f64>,
    endLongitude: Option<f64>,
    waterEstimated: Option<f64>,
    trainingEffectLabel: Option<String>,
    activityTrainingLoad: Option<f64>,
    minActivityLapDuration: Option<f64>,
    aerobicTrainingEffectMessage: Option<String>,
    anaerobicTrainingEffectMessage: Option<String>,
    hasSplits: Option<bool>,
    moderateIntensityMinutes: Option<i32>,
    vigorousIntensityMinutes: Option<i32>,
    pr: bool,
    autoCalcCalories: bool,
    elevationCorrected: Option<bool>,
    atpActivity: Option<bool>,
    favorite: bool,
    decoDive: Option<bool>,
    parent: bool,
    purposeful: bool,
    manualActivity: bool
}

