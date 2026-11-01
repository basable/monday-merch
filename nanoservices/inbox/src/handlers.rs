//! The handler trait implementation: one method per message this
//! nanoservice handles (routing.yaml). Handlers are the INTENT boundary:
//! they validate, write desired state through the typed store (`create`,
//! `update_spec`, `mark_deleted`, `nudge`) or the repository, dispatch
//! request-path adapters `Unfenced`, and send through `s` (the sender that
//! exposes exactly the declared sends). They never write status.

use basable_core::{AppError, Ctx};
use interfaces::{InboxHandler, InboxRoutes, InboxSender};

use crate::Inbox;

// Messages stay qualified: a message may share its name with this
// component (an `order` nanoservice handling an `Order` response).
impl<R: InboxRoutes> InboxHandler<R> for Inbox {
    async fn handle_list_inbound_messages_request(&self, _ctx: &Ctx, _msg: messages::ListInboundMessagesRequest, _s: InboxSender<'_, R>) -> Result<messages::InboundMessageList, AppError> {
        Err(crate::unimplemented_step("inbox.handle_list_inbound_messages_request"))
    }
    async fn handle_list_review_queue_request(&self, _ctx: &Ctx, _msg: messages::ListReviewQueueRequest, _s: InboxSender<'_, R>) -> Result<messages::InboundDocumentList, AppError> {
        Err(crate::unimplemented_step("inbox.handle_list_review_queue_request"))
    }
    async fn handle_get_inbound_document_request(&self, _ctx: &Ctx, _msg: messages::GetInboundDocumentRequest, _s: InboxSender<'_, R>) -> Result<messages::InboundDocument, AppError> {
        Err(crate::unimplemented_step("inbox.handle_get_inbound_document_request"))
    }
    async fn handle_get_inbound_file_link_request(&self, _ctx: &Ctx, _msg: messages::GetInboundFileLinkRequest, _s: InboxSender<'_, R>) -> Result<messages::DownloadLink, AppError> {
        Err(crate::unimplemented_step("inbox.handle_get_inbound_file_link_request"))
    }
    async fn handle_decide_inbound_document_request(&self, _ctx: &Ctx, _msg: messages::DecideInboundDocumentRequest, _s: InboxSender<'_, R>) -> Result<messages::InboundDocument, AppError> {
        Err(crate::unimplemented_step("inbox.handle_decide_inbound_document_request"))
    }
    async fn handle_inbound_email_event(&self, _ctx: &Ctx, _msg: messages::InboundEmailEvent, _s: InboxSender<'_, R>) -> Result<messages::WebhookAck, AppError> {
        Err(crate::unimplemented_step("inbox.handle_inbound_email_event"))
    }
}
