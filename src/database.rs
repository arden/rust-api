use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;

//static DB_POOL: Option<MyPool> = None;

static DB_USER: &'static str = "root";
static DB_PASSWORD: &'static str = "123456";
static DB_NAME: &'static str = "tujiao";

pub fn get_db_pool() -> MyPool {
    let db_config = MyOpts {
        user: Some(DB_USER.to_string()),
        pass: Some(DB_PASSWORD.to_string()),
        db_name: Some(DB_NAME.to_string()),
        ..Default::default()
    };
    let db_pool = MyPool::new(db_config).unwrap();
    return db_pool;
}

/*
pub fn getDbPoolSingleton() -> Option<MyPool> {
    let dbConfig = MyOpts {
        user: Some(DB_USER.to_string()),
        pass: Some(DB_PASSWORD.to_string()),
        db_name: Some(DB_NAME.to_string()),
        ..Default::default()
    };

    //if DB_POOL.is_none() {
    if let None = DB_POOL {} {
        //DB_POOL = Some(MyPool::new(dbConfig).unwrap());
    }

    return Some(DB_POOL);
}
*/
