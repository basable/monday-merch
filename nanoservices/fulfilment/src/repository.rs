//! Typed sqlx queries over this nanoservice's own pool (schema
//! `nano_fulfilment`). Only this crate queries these tables; another
//! nanoservice that needs the data sends a message (the Directive §2).
//! Errors are `sqlx::Error` as they come; the handler decides what a
//! failure means to its caller (`AppError::wrap`).

use uuid::Uuid;

// Rows are `model::<Table>`, qualified: the component struct below and a
// message type in `messages` may carry the same name as a table.
use crate::model;
use crate::Fulfilment;

impl Fulfilment {
    // --- stock_item ---

    pub async fn insert_stock_item(&self, row: &model::StockItem) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO stock_item (id, company_id, name, product_id, variant_id, size, proof_id, is_kit) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(&row.name)
        .bind(row.product_id)
        .bind(row.variant_id)
        .bind(&row.size)
        .bind(row.proof_id)
        .bind(row.is_kit)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_stock_item(&self, id: Uuid) -> Result<Option<model::StockItem>, sqlx::Error> {
        sqlx::query_as::<_, model::StockItem>("SELECT id, company_id, name, product_id, variant_id, size, proof_id, is_kit FROM stock_item WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- stock_level ---

    pub async fn insert_stock_level(&self, row: &model::StockLevel) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO stock_level (id, stock_item_id, warehouse_slug, on_hand, reserved) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(row.id)
        .bind(row.stock_item_id)
        .bind(&row.warehouse_slug)
        .bind(row.on_hand)
        .bind(row.reserved)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_stock_level(&self, id: Uuid) -> Result<Option<model::StockLevel>, sqlx::Error> {
        sqlx::query_as::<_, model::StockLevel>("SELECT id, stock_item_id, warehouse_slug, on_hand, reserved FROM stock_level WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- stock_movement ---

    pub async fn insert_stock_movement(&self, row: &model::StockMovement) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO stock_movement (id, stock_item_id, warehouse_slug, kind, on_hand_delta, reserved_delta, source, actor, occurred_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(row.id)
        .bind(row.stock_item_id)
        .bind(&row.warehouse_slug)
        .bind(&row.kind)
        .bind(row.on_hand_delta)
        .bind(row.reserved_delta)
        .bind(&row.source)
        .bind(&row.actor)
        .bind(&row.occurred_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_stock_movement(&self, id: Uuid) -> Result<Option<model::StockMovement>, sqlx::Error> {
        sqlx::query_as::<_, model::StockMovement>("SELECT id, stock_item_id, warehouse_slug, kind, on_hand_delta, reserved_delta, source, actor, occurred_at FROM stock_movement WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- kit_component ---

    pub async fn insert_kit_component(&self, row: &model::KitComponent) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO kit_component (id, kit_item_id, component_item_id, quantity) VALUES ($1, $2, $3, $4)",
        )
        .bind(row.id)
        .bind(row.kit_item_id)
        .bind(row.component_item_id)
        .bind(row.quantity)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_kit_component(&self, id: Uuid) -> Result<Option<model::KitComponent>, sqlx::Error> {
        sqlx::query_as::<_, model::KitComponent>("SELECT id, kit_item_id, component_item_id, quantity FROM kit_component WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- assembly_order ---

    pub async fn insert_assembly_order(&self, row: &model::AssemblyOrder) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO assembly_order (id, company_id, kit_item_id, warehouse_slug, quantity, completed_at, cancelled_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(row.kit_item_id)
        .bind(&row.warehouse_slug)
        .bind(row.quantity)
        .bind(&row.completed_at)
        .bind(&row.cancelled_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_assembly_order(&self, id: Uuid) -> Result<Option<model::AssemblyOrder>, sqlx::Error> {
        sqlx::query_as::<_, model::AssemblyOrder>("SELECT id, company_id, kit_item_id, warehouse_slug, quantity, completed_at, cancelled_at FROM assembly_order WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- inbound ---

    pub async fn insert_inbound(&self, row: &model::Inbound) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO inbound (id, name, source_generation, company_id, warehouse_slug, lines, received_lines, received_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(row.id)
        .bind(&row.name)
        .bind(row.source_generation)
        .bind(row.company_id)
        .bind(&row.warehouse_slug)
        .bind(&row.lines)
        .bind(&row.received_lines)
        .bind(&row.received_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_inbound(&self, id: Uuid) -> Result<Option<model::Inbound>, sqlx::Error> {
        sqlx::query_as::<_, model::Inbound>("SELECT id, name, source_generation, company_id, warehouse_slug, lines, received_lines, received_at FROM inbound WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- low_stock_rule ---

    pub async fn insert_low_stock_rule(&self, row: &model::LowStockRule) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO low_stock_rule (id, stock_item_id, threshold, notify_email) VALUES ($1, $2, $3, $4)",
        )
        .bind(row.id)
        .bind(row.stock_item_id)
        .bind(row.threshold)
        .bind(&row.notify_email)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_low_stock_rule(&self, id: Uuid) -> Result<Option<model::LowStockRule>, sqlx::Error> {
        sqlx::query_as::<_, model::LowStockRule>("SELECT id, stock_item_id, threshold, notify_email FROM low_stock_rule WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- shipment_batch ---

    pub async fn insert_shipment_batch(&self, row: &model::ShipmentBatch) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO shipment_batch (id, company_id, created_by, charges, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(&row.created_by)
        .bind(&row.charges)
        .bind(&row.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_shipment_batch(&self, id: Uuid) -> Result<Option<model::ShipmentBatch>, sqlx::Error> {
        sqlx::query_as::<_, model::ShipmentBatch>("SELECT id, company_id, created_by, charges, created_at FROM shipment_batch WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- shipment_address ---

    pub async fn insert_shipment_address(&self, row: &model::ShipmentAddress) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO shipment_address (shipment_id, address, erase_after, erased_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(row.shipment_id)
        .bind(&row.address)
        .bind(&row.erase_after)
        .bind(&row.erased_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_shipment_address(&self, id: Uuid) -> Result<Option<model::ShipmentAddress>, sqlx::Error> {
        sqlx::query_as::<_, model::ShipmentAddress>("SELECT shipment_id, address, erase_after, erased_at FROM shipment_address WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- shipment_event ---

    pub async fn insert_shipment_event(&self, row: &model::ShipmentEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO shipment_event (id, shipment_id, kind, detail, dedupe_key, occurred_at) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(row.id)
        .bind(row.shipment_id)
        .bind(&row.kind)
        .bind(&row.detail)
        .bind(&row.dedupe_key)
        .bind(&row.occurred_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_shipment_event(&self, id: Uuid) -> Result<Option<model::ShipmentEvent>, sqlx::Error> {
        sqlx::query_as::<_, model::ShipmentEvent>("SELECT id, shipment_id, kind, detail, dedupe_key, occurred_at FROM shipment_event WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}
