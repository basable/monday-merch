//! `CatalogService`: the Connect service of `catalog`. Each
//! method validates the request, converts the buffa message into the plain
//! `messages` struct, sends it through the api sender, and converts the
//! answer back. Errors map to Connect codes through basable_connect.

use std::sync::Arc;

use basable_connect::{ConnectRouter, into_connect_error, request_ctx};
use connectrpc::{RequestContext, ServiceRequest, ServiceResult};
use interfaces::{ApiRoutes, ApiSender};
use proto::connect::catalog::v1::CatalogService;
use proto::proto::catalog::v1::*;

use crate::Api;

/// The service holds the api sender (exactly the sends `api` declares),
/// built once from the router at mount.
pub struct CatalogServiceImpl<R: ApiRoutes + Send + Sync + 'static> {
    sender: ApiSender<'static, R>,
}

// The generated trait returns `impl Encodable`; answering with the owned
// message is the refinement this allow names.
#[allow(refining_impl_trait)]
impl<R: ApiRoutes + Send + Sync + 'static> CatalogService for CatalogServiceImpl<R> {
    async fn list_products(&self, ctx: RequestContext, request: ServiceRequest<'_, ListProductsRequest>) -> ServiceResult<ListProductsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListProductsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListProductsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_products")))
    }
    async fn get_product(&self, ctx: RequestContext, request: ServiceRequest<'_, GetProductRequest>) -> ServiceResult<GetProductResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetProductRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetProductResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_product")))
    }
    async fn list_categories(&self, ctx: RequestContext, request: ServiceRequest<'_, ListCategoriesRequest>) -> ServiceResult<ListCategoriesResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListCategoriesRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListCategoriesResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_categories")))
    }
    async fn price_lines(&self, ctx: RequestContext, request: ServiceRequest<'_, PriceLinesRequest>) -> ServiceResult<PriceLinesResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::PriceLinesRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `PriceLinesResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.price_lines")))
    }
    async fn upsert_product(&self, ctx: RequestContext, request: ServiceRequest<'_, UpsertProductRequest>) -> ServiceResult<UpsertProductResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpsertProductRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpsertProductResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.upsert_product")))
    }
    async fn upsert_variant(&self, ctx: RequestContext, request: ServiceRequest<'_, UpsertVariantRequest>) -> ServiceResult<UpsertVariantResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpsertVariantRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpsertVariantResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.upsert_variant")))
    }
    async fn set_price_tiers(&self, ctx: RequestContext, request: ServiceRequest<'_, SetPriceTiersRequest>) -> ServiceResult<SetPriceTiersResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::SetPriceTiersRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `SetPriceTiersResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.set_price_tiers")))
    }
    async fn set_decoration_options(&self, ctx: RequestContext, request: ServiceRequest<'_, SetDecorationOptionsRequest>) -> ServiceResult<SetDecorationOptionsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::SetDecorationOptionsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `SetDecorationOptionsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.set_decoration_options")))
    }
    async fn set_supplier_offers(&self, ctx: RequestContext, request: ServiceRequest<'_, SetSupplierOffersRequest>) -> ServiceResult<SetSupplierOffersResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::SetSupplierOffersRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `SetSupplierOffersResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.set_supplier_offers")))
    }
    async fn create_image_upload(&self, ctx: RequestContext, request: ServiceRequest<'_, CreateImageUploadRequest>) -> ServiceResult<CreateImageUploadResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CreateImageUploadRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CreateImageUploadResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.create_image_upload")))
    }
    async fn archive_product(&self, ctx: RequestContext, request: ServiceRequest<'_, ArchiveProductRequest>) -> ServiceResult<ArchiveProductResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ArchiveProductRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ArchiveProductResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.archive_product")))
    }
}

pub fn mount<R>(connect: &mut ConnectRouter, _api: &'static Api, router: &'static R)
where
    R: ApiRoutes + Send + Sync + 'static,
{
    connect.add_service(Arc::new(CatalogServiceImpl { sender: ApiSender::new(router) }));
}
