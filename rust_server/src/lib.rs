mod structs; //import structs to use within the file

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::http::header::ContentType;
use structs::send_objects::SendObjects;
use structs::{
    coordinate::Coordinate, dummy_friend::DummyFriend, dummy_object::DummyObject,
    numeric_object::NumericObject,
};
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::net::TcpListener;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::OnceCell;

//Health check implemented as the first endpoints to see if my server works correctly.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/**
 * Initializing test data.
 * OnceCell is used to initialize the objects during runtime.
 * Those objects cannot be created as a static lifetime because of how
 * Rust manages memory allocation. Rust initiates static values on a stack (for example defined
 * numbers) but uses heap to when the size of a variable can change during runtime (Strings). 
 * String types cannot be created during the compilation time as the value can be changed and
 * exceed the allocated memory.
 * OnceCell allows to initialize the struct when using String. 
**/

static DUMMY_OBJECT: OnceCell<DummyObject> = OnceCell::const_new();
async fn get_dummy_object() -> &'static DummyObject {
    return DUMMY_OBJECT.get_or_init(|| async {
        DummyObject {
            id: "63dcdee3a473b6e562063a35".to_owned(),
            index: 0,
            guid: "8c514ba0-f517-46ea-9dc3-dfd64c689bcf".to_owned(),
            is_active: false,
            balance: 2166.53,
            picture: "http://placehold.it/32x32".to_owned(),
            age: 29,
            eye_color: "blue".to_owned(),
            name: "JosefaGrant".to_owned(),
            gender: "female".to_owned(),
            company: "ZENTIA".to_owned(),
            email: "josefagrant@zentia.com".to_owned(),
            phone: "+1(805)529-2872".to_owned(),
            address: "758HubbardStreet,Roderfield,Massachusetts,6249".to_owned(),
            about: "InsitvoluptatedolorLoremnostrudquisLoremauteet.Ametcillumnullacommodoullamcoidelitquiadipisicingfugiatreprehenderitconsequatut.Deseruntsitexcepteurutsuntmagnaesse.Culpanisiquiexcepteurinirureveniamculpatempor.Estipsumdeseruntlaboreesse.Proidentnoneasintincididunttemporeiusmodinnulladolore.\r\n".to_owned(),
            registered: "2018-02-17T12:43:54-00:00".to_owned(),
            latitude: -62.176008,
            longitude: 123.106292,
            tags: vec!["minim".to_owned(), "Lorem".to_owned(), "ea".to_owned(), "incididunt".to_owned(), "voluptate".to_owned(), "laboris".to_owned(), "ex".to_owned()],
            friends: vec![
                DummyFriend{ id: 0, name: "KittyBrock" .to_owned()},
                DummyFriend{ id: 1, name: "MirandaMoody" .to_owned()},
                DummyFriend{ id: 2, name: "GeorgetteSherman" .to_owned()}
            ],
            greeting: "Hello,JosefaGrant!Youhave10unreadmessages.".to_owned(),
            favorite_fruit: "apple".to_owned()
        }
    }).await;
}

static NUMERIC_OBJECT: OnceCell<NumericObject> = OnceCell::const_new();
async fn get_numeric_object() -> &'static NumericObject {
    const COORDINATES: [Coordinate; 10] = [
        Coordinate {
            timestamp: 8828480041,
            latitude: -71.228786,
            longitude: -97.911745,
        },
        Coordinate {
            timestamp: 9299061322,
            latitude: 29.996248,
            longitude: -1.727763,
        },
        Coordinate {
            timestamp: 9052787749,
            latitude: -46.622638,
            longitude: 141.685921,
        },
        Coordinate {
            timestamp: 3491739584,
            latitude: -59.283697,
            longitude: 35.835898,
        },
        Coordinate {
            timestamp: 1196235471,
            latitude: -59.368215,
            longitude: -167.022009,
        },
        Coordinate {
            timestamp: 7014816742,
            latitude: -35.010705,
            longitude: -75.416994,
        },
        Coordinate {
            timestamp: 3190810328,
            latitude: 22.557889,
            longitude: -108.35877,
        },
        Coordinate {
            timestamp: 1756812663,
            latitude: -12.775935,
            longitude: 66.956942,
        },
        Coordinate {
            timestamp: 9237021137,
            latitude: 60.138851,
            longitude: -27.953068,
        },
        Coordinate {
            timestamp: 8377575119,
            latitude: 70.411835,
            longitude: 25.353941,
        },
    ];

    return NUMERIC_OBJECT
        .get_or_init(|| async {
            NumericObject {
                _id: "ef42a23d-4a77-49c5-be8a-7ced8479f915".to_owned(),
                index: 0,
                numbr: 36,
                coordinates: COORDINATES,
            }
        })
        .await;
}

static JSON_NUMERIC_OBJ: OnceCell<String> = OnceCell::const_new();
async fn get_global_numeric_object_json() -> &'static String {
    return JSON_NUMERIC_OBJ
        .get_or_init(|| async { serde_json::to_string(get_numeric_object().await).unwrap() })
        .await;
}
static JSON_DUMMY_OBJ: OnceCell<String> = OnceCell::const_new();
async fn get_global_dummy_object_json() -> &'static String {
    return JSON_DUMMY_OBJ
        .get_or_init(|| async { serde_json::to_string(get_dummy_object().await).unwrap() })
        .await;
}
/**
 * object_type is a number extracted from path indicating which object should be returned (This takes 2 two values, 1 for numericObject, 2 for dummyObject)
**/
async fn json_object(path: web::Path<u8>) -> HttpResponse {
    let object_type = path.into_inner();
    if object_type == 1 {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(get_global_numeric_object_json().await.to_string());
    } else if object_type == 2 {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(get_global_dummy_object_json().await.to_string());
    }
    return HttpResponse::BadRequest().body("Wrong objectType, the allowed values are either 1 for NumericObject or 2 for DummyObject");
}
static BSON_DUMMY_OBJECT: OnceCell<Vec<u8>> = OnceCell::const_new();
async fn get_global_dummy_object_bson() -> &'static Vec<u8> {
    return BSON_DUMMY_OBJECT
        .get_or_init(|| async {
            bson::to_raw_document_buf(get_dummy_object().await)
                .unwrap()
                .as_bytes()
                .to_owned()
        })
        .await;
}
static BSON_NUMERIC_OBJECT: OnceCell<Vec<u8>> = OnceCell::const_new();
async fn get_global_numeric_object_bson() -> &'static Vec<u8> {
    return BSON_NUMERIC_OBJECT
        .get_or_init(|| async {
            bson::to_raw_document_buf(get_numeric_object().await)
                .unwrap()
                .as_bytes()
                .to_owned()
        })
        .await;
}
/**
 * object_type is a number extracted from path indicating which object should be returned (This takes 2 two values, 1 for numericObject, 2 for dummyObject)
**/
async fn bson_object(path: web::Path<u8>) -> HttpResponse {
    let object_type = path.into_inner();
    if object_type == 1 {
        return HttpResponse::Ok()
            .content_type(ContentType::octet_stream())
            .body(get_global_numeric_object_bson().await.as_slice());
    } else if object_type == 2 {
        return HttpResponse::Ok()
            .content_type(ContentType::octet_stream())
            .body(get_global_dummy_object_bson().await.as_slice());
    }
    return HttpResponse::BadRequest().body("Wrong objectType, the allowed values are either 1 for NumericObject or 2 for DummyObject");
}
//For literature_review example to showcase how many bytes does a JSON response occupy
async fn json_bytes() -> HttpResponse {
    let json = r#"{ "id":123,"active":false, "name":"Adam", "balance": 123.23, "friendIDs": [324,234,434] }"#;
    let object: Value = serde_json::from_str(json).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&object).unwrap())
}

/**
 * The function is used for measuring the parsing time of a specified object serialized in a specified format
 *  sendObjectsDTO Object containing parameters for the benchmarking
 *  A JSON string with the results of the benchmark
 */
async fn trigger_reqs(request: web::Json<SendObjects>) -> HttpResponse {
    println!("REQUEST: {:?}", request);
    let client: Client = Client::new();
    let mut url = request.provider_url.to_owned();
    let mut parsing_time: u128 = 0;
    //JSON that will contain the results of the benchmark
    let mut response = serde_json::Map::new();
    response.insert(
        "amount".to_string(),
        serde_json::Value::from(request.amount),
    );
    response.insert(
        request.server_type.to_string().clone(),
        serde_json::Value::from(1),
    );
    let total_time = SystemTime::now();
    if request.object_format == "BSON" {
        println!("########### BSON ##########################");
        //Define URLs the server providers and define the objectType the function should request
        if request.object_type == "DummyObject" {
            url.push_str("/BSONObject/2");
        } else if request.object_type == "NumericObject" {
            url.push_str("/BSONObject/1");
        }
        let mut bson_response: HashMap<String, u128> = HashMap::new();
        bson_response.insert("amount".to_string(), u128::from(request.amount));
        for _ in 0..request.amount {
            let response_bytes = client
                .get(&url)
                .send()
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();
            let parsing = Instant::now();
            //Parse the bytes to the the specified object_type
            if request.object_type == "DummyObject" {
                let _obj: DummyObject = bson::from_slice(&response_bytes).unwrap();
            } else if request.object_type == "NumericObject" {
                let _obj: NumericObject = bson::from_slice(&response_bytes).unwrap();
            }
            parsing_time += parsing.elapsed().as_micros();
        }
        response.insert(
            "bson_total_time_ms".to_string(),
            serde_json::to_value(total_time.elapsed().unwrap().as_millis())
                .unwrap_or(Value::from(-1)),
        );
        response.insert(
            "bson_parsing_time_micro_s".to_string(),
            serde_json::to_value(parsing_time).unwrap_or(Value::from(-1)),
        );
        println!(
            "It took {} μs to pars {} objects",
            parsing_time, request.amount
        );
        println!(
            "It took {}ms to obtain and pars {} objects",
            total_time.elapsed().unwrap().as_millis(),
            request.amount
        );
        println!("##########################################");
    }
    if request.object_format == "JSON" {
        println!("########### JSON ##########################");
        if request.object_type == "DummyObject" {
            url.push_str("/JSONObject/2");
        } else if request.object_type == "NumericObject" {
            url.push_str("/JSONObject/1");
        }
        for _ in 0..request.amount {
            let response = client.get(&url).send().await.unwrap();
            let parsing = Instant::now();
            //Parse the bytes to the the specified object_type
            if request.object_type == "DummyObject" {
                let _obj: DummyObject = response.json::<DummyObject>().await.unwrap();
            } else if request.object_type == "NumericObject" {
                let _obj: NumericObject = response.json::<NumericObject>().await.unwrap();
            }
            parsing_time += parsing.elapsed().as_micros();
        }
        response.insert(
            "json_total_time_ms".to_string(),
            serde_json::to_value(total_time.elapsed().unwrap().as_millis())
                .unwrap_or(Value::from(-1)),
        );
        response.insert(
            "json_parsing_time_micro_s".to_string(),
            serde_json::to_value(parsing_time).unwrap_or(Value::from(-1)),
        );
        println!(
            "It took {} μs to parse {} objects",
            parsing_time, request.amount
        );
        println!(
            "It took {}ms to obtain and pars {} objects",
            total_time.elapsed().unwrap().as_millis(),
            request.amount
        );
        println!("##########################################")
    }
    return HttpResponse::Ok().body(serde_json::to_string(&response).unwrap());
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let port = listener.local_addr().unwrap().port();
    println!("Running server on port {}", port);
    //Endpoint routing
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .route("/health_check", web::get().to(health_check))
            .route("/JSONObject/{object_type}", web::get().to(json_object))
            .route("/BSONObject/{object_type}", web::get().to(bson_object))
            .route("json_bytes", web::get().to(json_bytes))
            .route("triggerReqs", web::post().to(trigger_reqs))
    })
    .keep_alive(Duration::from_secs(60))
    .listen(listener)?
    .run();

    Ok(server)
}
