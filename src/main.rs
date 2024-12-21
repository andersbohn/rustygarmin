mod gcdomain;

use std::any::Any;
use std::fs;
use tokio::main;
use gcdomain::Activity;
use tokio_postgres::{NoTls, Error, GenericClient};
use crate::gcdomain::{MyEnum, AA, AB};
// use crate::::

#[main]
async fn main() {
    println!("Hello, world!");
    if true {
        testmain();
    }
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
    // let path = "./resources/local/running.json";
    let data = fs::read_to_string(path);

    match data {
        Ok(jsonStr) => {
            // let res = serde_json::from_str::<Activity>(&jsonStr);
            let res = serde_json::from_str::<Vec<Activity>>(&jsonStr);
            match res {
                Ok(activities) => {
                    let prettyJson = serde_json::to_string_pretty(&activities).expect("badpretty");
                    let chopped = &prettyJson[..100];
                    println!("amazing json {}", chopped);
                    // println!("nexy - more amazing types {}", string);
                    filterForRunning(activities);
                    ()
                }
                Err(bad) => {
                    println!("baad json !! {}", bad.to_string());
                }
            }
        }
        Err(why) => println!("Open file failed : {:?}", why.kind()),
    }
}

fn isARunning(activity: &Activity) -> Option<&Activity> {
    match &activity {
        Activity::Running {
            activityId,
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
            steps,
            userPro,
            hasVideo,
            timeZoneId,
            beginTimestamp,
            sportTypeId,
            aerobicTrainingEffect,
            anaerobicTrainingEffect,
            avgVerticalOscillation,
            avgGroundContactTime,
            avgStrideLength,
            vO2MaxValue,
            avgVerticalRatio,
            avgGroundContactBalance,
            deviceId,
            minTemperature,
            maxTemperature,
            minElevation,
            maxElevation,
            maxDoubleCadence,
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
            manualActivity
        } => Some(activity), //Result::Ok(activity),
        Activity::Others {
            activityId,
            activityName
        } => None, // Result::Err(("other ".to_string() + activityName))?,
        _ => None //Result::Err("aarg".to_string())?,
    }
}
fn filterForRunning(activities: Vec<&Activity>) -> Vec<&Activity> {
    let rr: Vec<Activity> = activities.iter().filter_map(|a| isARunning(a)).collect();
    rr
}

// fn process_value(value: &MyEnum) -> Option<&MyEnum> {
//     match value {
//         MyEnum::A(a) => Some(a),
//         _ => None,
//     }
// }

pub fn testmain() {
    let values: Vec<MyEnum> = vec![
        MyEnum::A(AA { field1: 10, field2: "Hello".to_string() }),
        MyEnum::B(AB { field3: true }),
        // ... other values
    ];

    for value in values {
        if let MyEnum::B(ab) = value {
            println!("State quarter from {:?}", ab.field3);
        }
        // if let Some(a_value) = process_value(&value) {
        //     println!("A value: {:?}", a_value.field2);
        // } else {
        //     println!("Not an A value");
        // }
    }
}
