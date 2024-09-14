mod gcdomain;

use std::any::Any;
use std::fs;
use tokio::main;
use gcdomain::Activity;
use tokio_postgres::{NoTls, Error, GenericClient};

#[main]
async fn main() {
    println!("Hello, world!");
    if false {
        let meh = tokio_postgres::connect("host=localhost port=6432 user=gcdata password=CyZfrWBUZ9sZSYRfLm", NoTls).await;
        match meh {
            Ok(tpl) => {
                let client = tpl.0;
                let connection = tpl.1;
                println!("ok  ");
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });


                // Now we can execute a simple statement that just returns its parameter.
                let rowsResult = client
                    .query("SELECT $1::TEXT", &[&"hello world"])
                    .await;
                match rowsResult {
                    Ok(rows) => {
                        let value: &str = rows[0].get(0);
                        assert_eq!(value, "hello world");
                        println!("mufa {}", value);
                    }
                    Err(why) => {
                        println!("connection error: {}", why)
                    }
                }
            }
            Err(why) => {
                println!("connect db failed : {:?}", why.code());
            }
        }
    }

    let path = "./resources/local/gc-1972-01-01-2024-09-05.json";
    let data = fs::read_to_string(path);

    match data {
        Ok(jsonStr) => {
            let res = serde_json::from_str::<Vec<Activity>>(&jsonStr);
            match res {
                Ok(json) => {

                    let prettyJson = serde_json::to_string_pretty(&json).expect("badpretty");
                    let chopped = &prettyJson[..100];
                    println!("amazing json {}", chopped);
                    let string = match &json[17] {
                        Activity::Running { activityId,
                            activityName,
                            startTimeLocal,
                            startTimeGMT,
                            activityType,
                            eventType,
                            distance,
                            duration,
                            elapsedDuration,
                            movingDuration,
                            elevationGain,
                            elevationLoss,
                            averageSpeed,
                            maxSpeed,
                            startLatitude,
                            startLongitude,
                            hasPolyline,
                            hasImages,
                            ownerId,
                            ownerDisplayName,
                            ownerFullName,
                            ownerProfileImageUrlSmall,
                            ownerProfileImageUrlMedium,
                            ownerProfileImageUrlLarge,
                            calories,
                            bmrCalories,
                            averageHR,
                            maxHR,
                            userPro,
                            hasVideo,
                            timeZoneId,
                            beginTimestamp,
                            sportTypeId,
                            aerobicTrainingEffect,
                            anaerobicTrainingEffect,
                            deviceId,
                            minTemperature,
                            maxTemperature,
                            minElevation,
                            maxElevation,
                            maxVerticalSpeed,
                            manufacturer,
                            locationName,
                            lapCount,
                            endLatitude,
                            endLongitude,
                            waterEstimated,
                            trainingEffectLabel,
                            activityTrainingLoad,
                            minActivityLapDuration,
                            aerobicTrainingEffectMessage,
                            anaerobicTrainingEffectMessage,
                            hasSplits,
                            moderateIntensityMinutes,
                            vigorousIntensityMinutes,
                            pr,
                            autoCalcCalories,
                            elevationCorrected,
                            atpActivity,
                            favorite,
                            decoDive,
                            parent,
                            purposeful,
                            manualActivity } =>  ("hiky --> ".to_owned() + &duration.to_string()).to_string(),
                        Activity::Cycling { id, name,distance,duration } => "cycling".to_string(),
                        Activity::Bouldering { id, name,duration } => "bouldering".to_string(),
                        _ => "others".to_string()
                    };
                    println!("amazing types {}", string);
                }
                Err(bad) => {
                    println!("baad json !! {}", bad);
                }
            }
        }
        Err(why) => println!("Open file failed : {:?}", why.kind()),
    }
}
