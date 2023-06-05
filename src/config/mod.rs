pub mod config {
    use std::sync::Arc;
    use crate::database::database::Database;
    use tokio::task;

    pub trait DatabaseConfig {
        fn db(&self) -> Arc<Database>;
        fn take_handle1(&mut self) -> Option<task::JoinHandle<()>>;
        fn take_handle2(&mut self) -> Option<task::JoinHandle<()>>;
    }

    pub struct DatabaseConfigImpl {
        db: Arc<Database>,
        handle1: Option<task::JoinHandle<()>>,
        handle2: Option<task::JoinHandle<()>>
    }

    impl DatabaseConfigImpl {
        pub fn new() -> Self {
            let db = Arc::new(Database::new());
            let db_clone1 = db.clone();
            let db_clone2 = db.clone();

            let handle1 = task::spawn(async move {
                db_clone1.insert("key1".to_string(), "value1".to_string()).await;
            });

            let handle2 = task::spawn(async move {
                db_clone2.insert("key2".to_string(), "value2".to_string()).await;
            });

            Self {
                db,
                handle1: Some(handle1),
                handle2: Some(handle2)
            }
        }
    }

    impl DatabaseConfig for DatabaseConfigImpl {
        fn db(&self) -> Arc<Database> {
            self.db.clone()
        }

        fn take_handle1(&mut self) -> Option<task::JoinHandle<()>> {
            self.handle1.take()
        }

        fn take_handle2(&mut self) -> Option<task::JoinHandle<()>> {
            self.handle2.take()
        }
    }
}
