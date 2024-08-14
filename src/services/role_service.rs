use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: String,
    pub created_by: uuid::Uuid,
}

#[derive(Serialize)]
pub struct RoleResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_by: uuid::Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
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
        let current_role = sqlx::query!(
            r#"
            SELECT * FROM roles WHERE id = $1
            "#,
            role.created_by
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch role: {:?}", e);
            "Failed to fetch role".to_string()
        })?;

        if current_role.created_by != role.created_by {
            return Err("You are not authorized to update this role".to_string());
        }

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

    pub async fn get_roles_by_user_created(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<RoleResponse>, String> {
        let roles = sqlx::query!(
            r#"
            SELECT * FROM roles WHERE created_by = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get roles: {:?}", e);
            "Failed to get roles".to_string()
        })?;

        Ok(roles
            .into_iter()
            .map(|role| RoleResponse {
                id: role.id,
                name: role.name,
                description: role.description.unwrap_or_default(),
                created_by: role.created_by,
                created_at: role.created_at,
            })
            .collect())
    }
}
