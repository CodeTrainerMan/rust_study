use core::fmt;
use std::fmt::Formatter;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse,ResponseError};
use diesel::result::Error as DieselError;
use serde_json::json;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct CustomError{
    pub error_status_code:u16,
    pub error_message:String
}

impl CustomError{
    pub fn new(error_status_code:u16,error_message:String)->CustomError{
        CustomError{
             error_status_code,
            error_message,

        }
    }
}

impl fmt::Display for CustomError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}
impl From<DieselError> for CustomError{
    fn from(error: DieselError) -> CustomError {
        match error{
            DieselError::DatabaseError(_,err)=>
                CustomError::new(409,err.message().to_string()),
            DieselError::NotFound =>
                CustomError::new(404,"Don't find any employee".to_string()),

            err=>CustomError::new(500,
                                  format!("unknow diesel error {}",err))
        }


    }
}

impl ResponseError for CustomError{
    fn error_response(&self) -> HttpResponse{
        let status_code = match StatusCode::from_u16(self.error_status_code){
            Ok(status_code)=>status_code,
            Err(_)=>StatusCode::INTERNAL_SERVER_ERROR
        };
        let error_message = match status_code.as_u16()<500{
            ture=>self.error_message.clone(),
            false =>"service error".to_string(),
        };
        HttpResponse::build(status_code).json(json!({"message":error_message}))
    }
}