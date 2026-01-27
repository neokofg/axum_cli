use anyhow::Result;
use tera::{Context, Tera};

pub fn create_engine() -> Result<Tera> {
    let mut tera = Tera::default();

    // Feature templates
    tera.add_raw_template("feature/mod.rs", FEATURE_MOD)?;
    tera.add_raw_template("feature/model.rs", FEATURE_MODEL)?;
    tera.add_raw_template("feature/dto.rs", FEATURE_DTO)?;
    tera.add_raw_template("feature/repository.rs", FEATURE_REPOSITORY)?;
    tera.add_raw_template("feature/service.rs", FEATURE_SERVICE)?;
    tera.add_raw_template("feature/handlers.rs", FEATURE_HANDLERS)?;
    tera.add_raw_template("feature/routes.rs", FEATURE_ROUTES)?;
    tera.add_raw_template("feature/tests.rs", FEATURE_TESTS)?;

    // Migration template
    tera.add_raw_template("migration/up.sql", MIGRATION_UP)?;
    tera.add_raw_template("migration/down.sql", MIGRATION_DOWN)?;

    // Worker template
    tera.add_raw_template("worker/mod.rs", WORKER_TEMPLATE)?;

    // Middleware template
    tera.add_raw_template("middleware/mod.rs", MIDDLEWARE_TEMPLATE)?;

    Ok(tera)
}

pub fn create_context(name: &str) -> Context {
    let mut ctx = Context::new();
    let snake = crate::utils::to_snake_case(name);
    let pascal = crate::utils::to_pascal_case(name);
    let plural = crate::utils::pluralize(&snake);
    let singular = crate::utils::singularize(&snake);

    ctx.insert("name", &snake);
    ctx.insert("name_pascal", &pascal);
    ctx.insert("name_plural", &plural);
    ctx.insert("name_singular", &singular);
    ctx.insert("table_name", &plural);

    ctx
}

// ==================== FEATURE TEMPLATES ====================

const FEATURE_MOD: &str = r#"mod dto;
mod handlers;
mod model;
mod repository;
mod routes;
mod service;

#[cfg(test)]
mod tests;

pub use dto::*;
pub use model::*;
pub use repository::*;
pub use routes::{protected_router, public_router};
pub use service::*;
"#;

const FEATURE_MODEL: &str = r#"use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::{{ table_name }};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = {{ table_name }})]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct {{ name_pascal }} {
    pub id: Uuid,
{%- for field in fields %}
    pub {{ field.name }}: {{ field.rust_type }},
{%- endfor %}
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = {{ table_name }})]
pub struct New{{ name_pascal }} {
    pub id: Uuid,
{%- for field in fields %}
    pub {{ field.name }}: {{ field.rust_type }},
{%- endfor %}
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = {{ table_name }})]
pub struct Update{{ name_pascal }} {
{%- for field in fields %}
    pub {{ field.name }}: Option<{{ field.rust_type }}>,
{%- endfor %}
    pub updated_at: DateTime<Utc>,
}
"#;

const FEATURE_DTO: &str = r#"use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::{{ name_pascal }};

#[derive(Debug, Deserialize, Validate)]
pub struct Create{{ name_pascal }}Request {
{%- for field in fields %}
{%- if field.validation %}
    #[validate({{ field.validation }})]
{%- endif %}
    pub {{ field.name }}: {{ field.rust_type }},
{%- endfor %}
}

#[derive(Debug, Deserialize, Validate)]
pub struct Update{{ name_pascal }}Request {
{%- for field in fields %}
{%- if field.validation %}
    #[validate({{ field.validation }})]
{%- endif %}
    pub {{ field.name }}: Option<{{ field.rust_type }}>,
{%- endfor %}
}

#[derive(Debug, Serialize)]
pub struct {{ name_pascal }}Response {
    pub id: Uuid,
{%- for field in fields %}
    pub {{ field.name }}: {{ field.rust_type }},
{%- endfor %}
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<{{ name_pascal }}> for {{ name_pascal }}Response {
    fn from(model: {{ name_pascal }}) -> Self {
        Self {
            id: model.id,
{%- for field in fields %}
            {{ field.name }}: model.{{ field.name }},
{%- endfor %}
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
"#;

const FEATURE_REPOSITORY: &str = r#"use diesel::prelude::*;
use uuid::Uuid;

use super::{New{{ name_pascal }}, Update{{ name_pascal }}, {{ name_pascal }}};
use crate::config::DbConnection;
use crate::core::ApiError;
use crate::schema::{{ table_name }};

pub struct {{ name_pascal }}Repository;

impl {{ name_pascal }}Repository {
    pub fn find_by_id(conn: &mut DbConnection, id: Uuid) -> Result<{{ name_pascal }}, ApiError> {
        {{ table_name }}::table
            .filter({{ table_name }}::id.eq(id))
            .first(conn)
            .map_err(ApiError::from)
    }

    pub fn find_all(
        conn: &mut DbConnection,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<{{ name_pascal }}>, ApiError> {
        {{ table_name }}::table
            .order({{ table_name }}::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load(conn)
            .map_err(ApiError::from)
    }

    pub fn count(conn: &mut DbConnection) -> Result<i64, ApiError> {
        {{ table_name }}::table
            .count()
            .get_result(conn)
            .map_err(ApiError::from)
    }

    pub fn create(conn: &mut DbConnection, new_item: New{{ name_pascal }}) -> Result<{{ name_pascal }}, ApiError> {
        diesel::insert_into({{ table_name }}::table)
            .values(&new_item)
            .returning({{ name_pascal }}::as_returning())
            .get_result(conn)
            .map_err(ApiError::from)
    }

    pub fn update(
        conn: &mut DbConnection,
        id: Uuid,
        update_item: Update{{ name_pascal }},
    ) -> Result<{{ name_pascal }}, ApiError> {
        diesel::update({{ table_name }}::table.filter({{ table_name }}::id.eq(id)))
            .set(&update_item)
            .returning({{ name_pascal }}::as_returning())
            .get_result(conn)
            .map_err(ApiError::from)
    }

    pub fn delete(conn: &mut DbConnection, id: Uuid) -> Result<usize, ApiError> {
        diesel::delete({{ table_name }}::table.filter({{ table_name }}::id.eq(id)))
            .execute(conn)
            .map_err(ApiError::from)
    }
}
"#;

const FEATURE_SERVICE: &str = r#"use chrono::Utc;
use uuid::Uuid;

use super::{
    Create{{ name_pascal }}Request, New{{ name_pascal }}, Update{{ name_pascal }},
    Update{{ name_pascal }}Request, {{ name_pascal }}, {{ name_pascal }}Repository,
};
use crate::config::DbPool;
use crate::core::{ApiError, Paginated, PaginationParams};

pub struct {{ name_pascal }}Service;

impl {{ name_pascal }}Service {
    pub fn find_by_id(pool: &DbPool, id: Uuid) -> Result<{{ name_pascal }}, ApiError> {
        let mut conn = pool.get()?;
        {{ name_pascal }}Repository::find_by_id(&mut conn, id)
    }

    pub fn list(pool: &DbPool, params: &PaginationParams) -> Result<Paginated<{{ name_pascal }}>, ApiError> {
        let mut conn = pool.get()?;
        let items = {{ name_pascal }}Repository::find_all(&mut conn, params.limit(), params.offset())?;
        let total = {{ name_pascal }}Repository::count(&mut conn)?;

        Ok(Paginated::new(items, total, params))
    }

    pub fn create(pool: &DbPool, request: Create{{ name_pascal }}Request) -> Result<{{ name_pascal }}, ApiError> {
        let mut conn = pool.get()?;

        let new_item = New{{ name_pascal }} {
            id: Uuid::new_v4(),
{%- for field in fields %}
            {{ field.name }}: request.{{ field.name }},
{%- endfor %}
        };

        {{ name_pascal }}Repository::create(&mut conn, new_item)
    }

    pub fn update(pool: &DbPool, id: Uuid, request: Update{{ name_pascal }}Request) -> Result<{{ name_pascal }}, ApiError> {
        let mut conn = pool.get()?;

        // Check if exists
        {{ name_pascal }}Repository::find_by_id(&mut conn, id)?;

        let update_item = Update{{ name_pascal }} {
{%- for field in fields %}
            {{ field.name }}: request.{{ field.name }},
{%- endfor %}
            updated_at: Utc::now(),
        };

        {{ name_pascal }}Repository::update(&mut conn, id, update_item)
    }

    pub fn delete(pool: &DbPool, id: Uuid) -> Result<(), ApiError> {
        let mut conn = pool.get()?;

        // Check if exists
        {{ name_pascal }}Repository::find_by_id(&mut conn, id)?;

        {{ name_pascal }}Repository::delete(&mut conn, id)?;
        Ok(())
    }
}
"#;

const FEATURE_HANDLERS: &str = r#"use axum::extract::{Path, Query, State};
use uuid::Uuid;

use super::{
    Create{{ name_pascal }}Request, Update{{ name_pascal }}Request,
    {{ name_pascal }}Response, {{ name_pascal }}Service,
};
use crate::core::extractors::ValidatedJson;
use crate::core::{ApiError, ApiResponse, Created, NoContent, PaginationParams};
use crate::AppState;

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<ApiResponse<Vec<{{ name_pascal }}Response>>, ApiError> {
    let paginated = {{ name_pascal }}Service::list(&state.db_pool, &params)?;
    let meta = paginated.meta();
    let items: Vec<{{ name_pascal }}Response> = paginated.items.into_iter().map(Into::into).collect();

    Ok(ApiResponse::with_meta(items, meta))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<{{ name_pascal }}Response>, ApiError> {
    let item = {{ name_pascal }}Service::find_by_id(&state.db_pool, id)?;
    Ok(ApiResponse::success(item.into()))
}

pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<Create{{ name_pascal }}Request>,
) -> Result<Created<{{ name_pascal }}Response>, ApiError> {
    let item = {{ name_pascal }}Service::create(&state.db_pool, request)?;
    Ok(Created(item.into()))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    ValidatedJson(request): ValidatedJson<Update{{ name_pascal }}Request>,
) -> Result<ApiResponse<{{ name_pascal }}Response>, ApiError> {
    let item = {{ name_pascal }}Service::update(&state.db_pool, id, request)?;
    Ok(ApiResponse::success(item.into()))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<NoContent, ApiError> {
    {{ name_pascal }}Service::delete(&state.db_pool, id)?;
    Ok(NoContent)
}
"#;

const FEATURE_ROUTES: &str = r#"use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::handlers;
use crate::AppState;

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list))
        .route("/", post(handlers::create))
        .route("/{id}", get(handlers::get))
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/{id}", put(handlers::update))
        .route("/{id}", delete(handlers::delete))
}
"#;

const FEATURE_TESTS: &str = r#"#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::{{ name }}::dto::{Create{{ name_pascal }}Request, Update{{ name_pascal }}Request};
    use validator::Validate;

    #[test]
    fn test_create_request_validation() {
        // Add validation tests here
    }

    #[test]
    fn test_update_request_validation() {
        // Add validation tests here
    }
}
"#;

// ==================== MIGRATION TEMPLATES ====================

const MIGRATION_UP: &str = r#"CREATE TABLE {{ table_name }} (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
{%- for field in fields %}
    {{ field.name }} {{ field.sql_type }}{% if not field.nullable %} NOT NULL{% endif %}{% if field.default %} DEFAULT {{ field.default }}{% endif %},
{%- endfor %}
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_{{ table_name }}_created_at ON {{ table_name }}(created_at);
"#;

const MIGRATION_DOWN: &str = r#"DROP TABLE IF EXISTS {{ table_name }};
"#;

// ==================== WORKER TEMPLATE ====================

const WORKER_TEMPLATE: &str = r#"use serde::{Deserialize, Serialize};
use tracing::info;

use crate::infrastructure::queue::QueueClient;

pub const {{ name | upper }}_JOB_TYPE: &str = "{{ name }}";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ name_pascal }}Args {
    // Add job arguments here
    pub id: String,
}

pub struct {{ name_pascal }}Worker {
    queue: QueueClient,
}

impl {{ name_pascal }}Worker {
    pub fn new(queue: QueueClient) -> Self {
        Self {
            queue: queue.with_queue("{{ name }}"),
        }
    }

    pub async fn enqueue(&self, args: {{ name_pascal }}Args) -> Result<String, redis::RedisError> {
        self.queue.enqueue({{ name | upper }}_JOB_TYPE, args).await
    }
}

pub async fn process_{{ name }}_job(job: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let args: {{ name_pascal }}Args = serde_json::from_value(job["args"].clone())?;

    info!(id = %args.id, "Processing {{ name }} job");

    // TODO: Implement job logic here

    info!(id = %args.id, "{{ name_pascal }} job completed");

    Ok(())
}
"#;

// ==================== MIDDLEWARE TEMPLATE ====================

const MIDDLEWARE_TEMPLATE: &str = r#"use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;

pub async fn {{ name }}_middleware(request: Request<Body>, next: Next) -> Response {
    // Before request
    info!("{{ name_pascal }} middleware: before request");

    let response = next.run(request).await;

    // After request
    info!("{{ name_pascal }} middleware: after request");

    response
}
"#;
