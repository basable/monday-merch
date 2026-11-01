//! The handler trait implementation: one method per message this
//! nanoservice handles (routing.yaml). Handlers are the INTENT boundary:
//! they validate, write desired state through the typed store (`create`,
//! `update_spec`, `mark_deleted`, `nudge`) or the repository, dispatch
//! request-path adapters `Unfenced`, and send through `s` (the sender that
//! exposes exactly the declared sends). They never write status.

use basable_core::{AppError, Ctx};
use interfaces::{CatalogHandler, CatalogRoutes, CatalogSender};

use crate::Catalog;

// Messages stay qualified: a message may share its name with this
// component (an `order` nanoservice handling an `Order` response).
impl<R: CatalogRoutes> CatalogHandler<R> for Catalog {
    async fn handle_list_products_request(&self, _ctx: &Ctx, _msg: messages::ListProductsRequest, _s: CatalogSender<'_, R>) -> Result<messages::ProductPage, AppError> {
        Err(crate::unimplemented_step("catalog.handle_list_products_request"))
    }
    async fn handle_get_product_request(&self, _ctx: &Ctx, _msg: messages::GetProductRequest, _s: CatalogSender<'_, R>) -> Result<messages::Product, AppError> {
        Err(crate::unimplemented_step("catalog.handle_get_product_request"))
    }
    async fn handle_list_categories_request(&self, _ctx: &Ctx, _msg: messages::ListCategoriesRequest, _s: CatalogSender<'_, R>) -> Result<messages::CategoryList, AppError> {
        Err(crate::unimplemented_step("catalog.handle_list_categories_request"))
    }
    async fn handle_price_lines_request(&self, _ctx: &Ctx, _msg: messages::PriceLinesRequest, _s: CatalogSender<'_, R>) -> Result<messages::PricedLines, AppError> {
        Err(crate::unimplemented_step("catalog.handle_price_lines_request"))
    }
    async fn handle_upsert_product_request(&self, _ctx: &Ctx, _msg: messages::UpsertProductRequest, _s: CatalogSender<'_, R>) -> Result<messages::Product, AppError> {
        Err(crate::unimplemented_step("catalog.handle_upsert_product_request"))
    }
    async fn handle_upsert_variant_request(&self, _ctx: &Ctx, _msg: messages::UpsertVariantRequest, _s: CatalogSender<'_, R>) -> Result<messages::Variant, AppError> {
        Err(crate::unimplemented_step("catalog.handle_upsert_variant_request"))
    }
    async fn handle_set_price_tiers_request(&self, _ctx: &Ctx, _msg: messages::SetPriceTiersRequest, _s: CatalogSender<'_, R>) -> Result<messages::Product, AppError> {
        Err(crate::unimplemented_step("catalog.handle_set_price_tiers_request"))
    }
    async fn handle_set_decoration_options_request(&self, _ctx: &Ctx, _msg: messages::SetDecorationOptionsRequest, _s: CatalogSender<'_, R>) -> Result<messages::Product, AppError> {
        Err(crate::unimplemented_step("catalog.handle_set_decoration_options_request"))
    }
    async fn handle_set_supplier_offers_request(&self, _ctx: &Ctx, _msg: messages::SetSupplierOffersRequest, _s: CatalogSender<'_, R>) -> Result<messages::SupplierOffers, AppError> {
        Err(crate::unimplemented_step("catalog.handle_set_supplier_offers_request"))
    }
    async fn handle_create_image_upload_request(&self, _ctx: &Ctx, _msg: messages::CreateImageUploadRequest, _s: CatalogSender<'_, R>) -> Result<messages::ImageUpload, AppError> {
        Err(crate::unimplemented_step("catalog.handle_create_image_upload_request"))
    }
    async fn handle_archive_product_request(&self, _ctx: &Ctx, _msg: messages::ArchiveProductRequest, _s: CatalogSender<'_, R>) -> Result<messages::Product, AppError> {
        Err(crate::unimplemented_step("catalog.handle_archive_product_request"))
    }
    async fn handle_get_supplier_offers_request(&self, _ctx: &Ctx, _msg: messages::GetSupplierOffersRequest, _s: CatalogSender<'_, R>) -> Result<messages::SupplierOffers, AppError> {
        Err(crate::unimplemented_step("catalog.handle_get_supplier_offers_request"))
    }
    async fn handle_get_customs_data_request(&self, _ctx: &Ctx, _msg: messages::GetCustomsDataRequest, _s: CatalogSender<'_, R>) -> Result<messages::CustomsData, AppError> {
        Err(crate::unimplemented_step("catalog.handle_get_customs_data_request"))
    }
    async fn handle_match_supplier_request(&self, _ctx: &Ctx, _msg: messages::MatchSupplierRequest, _s: CatalogSender<'_, R>) -> Result<messages::SupplierMatch, AppError> {
        Err(crate::unimplemented_step("catalog.handle_match_supplier_request"))
    }
    async fn handle_render_shop_page_request(&self, _ctx: &Ctx, _msg: messages::RenderShopPageRequest, _s: CatalogSender<'_, R>) -> Result<messages::ShopPage, AppError> {
        Err(crate::unimplemented_step("catalog.handle_render_shop_page_request"))
    }
}
