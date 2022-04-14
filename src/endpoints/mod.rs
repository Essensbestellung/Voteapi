use actix::prelude::*; // Addr
use actix_redis::{Command, RedisActor};
use actix_web::{error, web, HttpResponse};
use futures_util::future::try_join_all;
use redis_async::{error::Error, resp::FromResp, resp::RespValue, resp_array};
extern crate serde_json;
//use serde_json::Result;

// use redis::{aio::Connection, AsyncCommands, FromRedisValue};

use crate::model;

// pub async fn cast_vote(req_body: String, redis: web::Data<Addr<RedisActor>>) -> impl Responder {
pub async fn cast_vote(
    req_body: web::Json<model::Vote>,
    redis: web::Data<Addr<RedisActor>>,
) -> actix_web::Result<HttpResponse> {
    // Deserialize the Request Body
    let vote = req_body.into_inner();
    let key = vote.name.clone();

    let one = redis.send(Command(resp_array!["SISMEMBER", "voters", &key]));
    log::info!("Checking if {} has already voted", &key);

    let res = one
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;
    let i_res: i64 = FromResp::from_resp_int(res).map_err(error::ErrorInternalServerError)?;
    log::info!("Amounts of {} in VoterList: {}", &key, i_res);
    if i_res == 0 {
        //TODO change Set to sorted Set and reiplement the logic with sorted Set
        //https://clavinjune.dev/en/blogs/create-redis-sets-with-member-expiration/
        let result_sadd = redis.send(Command(resp_array!["SADD", "voters", &key]));
        log::info!("Adding {} to the voters set", &key);

        let resp_sadd = result_sadd
            .await
            .map_err(error::ErrorInternalServerError)?
            .map_err(error::ErrorInternalServerError)?;
        FromResp::from_resp_int(resp_sadd).map_err(error::ErrorInternalServerError)?;
        log::info!("Added {} to the voters set", &key);
    }

    // let result_order = redis.send(Command(resp_array!["SET", &key, &vote.order.to_string()]));
    let s_order = serde_json::to_string(&vote).unwrap();
    let i_expire = 7 * 60 * 60;
    let result_order = redis.send(Command(resp_array![
        "SET",
        &key,
        &s_order,
        "EX",
        i_expire.to_string()
    ]));
    log::info!("Adding {} order for {}", &s_order, &key);

    let resp_order = result_order
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;
    let result: Result<String, _> = FromResp::from_resp(resp_order);

    if result.is_err() {
        log::error!("Could not Set key");
        log::error!("{}", result.err().unwrap());
        return Ok(HttpResponse::InternalServerError().finish());
    }
    log::info!("Added {} order for {}", &s_order, &key);
    Ok(HttpResponse::Ok().body("successfully cached values"))
}

pub async fn get_result(redis: web::Data<Addr<RedisActor>>) -> actix_web::Result<HttpResponse> {
    // Check if result has already been calculated
    let result_get = redis.send(Command(resp_array!["GET", "_result"]));
    // log::info!("Checking if {} has already voted", &key);

    let resp_get = result_get
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    // If Nil was returned Option T is Option<None>
    // Else it is Option<Some> with the corresponding Transformation

    let option_res: Option<String> =
        FromResp::from_resp(resp_get).map_err(error::ErrorInternalServerError)?;

    // we already have a cached result no need to recalculate
    if option_res.is_some() {
        log::info!("Using chached result value");
        return Ok(HttpResponse::Ok().body(option_res.unwrap()));
    }
    log::info!("No cached result found, starting calculation");

    // Grabbing all members of the set voters
    let res_smembers = redis.send(Command(resp_array!["SMEMBERS", "voters"]));
    log::info!("Grabbing all Members of voters; SMEMBERS");
    let resp_smembers = res_smembers
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;
    //SMEMBERS returns an arry of RespValues so we convert it to an vector

    let vec_res: Vec<String> =
        FromResp::from_resp(resp_smembers).map_err(error::ErrorInternalServerError)?;
    log::info!("Following Members found: {:?}", vec_res);

    //Simple Voting result order gets ignored all votes are equal
    let mut vec_future_votes = Vec::new();
    for voter in vec_res {
        vec_future_votes.push(redis.send(Command(resp_array!["GET", voter])))
    }

    let res = try_join_all(vec_future_votes)
        .await
        .map_err(error::ErrorInternalServerError)?
        .into_iter()
        .map(|item| {
            let i: String = FromResp::from_resp(item.map_err(error::ErrorInternalServerError)?)
                .map_err(error::ErrorInternalServerError)?;
            serde_json::from_str(&i).map_err(error::ErrorInternalServerError)
        })
        .collect::<Result<Vec<model::Vote>, _>>()?;

    log::info!("{:?}", res);

    // loop over cast votes
    // if element not in tally add to it else add one to the rooster
    let mut ret = model::Result {
        timestamp: 0,
        elements: vec![],
    };
    for votes in res {
        for vote in votes.order {
            let opt = ret.elements.iter_mut().find(|x| *x == &vote.name);
            match opt {
                Some(_) => {
                    // log::info!("{} already in list tally up", &opt.unwrap().name);
                    let res = opt.unwrap();
                    res.votes += 1;
                    log::info!("{} already in list tally up", res.name);
                }
                None => ret.elements.push(model::ResultElement {
                    votes: 0,
                    name: vote.name.clone(),
                }),
                _ => (),
            }
        }
    }

    Ok(HttpResponse::Ok().body("Need to calculate Result"))
}
