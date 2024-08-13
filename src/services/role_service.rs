use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: String,
    pub created_by: uuid::Uuid,
}

pub struct RoleService {
    pub pool: sqlx::PgPool,
}

impl RoleService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_role(&self, role: CreateRoleRequest) -> Result<(), String> {
        sqlx::query!(
            r#"
            INSERT INTO roles (name, description, created_by)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            role.name,
            role.description,
            role.created_by
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create role: {:?}", e);
            "Failed to create role".to_string()
        })?;

        Ok(())
    }

    pub async fn update_role(&self, role: CreateRoleRequest) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE roles SET name = $1, description = $2 WHERE id = $3
            "#,
            role.name,
            role.description,
            role.created_by
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update role: {:?}", e);
            "Failed to update role".to_string()
        })?;

        Ok(())
    }
}
