use serde::{de::Visitor, Deserialize, Deserializer,Serialize};



#[derive(Serialize,Deserialize)]
// #[serde(tag = "activityType.typeKey")]
#[serde(tag = "atype")]
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
    },#[serde(rename = "strength_training")]
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
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        duration: f64,
    },
    // indoor_cardio
    // indoor_climbing
    // indoor_cycling
    // indoor_rowing
    // indoor_running
    // lap_swimming
    // mountaineering
    // multi_sport
    // obstacle_run
    // open_water_swimming
    // other
    // resort_skiing_snowboarding_ws
    // rowing_v2
    // running
    // stair_climbing
    // stand_up_paddleboarding_v2
    // strength_training
    // trail_running
    // treadmill_running
    Others,
}

/*struct ActivityVisitor;

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
}*/