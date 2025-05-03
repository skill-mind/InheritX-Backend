use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub async fn create_pool() -> deadpool_postgres::Pool {
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.dbname = Some("inheritx_db".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}

pub async fn run_migrations(pool: &deadpool_postgres::Pool) {
    let client = pool.get().await.unwrap();

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS notifications (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            body TEXT NOT NULL,
            is_read BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS user_activities (
            id SERIAL PRIMARY KEY,
            user_id VARCHAR(255) NOT NULL,
            date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            activity_type VARCHAR(50) NOT NULL,
            details TEXT NOT NULL,
            action_type VARCHAR(50) NOT NULL,
            action_link TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        DO $$ BEGIN
            CREATE TYPE claim_status AS ENUM ('pending', 'approved', 'rejected');
        EXCEPTION
            WHEN duplicate_object THEN null;
        END $$;

        CREATE TABLE IF NOT EXISTS claims (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL,
            amount DECIMAL(10,2) NOT NULL,
            status claim_status NOT NULL DEFAULT 'pending',
            description TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS kyc_records (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL,
            full_name VARCHAR(255) NOT NULL,
            date_of_birth VARCHAR(50) NOT NULL,
            id_type VARCHAR(50) NOT NULL,
            id_number VARCHAR(100) NOT NULL,
            address TEXT NOT NULL,
            verification_status VARCHAR(50) NOT NULL DEFAULT 'pending',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ
        );
    ",
        )
        .await
        .unwrap();
}
