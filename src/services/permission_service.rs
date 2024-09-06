use serde::Serialize;

#[derive(Serialize)]
pub struct PermissionResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub struct PermissionService {
    pub pool: sqlx::PgPool,
}

pub trait PermissionServiceImpl {
    fn new(pool: sqlx::PgPool) -> Self;

    async fn get_permissions(&self) -> Result<Vec<PermissionResponse>, String>;
}

impl PermissionServiceImpl for PermissionService {
    fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    async fn get_permissions(&self) -> Result<Vec<PermissionResponse>, String> {
        let permissions = sqlx::query!(
            r#"
            SELECT id, name, description, created_at
            FROM permissions
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get permissions: {:?}", e);
            "Failed to get permissions".to_string()
        })
        .unwrap();

        let permissions = permissions
            .iter()
            .map(|permission| PermissionResponse {
                id: permission.id,
                name: permission.name.to_owned(),
                description: permission.description.to_owned().unwrap_or_default(),
                created_at: permission.created_at,
            })
            .collect();

        Ok(permissions)
    }
}
