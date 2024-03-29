


use actix_web::{web, HttpRequest, HttpResponse, Result};

// POST api/auth/signup
pub async fn signup(user_dto: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/login
pub async fn login(login_dto: web::Json<LoginDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_service::login(login_dto.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGIN_SUCCESS,
            token_res,
        ))),
        Err(err) => Ok(err.reponse()),
    }
}

pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match req.headers().get(constants::AUTHORIZATION) {
        Some(auth_header) => {
            account_service::logout(auth_header, &pool);
            Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_LOGOUT_SUCCESS,
                constants::EMPTY,
            )));
        },
        None => {
            Ok(HttpResponse::BadRequest().json(ResponseBody::new(
                constants::MESSAGE_TOKEN_MISSING,
                constants::EMPTY,
            )))
        }
    }
}