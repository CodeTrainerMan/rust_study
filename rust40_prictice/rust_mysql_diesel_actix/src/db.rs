use crate::error_handler::CustomError;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;
use diesel_migrations::embed_migrations;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

embed_migrations!();



lazy_static!{
    static ref POOL:Pool = {
        let db_url = env::var("database_url").expect("database url not set");
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        Pool::new(manager).expect("database connection fail")
    };
}


pub fn init(){
    lazy_static::initialize(&POOL);
    let conn = connection().expect("get databse connect fail");
    embedded_migrations::run(&conn).unwarp();
}
pub fn connection() -> Result<DbConnection,CustomError>{
    POOL.get().map_err(|e|CustomError::new(500,format!("get database connection fail {}",e)))
}