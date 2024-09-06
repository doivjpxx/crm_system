use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateResourceRequest {
    pub plan_id: uuid::Uuid,
    pub max: i64,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ResourceResponse {
    pub id: uuid::Uuid,
    pub plan_id: uuid::Uuid,
    pub max: i64,
    pub name: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub struct ResourceService {
    pub pool: sqlx::PgPool,
}

pub trait ResourceServiceImpl {
     fn new(pool: sqlx::PgPool) -> Self;

     async fn create_resource(&self, resource: CreateResourceRequest) -> Result<(), String>;

     async fn get_resource(&self, id: uuid::Uuid) -> Result<ResourceResponse, String>;

     async fn get_resources_for_plan(
        &self,
        plan_id: uuid::Uuid,
    ) -> Result<Vec<ResourceResponse>, String>;

     async fn update_resource(
        &self,
        id: uuid::Uuid,
        resource: CreateResourceRequest,
    ) -> Result<(), String>;

     async fn get_resources(&self) -> Result<Vec<ResourceResponse>, String>;
}

impl ResourceServiceImpl for ResourceService {
     fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

     async fn create_resource(&self, resource: CreateResourceRequest) -> Result<(), String> {
        sqlx::query!(
            r#"
            INSERT INTO resources (plan_id, max, name, description)
            VALUES ($1, $2, $3, $4)
            "#,
            resource.plan_id,
            resource.max,
            resource.name,
            resource.description
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create resource: {:?}", e);
            "Failed to create resource".to_string()
        })?;

        Ok(())
    }

     async fn get_resource(&self, id: uuid::Uuid) -> Result<ResourceResponse, String> {
        let resource = sqlx::query!(
            r#"
            SELECT id, plan_id, max, name, description, created_at
            FROM resources
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get resource: {:?}", e);
            "Failed to get resource".to_string()
        })?;

        Ok(ResourceResponse {
            id: resource.id,
            plan_id: resource.plan_id,
            max: resource.max,
            name: resource.name,
            description: resource.description.unwrap_or_default(),
            created_at: resource.created_at,
        })
    }

     async fn get_resources_for_plan(
        &self,
        plan_id: uuid::Uuid,
    ) -> Result<Vec<ResourceResponse>, String> {
        let resources = sqlx::query!(
            r#"
            SELECT id, plan_id, max, name, description, created_at
            FROM resources
            WHERE plan_id = $1
            "#,
            plan_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get resources: {:?}", e);
            "Failed to get resources".to_string()
        })?;

        Ok(resources
            .into_iter()
            .map(|resource| ResourceResponse {
                id: resource.id,
                plan_id: resource.plan_id,
                max: resource.max,
                name: resource.name,
                description: resource.description.unwrap_or_default(),
                created_at: resource.created_at,
            })
            .collect())
    }

     async fn update_resource(
        &self,
        id: uuid::Uuid,
        resource: CreateResourceRequest,
    ) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE resources
            SET plan_id = $1, max = $2, name = $3, description = $4
            WHERE id = $5
            "#,
            resource.plan_id,
            resource.max,
            resource.name,
            resource.description,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update resource: {:?}", e);
            "Failed to update resource".to_string()
        })?;

        Ok(())
    }

     async fn get_resources(&self) -> Result<Vec<ResourceResponse>, String> {
        let resources = sqlx::query!(
            r#"
            SELECT id, plan_id, max, name, description, created_at
            FROM resources
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get resources: {:?}", e);
            "Failed to get resources".to_string()
        })?;

        Ok(resources
            .into_iter()
            .map(|resource| ResourceResponse {
                id: resource.id,
                plan_id: resource.plan_id,
                max: resource.max,
                name: resource.name,
                description: resource.description.unwrap_or_default(),
                created_at: resource.created_at,
            })
            .collect())
    }
}
