use sqlx::{PgPool, types::time::OffsetDateTime};

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(type_name = "ticket_status", rename_all = "lowercase")]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
}

#[derive(Debug)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub status: TicketStatus,
}

pub async fn create_ticket(pool: &PgPool, title: &str, desc: &str) -> anyhow::Result<i32> {
    let req = sqlx::query!(
        r#"
        INSERT INTO tickets (title, description)
        VALUES ($1, $2)
        RETURNING id
        "#,
        title,
        desc
    )
    .fetch_one(pool)
    .await?;

    Ok(req.id)
}

pub async fn get_all_tickets(pool: &PgPool) -> anyhow::Result<Vec<Ticket>> {
    let tickets = sqlx::query_as!(
        Ticket, r#"SELECT id, title, description, created_at, status AS "status: TicketStatus" FROM tickets ORDER BY created_at DESC"#
    )
    .fetch_all(pool)
    .await?;

    Ok(tickets)
}

pub async fn update_ticket_securely(pool: PgPool, ticket_id: i32) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "UPDATE tickets SET title = 'Reviewed' WHERE id = $1",
        ticket_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
