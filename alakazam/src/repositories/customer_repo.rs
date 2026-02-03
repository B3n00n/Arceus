use crate::{error::Result, models::Customer};
use sqlx::PgPool;

pub struct CustomerRepository {
    pool: PgPool,
}

impl CustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new customer
    pub async fn create(
        &self,
        name: &str,
        phone_number: Option<&str>,
        email: Option<&str>,
    ) -> Result<Customer> {
        let customer = sqlx::query_as::<_, Customer>(
            "INSERT INTO customers (name, phone_number, email)
             VALUES ($1, $2, $3)
             RETURNING id, name, phone_number, email, created_at"
        )
        .bind(name)
        .bind(phone_number)
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(customer)
    }

    /// List all customers
    pub async fn list_all(&self) -> Result<Vec<Customer>> {
        let customers = sqlx::query_as::<_, Customer>(
            "SELECT id, name, phone_number, email, created_at
             FROM customers
             ORDER BY name ASC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(customers)
    }

    /// Get customer by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Customer>> {
        let customer = sqlx::query_as::<_, Customer>(
            "SELECT id, name, phone_number, email, created_at
             FROM customers
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(customer)
    }

    /// Update customer
    pub async fn update(
        &self,
        id: i32,
        name: &str,
        phone_number: Option<&str>,
        email: Option<&str>,
    ) -> Result<Customer> {
        let customer = sqlx::query_as::<_, Customer>(
            "UPDATE customers
             SET name = $2, phone_number = $3, email = $4
             WHERE id = $1
             RETURNING id, name, phone_number, email, created_at"
        )
        .bind(id)
        .bind(name)
        .bind(phone_number)
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(customer)
    }

    /// Delete customer (will fail if customer has arcades due to FK constraint)
    pub async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM customers WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get arcade IDs assigned to a customer
    pub async fn get_arcade_ids(&self, customer_id: i32) -> Result<Vec<i32>> {
        let ids = sqlx::query_scalar::<_, i32>(
            "SELECT id FROM arcades WHERE customer_id = $1 ORDER BY id"
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(ids)
    }

    /// Set arcade assignments for a customer (replaces existing)
    /// This sets customer_id on the specified arcades and clears it from any previously assigned arcades
    pub async fn set_arcade_assignments(&self, customer_id: i32, arcade_ids: &[i32]) -> Result<()> {
        // First, unassign all arcades currently assigned to this customer
        sqlx::query("UPDATE arcades SET customer_id = NULL WHERE customer_id = $1")
            .bind(customer_id)
            .execute(&self.pool)
            .await?;

        // Then, assign the specified arcades to this customer
        if !arcade_ids.is_empty() {
            sqlx::query(
                "UPDATE arcades SET customer_id = $1 WHERE id = ANY($2)"
            )
            .bind(customer_id)
            .bind(arcade_ids)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    /// Check if customer has any arcades assigned
    pub async fn has_arcades(&self, customer_id: i32) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM arcades WHERE customer_id = $1"
        )
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}
