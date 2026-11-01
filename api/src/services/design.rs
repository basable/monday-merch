//! `DesignService`: the Connect service of `design`. Each
//! method validates the request, converts the buffa message into the plain
//! `messages` struct, sends it through the api sender, and converts the
//! answer back. Errors map to Connect codes through basable_connect.

use std::sync::Arc;

use basable_connect::{ConnectRouter, into_connect_error, request_ctx};
use connectrpc::{RequestContext, ServiceRequest, ServiceResult};
use interfaces::{ApiRoutes, ApiSender};
use proto::connect::design::v1::DesignService;
use proto::proto::design::v1::*;

use crate::Api;

/// The service holds the api sender (exactly the sends `api` declares),
/// built once from the router at mount.
pub struct DesignServiceImpl<R: ApiRoutes + Send + Sync + 'static> {
    sender: ApiSender<'static, R>,
}

// The generated trait returns `impl Encodable`; answering with the owned
// message is the refinement this allow names.
#[allow(refining_impl_trait)]
impl<R: ApiRoutes + Send + Sync + 'static> DesignService for DesignServiceImpl<R> {
    async fn get_brand_profile(&self, ctx: RequestContext, request: ServiceRequest<'_, GetBrandProfileRequest>) -> ServiceResult<GetBrandProfileResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetBrandProfileRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetBrandProfileResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_brand_profile")))
    }
    async fn update_brand_profile(&self, ctx: RequestContext, request: ServiceRequest<'_, UpdateBrandProfileRequest>) -> ServiceResult<UpdateBrandProfileResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpdateBrandProfileRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpdateBrandProfileResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.update_brand_profile")))
    }
    async fn list_brand_assets(&self, ctx: RequestContext, request: ServiceRequest<'_, ListBrandAssetsRequest>) -> ServiceResult<ListBrandAssetsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListBrandAssetsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListBrandAssetsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_brand_assets")))
    }
    async fn create_asset_upload(&self, ctx: RequestContext, request: ServiceRequest<'_, CreateAssetUploadRequest>) -> ServiceResult<CreateAssetUploadResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CreateAssetUploadRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CreateAssetUploadResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.create_asset_upload")))
    }
    async fn confirm_asset_upload(&self, ctx: RequestContext, request: ServiceRequest<'_, ConfirmAssetUploadRequest>) -> ServiceResult<ConfirmAssetUploadResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ConfirmAssetUploadRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ConfirmAssetUploadResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.confirm_asset_upload")))
    }
    async fn delete_brand_asset(&self, ctx: RequestContext, request: ServiceRequest<'_, DeleteBrandAssetRequest>) -> ServiceResult<DeleteBrandAssetResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::DeleteBrandAssetRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `DeleteBrandAssetResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.delete_brand_asset")))
    }
    async fn create_proof(&self, ctx: RequestContext, request: ServiceRequest<'_, CreateProofRequest>) -> ServiceResult<CreateProofResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CreateProofRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CreateProofResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.create_proof")))
    }
    async fn list_proofs(&self, ctx: RequestContext, request: ServiceRequest<'_, ListProofsRequest>) -> ServiceResult<ListProofsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListProofsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListProofsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_proofs")))
    }
    async fn create_design(&self, ctx: RequestContext, request: ServiceRequest<'_, CreateDesignRequest>) -> ServiceResult<CreateDesignResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CreateDesignRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CreateDesignResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.create_design")))
    }
    async fn get_design(&self, ctx: RequestContext, request: ServiceRequest<'_, GetDesignRequest>) -> ServiceResult<GetDesignResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetDesignRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetDesignResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_design")))
    }
    async fn list_designs(&self, ctx: RequestContext, request: ServiceRequest<'_, ListDesignsRequest>) -> ServiceResult<ListDesignsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListDesignsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListDesignsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_designs")))
    }
    async fn save_design_draft(&self, ctx: RequestContext, request: ServiceRequest<'_, SaveDesignDraftRequest>) -> ServiceResult<SaveDesignDraftResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::SaveDesignDraftRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `SaveDesignDraftResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.save_design_draft")))
    }
    async fn save_design_version(&self, ctx: RequestContext, request: ServiceRequest<'_, SaveDesignVersionRequest>) -> ServiceResult<SaveDesignVersionResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::SaveDesignVersionRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `SaveDesignVersionResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.save_design_version")))
    }
    async fn archive_design(&self, ctx: RequestContext, request: ServiceRequest<'_, ArchiveDesignRequest>) -> ServiceResult<ArchiveDesignResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ArchiveDesignRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ArchiveDesignResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.archive_design")))
    }
    async fn create_preview_upload(&self, ctx: RequestContext, request: ServiceRequest<'_, CreatePreviewUploadRequest>) -> ServiceResult<CreatePreviewUploadResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CreatePreviewUploadRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CreatePreviewUploadResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.create_preview_upload")))
    }
    async fn list_design_templates(&self, ctx: RequestContext, request: ServiceRequest<'_, ListDesignTemplatesRequest>) -> ServiceResult<ListDesignTemplatesResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListDesignTemplatesRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListDesignTemplatesResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_design_templates")))
    }
    async fn get_design_template(&self, ctx: RequestContext, request: ServiceRequest<'_, GetDesignTemplateRequest>) -> ServiceResult<GetDesignTemplateResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetDesignTemplateRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetDesignTemplateResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_design_template")))
    }
    async fn upsert_design_template(&self, ctx: RequestContext, request: ServiceRequest<'_, UpsertDesignTemplateRequest>) -> ServiceResult<UpsertDesignTemplateResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpsertDesignTemplateRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpsertDesignTemplateResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.upsert_design_template")))
    }
}

pub fn mount<R>(connect: &mut ConnectRouter, _api: &'static Api, router: &'static R)
where
    R: ApiRoutes + Send + Sync + 'static,
{
    connect.add_service(Arc::new(DesignServiceImpl { sender: ApiSender::new(router) }));
}
