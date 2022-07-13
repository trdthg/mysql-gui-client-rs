pub fn get_databases() -> String {
    format!("SHOW DATABASES;")
}

pub fn get_table_meta(database_name: &str) -> String {
    format!(
        r#"
    SELECT
        TABLE_SCHEMA , -- 库名
        TABLE_NAME , -- 表名
        COLUMN_NAME , -- 列名
        ORDINAL_POSITION , -- 列的排列顺序
        COLUMN_DEFAULT , -- 默认值

        IS_NULLABLE , -- 是否为空
        DATA_TYPE , -- 数据类型
        COLUMN_TYPE , -- 列类型
        CHARACTER_MAXIMUM_LENGTH , -- 字符最大长度
        NUMERIC_PRECISION , -- 数值精度 (最大位数)

        NUMERIC_SCALE , -- 小数精度
        COLUMN_KEY, -- 'KEY'
        EXTRA , -- 额外说明
        COLUMN_COMMENT -- '注释'
    FROM
        information_schema.`COLUMNS`
    WHERE
        TABLE_SCHEMA = '{database_name}'
    ORDER BY
        TABLE_NAME,
        ORDINAL_POSITION;
    "#
    )
}

#[derive(Debug, Clone)]
pub struct TableMeta {
    pub table_schema: String,           // 库名
    pub table_name: String,             // 表名
    pub column_name: String,            // 列名
    pub ordinal_position: u64,          // 列的排列顺序
    pub column_default: Option<String>, // 默认值

    pub is_nullable: String,                   // 是否为空
    pub data_type: String,                     // 数据类型
    pub column_type: String,                   // 列类型
    pub character_maximum_length: Option<u64>, // 字符最大长度
    pub numeric_precision: Option<u64>,        // 数值精度 (最大位数)

    pub numeric_scale: Option<u64>,     // 小数精度
    pub column_key: Option<String>,     // KEY
    pub extra: Option<String>,          // 额外说明
    pub column_comment: Option<String>, //注释
}

impl From<&sqlx::mysql::MySqlRow> for TableMeta {
    fn from(row: &sqlx::mysql::MySqlRow) -> Self {
        use sqlx::Row;
        println!("{:#?}", row);
        TableMeta {
            table_schema: row.get(0),
            table_name: row.get(1),
            column_name: row.get(2),
            ordinal_position: row.get(3),
            column_default: row.get(4),

            is_nullable: row.get(5),
            data_type: row.get(6),
            column_type: row.get(7),
            character_maximum_length: row.get(8),
            numeric_precision: row.get(9),

            numeric_scale: row.get(10),
            column_key: row.get(11),
            extra: row.get(12),
            column_comment: row.get(13),
        }
    }
}

#[cfg(test)]
mod test {
    use super::get_table_meta;

    #[test]
    fn it_should_be_correct() {
        let sql = get_table_meta("aaa");
        println!("{}", sql);
    }
}

pub fn get_100_row(db_name: &str, table_name: &str) -> String {
    format!(
        r#"
        SELECT * FROM {db_name}.{table_name} LIMIT 100
    "#
    )
}
