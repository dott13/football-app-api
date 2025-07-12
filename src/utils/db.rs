use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {    
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap()).await?;

    Ok(db)
}