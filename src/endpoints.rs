// ::相当于引用， 支持多级引用，比如A::B::C
// 另外一种写法是: use actix_web::*;
use actix_web::{get,Responder,HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResponseResult {
    status: String,
}

#[get("/")] // 这里类似于Spring的注解，所以注解的思维是通用的
async fn news() -> impl Responder {
    //format!("news") // 这不是一个函数，是一个宏
    HttpResponse::Ok().json(ResponseResult {
        status: "200".to_string(), //
    })
}

#[get("/health")]
// -> 函数的返回值类型， fn 代表是一个函数, async 代表是一个异步函数
async fn health() -> impl Responder {
	HttpResponse::Ok().json(ResponseResult { //这里要显示指定一个类型
        status: "200".to_string(), // 显示要转为tostring
    })
}
