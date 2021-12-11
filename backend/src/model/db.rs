use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;
