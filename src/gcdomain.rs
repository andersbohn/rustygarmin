use serde::{de::Visitor, Deserialize, Deserializer,Serialize};



#[derive(Serialize)]
pub enum Activity {
    #[serde(rename = "hiking")]
    Hiking {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        // startTimeLocal: String,
        // startTimeGMT: String,
        // activityType: ActivityType,
        // eventType: EventType,
        distance: f64,
        duration: f64,
        // elapsedDuration: Option<f64>,
        // movingDuration: Option<f64>,
        // elevationGain: Option<f64>,
        // elevationLoss: Option<f64>,
        // averageSpeed: Option<f64>,
        // maxSpeed: Option<f64>,
        // startLatitude: Option<f64>,
        // startLongitude: Option<f64>,
        // hasPolyline: bool,
        // hasImages: bool,
        // ownerId: i64,
        // ownerDisplayName: String,
        // ownerFullName: String,
        // ownerProfileImageUrlSmall: String,
        // ownerProfileImageUrlMedium: String,
        // ownerProfileImageUrlLarge: String,
        // calories: f64,
        // bmrCalories: Option<f64>,
        // averageHR: Option<f64>,
        // maxHR: Option<f64>,
        // userPro: bool,
        // hasVideo: bool,
        // timeZoneId: i32,
        // beginTimestamp: Option<i64>,
        // sportTypeId: Option<i32>,
        // aerobicTrainingEffect: Option<f64>,
        // anaerobicTrainingEffect: Option<f64>,
        // deviceId: Option<i64>,
        // minTemperature: Option<f64>,
        // maxTemperature: Option<f64>,
        // minElevation: Option<f64>,
        // maxElevation: Option<f64>,
        // maxVerticalSpeed: Option<f64>,
        // manufacturer: Option<String>,
        // locationName: Option<String>,
        // lapCount: Option<i32>,
        // endLatitude: Option<f64>,
        // endLongitude: Option<f64>,
        // waterEstimated: Option<f64>,
        // trainingEffectLabel: Option<String>,
        // activityTrainingLoad: Option<f64>,
        // minActivityLapDuration: Option<f64>,
        // aerobicTrainingEffectMessage: Option<String>,
        // anaerobicTrainingEffectMessage: Option<String>,
        // hasSplits: Option<bool>,
        // moderateIntensityMinutes: Option<i32>,
        // vigorousIntensityMinutes: Option<i32>,
        // pr: bool,
        // autoCalcCalories: bool,
        // elevationCorrected: Option<bool>,
        // atpActivity: Option<bool>,
        // favorite: bool,
        // decoDive: Option<bool>,
        // parent: bool,
        // purposeful: bool,
        // manualActivity: bool
    },
    #[serde(rename = "cycling")]
    Cycling {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        // startTimeLocal: String,
        // startTimeGMT: String,
        // activityType: ActivityType,
        // eventType: EventType,
        distance: f64,
        duration: f64,

    },
    Others,
}

struct ActivityVisitor;

impl<'de> Visitor<'de> for ActivityVisitor {
    type Value = Activity;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Activity type")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        let activity_type = map.next_key::<String>()?.ok_or_else(|| serde::de::Error::custom("Missing activity type"))?;
        let value = map.next_value::<serde_json::Value>()?;

        match activity_type.as_str() {
            "hiking" => Ok(Activity::Hiking {
                id: value["activityId"].as_u64().unwrap(),
                name: value["activityName"].as_str().unwrap().to_string(),
                distance: value["distance"].as_f64().unwrap(),
                duration: value["duration"].as_f64().unwrap(),
            }),
            "cycling" => Ok(Activity::Cycling {
                id: value["activityId"].as_u64().unwrap(),
                name: value["activityName"].as_str().unwrap().to_string(),
                distance: value["distance"].as_f64().unwrap(),
                duration: value["duration"].as_f64().unwrap(),
                // avg_stride_length: value["avgStrideLength"].as_f64().unwrap(),
            }),
            _ => Err(serde::de::Error::custom("Unknown activity type")),
        }
    }
}

impl<'de> Deserialize<'de> for Activity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ActivityVisitor)
    }
}