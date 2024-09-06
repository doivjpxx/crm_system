use crate::models::user_group_model::UserGroupModel;

pub struct UserGroupService {
    pub pool: sqlx::PgPool,
}

pub trait UserGroupServiceImpl {
    fn new(pool: sqlx::PgPool) -> Self;
    async fn get_user_groups_by_parent_id(
        &self,
        parent_id: uuid::Uuid,
    ) -> Result<Vec<UserGroupModel>, String>;
    async fn get_user_groups_by_child_id(
        &self,
        child_id: uuid::Uuid,
    ) -> Result<Vec<UserGroupModel>, String>;
}

impl UserGroupServiceImpl for UserGroupService {
    fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    async fn get_user_groups_by_parent_id(
        &self,
        parent_id: uuid::Uuid,
    ) -> Result<Vec<UserGroupModel>, String> {
        let user_groups = sqlx::query!(
            r#"
            SELECT id, user_id, parent_id, created_at
            FROM user_groups
            WHERE parent_id = $1
            "#,
            parent_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user groups: {:?}", e);
            "Failed to get user groups".to_string()
        })?;

        Ok(user_groups
            .into_iter()
            .map(|user_group| UserGroupModel {
                id: user_group.id,
                user_id: user_group.user_id,
                parent_id: user_group.parent_id,
                created_at: chrono::DateTime::from_utc(user_group.created_at.unwrap(), chrono::Utc),
            })
            .collect())
    }

    async fn get_user_groups_by_child_id(
        &self,
        child_id: uuid::Uuid,
    ) -> Result<Vec<UserGroupModel>, String> {
        let user_groups = sqlx::query!(
            r#"
            SELECT id, user_id, parent_id, created_at
            FROM user_groups
            WHERE user_id = $1
            "#,
            child_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user groups: {:?}", e);
            "Failed to get user groups".to_string()
        })?;

        Ok(user_groups
            .into_iter()
            .map(|user_group| UserGroupModel {
                id: user_group.id,
                user_id: user_group.user_id,
                parent_id: user_group.parent_id,
                created_at: chrono::DateTime::from_utc(user_group.created_at.unwrap(), chrono::Utc),
            })
            .collect())
    }
}
