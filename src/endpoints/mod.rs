use actix_web::{ web, HttpResponse, Responder, error};
use actix::prelude::*; // Addr
use actix_redis::{Command, RedisActor};
use redis_async::{resp::RespValue, resp_array, resp::FromResp, error as redisError};
use futures_util::future::try_join_all;
extern crate serde_json;
//use serde_json::Result;

// use redis::{aio::Connection, AsyncCommands, FromRedisValue};

use crate::model;

// pub async fn cast_vote(req_body: String, redis: web::Data<Addr<RedisActor>>) -> impl Responder {
pub async fn cast_vote(req_body: web::Json<model::Vote>, redis: web::Data<Addr<RedisActor>>) ->  actix_web::Result<HttpResponse>{
    // Deserialize the Request Body
    let vote = req_body.into_inner();
    let key = vote.name;

    let one = redis.send(Command(resp_array!["SISMEMBER", "voters", &key]));
    log::info!("Checking if {} has already voted", &key);

    let res = one.await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;
    let iRes: i64 = FromResp::from_resp_int(res).map_err(error::ErrorInternalServerError)?;
    log::info!("Amounts of {} in VoterList: {}", &key, iRes);
    if iRes == 0
    {
        let one = redis.send(Command(resp_array!["SADD", "voters", &key]));
        log::info!("Adding {} to the voters set", &key);

        let res = one.await
            .map_err(error::ErrorInternalServerError)?
            .map_err(error::ErrorInternalServerError)?;
        let iRes: i64 = FromResp::from_resp_int(res).map_err(error::ErrorInternalServerError)?;
        log::info!("Added {} to the voters set", &key);
    }

    // let result_order = redis.send(Command(resp_array!["SET", &key, &vote.order.to_string()]));
    let sOrder = serde_json::to_string(&vote.order).unwrap();
    let expire = 7*60*60;
    let result_order = redis.send(Command(resp_array!["SET", &key, &sOrder, "EX", expire.to_string() ]));
    log::info!("Adding {} order for {}", &sOrder, &key);

    let res = result_order.await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;
    let result : Result<String, _> = FromResp::from_resp(res);

    if result.is_err()
    {
        log::error!("Could not Set key");
        log::error!("{}", result.err().unwrap());
        return Ok(HttpResponse::InternalServerError().finish());
    }
    log::info!("Added {} order for {}", &sOrder, &key);
    Ok(HttpResponse::Ok().body("successfully cached values"))
}

pub async fn get_result(redis: web::Data<Addr<RedisActor>>) -> actix_web::Result<HttpResponse>
{
    // Check if result has already been calculated
    let one = redis.send(Command(resp_array!["GET", "_result"]));
    // log::info!("Checking if {} has already voted", &key);

    let res = one.await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    // Convert Response to Option<T>
    // If Nil was returned Option T is Option<None>
    // Else it is Option<Some> with the corresponding Transformation

    let resOption: Option<String> = FromResp::from_resp(res).map_err(error::ErrorInternalServerError)?;

    // we already have a cached result no need to recalculate
    if resOption.is_some()
    {
        log::info!("Using chached result value");
        return Ok(HttpResponse::Ok().body(resOption.unwrap()));
    }
    log::info!("No cached result found, starting calculation");

    // Grabbing all members of the set voters
    let two = redis.send(Command(resp_array!["SMEMBERS", "voters"]));
    log::info!("Grabbing all Members of voters; SMEMBERS");
    let res = two.await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;
    //SMEMBERS returns an arry of RespValues so we convert it to an vector


    let resOption: Vec<String> = FromResp::from_resp(res).map_err(error::ErrorInternalServerError)?;
    log::info!("Following Members found: {:?}" ,resOption);
    // let resVec: Vec<String> = resOption
    //     .iter()
    //     .map(|resp| FromResp::from_resp(resp).map_err(error::ErrorInternalServerError).unwrap())
    //     .collect();


    // let res = try_join_all([one, two, three])
    // .await
    // .map_err(error::ErrorInternalServerError)?
    // .into_iter()
    // .map(|item| item.map_err(error::ErrorInternalServerError))
    // .collect::<Result<Vec<_>, _>>()?;

    // // successful operations return "OK", so confirm that all returned as so
    // if res
    //     .iter()
    //     .all(|res| matches!(res, RespValue::SimpleString(x) if x == "OK"))


    Ok(HttpResponse::Ok().body("Need to calculate Result"))
}
