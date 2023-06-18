use sqlx::prelude::*;

#[tokio::main]
async fn main() {
    let conn = sqlx::MySqlPool::connect("mysql://root:my-secret-pw@localhost:43528/test").await;
    if conn.is_err() {
        panic!("connect error: {:#?}", conn.err());
    }
    let conn = conn.unwrap();
    // query columns info from meta data
    let columns = sqlx::query(
            "SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_DEFAULT, COLUMN_COMMENT FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = 'userdd'",
        ).fetch_all(&conn)
        .await
        .unwrap();
    for column in columns {
        let column_name: String = column.get(0);
        let data_type: String = column.get(1);
        let is_nullable: String = column.get(2);
        let column_default: Option<String> = column.get(3);
        let column_comment: Option<String> = column.get(4);
        println!(
            "column_name: {}, data_type: {}, is_nullable: {}, column_default: {:?}, column_comment: {:?}",
            column_name, data_type, is_nullable, column_default, column_comment
        );
    }
}
