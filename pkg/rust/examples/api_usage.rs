#[cfg(feature = "sled-storage")]
mod api_usage {
    use {
        futures::executor::block_on,
        gluesql::prelude::{Glue, SledStorage},
    };

    fn mutable_api() {
        let storage = SledStorage::new("data/mutable-api").unwrap();
        let mut glue = Glue::new(storage);

        let sqls = [
            "CREATE TABLE Glue (id INTEGER);",
            "INSERT INTO Glue VALUES (100);",
            "INSERT INTO Glue VALUES (200);",
            "DROP TABLE Glue;",
        ];

        for sql in sqls {
            glue.execute(sql).unwrap();
        }
    }

    async fn async_mutable_api() {
        let storage = SledStorage::new("data/async-mutable-api").unwrap();
        let mut glue = Glue::new(storage);

        let sqls = [
            "CREATE TABLE Glue (id INTEGER);",
            "INSERT INTO Glue VALUES (100);",
            "INSERT INTO Glue VALUES (200);",
            "DROP TABLE Glue;",
        ];

        for sql in sqls {
            glue.execute_async(sql).await.unwrap();
        }
    }

    pub fn run() {
        mutable_api();
        block_on(async_mutable_api());
    }
}

fn main() {
    #[cfg(feature = "sled-storage")]
    api_usage::run();
}
