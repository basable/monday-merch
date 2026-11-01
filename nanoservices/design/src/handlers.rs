//! The handler trait implementation: one method per message this
//! nanoservice handles (routing.yaml). Handlers are the INTENT boundary:
//! they validate, write desired state through the typed store (`create`,
//! `update_spec`, `mark_deleted`, `nudge`) or the repository, dispatch
//! request-path adapters `Unfenced`, and send through `s` (the sender that
//! exposes exactly the declared sends). They never write status.

use basable_core::{AppError, Ctx};
use interfaces::{DesignHandler, DesignRoutes, DesignSender};

use crate::Design;

// Messages stay qualified: a message may share its name with this
// component (an `order` nanoservice handling an `Order` response).
impl<R: DesignRoutes> DesignHandler<R> for Design {
    async fn handle_get_brand_profile_request(&self, _ctx: &Ctx, _msg: messages::GetBrandProfileRequest, _s: DesignSender<'_, R>) -> Result<messages::BrandProfile, AppError> {
        Err(crate::unimplemented_step("design.handle_get_brand_profile_request"))
    }
    async fn handle_update_brand_profile_request(&self, _ctx: &Ctx, _msg: messages::UpdateBrandProfileRequest, _s: DesignSender<'_, R>) -> Result<messages::BrandProfile, AppError> {
        Err(crate::unimplemented_step("design.handle_update_brand_profile_request"))
    }
    async fn handle_list_brand_assets_request(&self, _ctx: &Ctx, _msg: messages::ListBrandAssetsRequest, _s: DesignSender<'_, R>) -> Result<messages::BrandAssetList, AppError> {
        Err(crate::unimplemented_step("design.handle_list_brand_assets_request"))
    }
    async fn handle_create_asset_upload_request(&self, _ctx: &Ctx, _msg: messages::CreateAssetUploadRequest, _s: DesignSender<'_, R>) -> Result<messages::AssetUpload, AppError> {
        Err(crate::unimplemented_step("design.handle_create_asset_upload_request"))
    }
    async fn handle_confirm_asset_upload_request(&self, _ctx: &Ctx, _msg: messages::ConfirmAssetUploadRequest, _s: DesignSender<'_, R>) -> Result<messages::BrandAsset, AppError> {
        Err(crate::unimplemented_step("design.handle_confirm_asset_upload_request"))
    }
    async fn handle_delete_brand_asset_request(&self, _ctx: &Ctx, _msg: messages::DeleteBrandAssetRequest, _s: DesignSender<'_, R>) -> Result<messages::BrandAsset, AppError> {
        Err(crate::unimplemented_step("design.handle_delete_brand_asset_request"))
    }
    async fn handle_create_proof_request(&self, _ctx: &Ctx, _msg: messages::CreateProofRequest, _s: DesignSender<'_, R>) -> Result<messages::Proof, AppError> {
        Err(crate::unimplemented_step("design.handle_create_proof_request"))
    }
    async fn handle_list_proofs_request(&self, _ctx: &Ctx, _msg: messages::ListProofsRequest, _s: DesignSender<'_, R>) -> Result<messages::ProofList, AppError> {
        Err(crate::unimplemented_step("design.handle_list_proofs_request"))
    }
    async fn handle_approve_proofs_request(&self, _ctx: &Ctx, _msg: messages::ApproveProofsRequest, _s: DesignSender<'_, R>) -> Result<messages::ProofApproval, AppError> {
        Err(crate::unimplemented_step("design.handle_approve_proofs_request"))
    }
    async fn handle_get_approved_artwork_request(&self, _ctx: &Ctx, _msg: messages::GetApprovedArtworkRequest, _s: DesignSender<'_, R>) -> Result<messages::ArtworkBundle, AppError> {
        Err(crate::unimplemented_step("design.handle_get_approved_artwork_request"))
    }
    async fn handle_create_design_request(&self, _ctx: &Ctx, _msg: messages::CreateDesignRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignDocument, AppError> {
        Err(crate::unimplemented_step("design.handle_create_design_request"))
    }
    async fn handle_get_design_request(&self, _ctx: &Ctx, _msg: messages::GetDesignRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignDocument, AppError> {
        Err(crate::unimplemented_step("design.handle_get_design_request"))
    }
    async fn handle_list_designs_request(&self, _ctx: &Ctx, _msg: messages::ListDesignsRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignList, AppError> {
        Err(crate::unimplemented_step("design.handle_list_designs_request"))
    }
    async fn handle_save_design_draft_request(&self, _ctx: &Ctx, _msg: messages::SaveDesignDraftRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignDocument, AppError> {
        Err(crate::unimplemented_step("design.handle_save_design_draft_request"))
    }
    async fn handle_save_design_version_request(&self, _ctx: &Ctx, _msg: messages::SaveDesignVersionRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignVersion, AppError> {
        Err(crate::unimplemented_step("design.handle_save_design_version_request"))
    }
    async fn handle_archive_design_request(&self, _ctx: &Ctx, _msg: messages::ArchiveDesignRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignDocument, AppError> {
        Err(crate::unimplemented_step("design.handle_archive_design_request"))
    }
    async fn handle_create_preview_upload_request(&self, _ctx: &Ctx, _msg: messages::CreatePreviewUploadRequest, _s: DesignSender<'_, R>) -> Result<messages::PreviewUpload, AppError> {
        Err(crate::unimplemented_step("design.handle_create_preview_upload_request"))
    }
    async fn handle_list_design_templates_request(&self, _ctx: &Ctx, _msg: messages::ListDesignTemplatesRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignTemplateList, AppError> {
        Err(crate::unimplemented_step("design.handle_list_design_templates_request"))
    }
    async fn handle_get_design_template_request(&self, _ctx: &Ctx, _msg: messages::GetDesignTemplateRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignTemplate, AppError> {
        Err(crate::unimplemented_step("design.handle_get_design_template_request"))
    }
    async fn handle_upsert_design_template_request(&self, _ctx: &Ctx, _msg: messages::UpsertDesignTemplateRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignTemplate, AppError> {
        Err(crate::unimplemented_step("design.handle_upsert_design_template_request"))
    }
    async fn handle_resolve_design_version_request(&self, _ctx: &Ctx, _msg: messages::ResolveDesignVersionRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignVersionRef, AppError> {
        Err(crate::unimplemented_step("design.handle_resolve_design_version_request"))
    }
    async fn handle_adopt_designs_request(&self, _ctx: &Ctx, _msg: messages::AdoptDesignsRequest, _s: DesignSender<'_, R>) -> Result<messages::DesignAdoption, AppError> {
        Err(crate::unimplemented_step("design.handle_adopt_designs_request"))
    }
}
