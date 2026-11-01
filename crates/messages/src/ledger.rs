//! The ledger's object types, as the producers send them and the ledger
//! books them (the Directive §11). A snapshot is the WHOLE object as its
//! producer knows it: its identity, a strictly increasing version, its
//! accounting state and whatever the accounting rules read (amounts, the
//! period, the counterparty). Rendered from `owns.ledger` in
//! `docs/01-enterprise-merch/manifest.yaml`; the TODO fields are yours to add,
//! the framework fields are not yours to remove or rename (a rename changes
//! the stored form; see `docs/frameworks/ledger.md`).

use basable_ledger::{LedgerObject, ObjectId, ProcessId};
use serde::{Deserialize, Serialize};

/// A booking: any mix of snapshots, across processes. The objects of one
/// process book atomically, in the order sent, or not at all; the outcomes
/// come back per snapshot, in the same order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub snapshots: Vec<LedgerSnapshot>,
}

/// One outcome per snapshot sent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booked {
    pub outcomes: Vec<BookingOutcome>,
}

/// What happened to one snapshot: `basable_ledger::Outcome` as a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookingOutcome {
    /// Booked in this call.
    Booked,
    /// A snapshot at the version already booked: nothing to do.
    AlreadyBooked,
    /// A snapshot older than the version booked: dropped.
    Stale,
    /// Rejected, with the reason; every snapshot of the same process in the
    /// call is rejected with it.
    Rejected(String),
}

impl From<basable_ledger::Outcome> for BookingOutcome {
    fn from(o: basable_ledger::Outcome) -> Self {
        match o {
            basable_ledger::Outcome::Booked => Self::Booked,
            basable_ledger::Outcome::AlreadyBooked => Self::AlreadyBooked,
            basable_ledger::Outcome::Stale => Self::Stale,
            basable_ledger::Outcome::Rejected(r) => Self::Rejected(r.message().to_owned()),
        }
    }
}

/// Every object type the ledger books.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedgerSnapshot {
    Invoice(InvoiceSnapshot),
    CreditNote(CreditNoteSnapshot),
    Payment(PaymentSnapshot),
    VatReturn(VatReturnSnapshot),
    SupplierBill(SupplierBillSnapshot),
}

/// The accounting states of `invoice`, as `billing` reports them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceState {
    Issued,
}

/// A snapshot of `invoice`, sent by `billing` and booked by the ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSnapshot {
    /// The business process the object belongs to: a root object's own id.
    /// The objects of one process book atomically.
    pub process_id: ProcessId,
    /// The object's id, unique within the process.
    pub id: ObjectId,
    /// The parent within the same process, if any. Never changes once set.
    pub parent_id: Option<ObjectId>,
    /// Strictly increasing per object: a higher version books, an equal one
    /// is a duplicate, a lower one is stale. The producer's status carries
    /// it and bumps it in the same write that changes anything the rules
    /// read (a state, an amount).
    pub version: u64,
    /// The accounting state.
    pub state: InvoiceState,
    // TODO: the fields the accounting rules read (amounts in minor units as
    // `i128` with `#[serde(with = "basable_ledger::amount_string")]`, the
    // currency, the period, the counterparty). A desired state is a pure
    // function of these.
}

impl LedgerObject for InvoiceSnapshot {
    const TYPE: &'static str = "invoice";
    type State = InvoiceState;

    fn process_id(&self) -> ProcessId {
        self.process_id
    }

    fn object_id(&self) -> ObjectId {
        self.id
    }

    fn parent_id(&self) -> Option<ObjectId> {
        self.parent_id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn state(&self) -> InvoiceState {
        self.state
    }
}

/// The accounting states of `credit_note`, as `billing` reports them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteState {
    Issued,
}

/// A snapshot of `credit_note`, sent by `billing` and booked by the ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNoteSnapshot {
    /// The business process the object belongs to: a root object's own id.
    /// The objects of one process book atomically.
    pub process_id: ProcessId,
    /// The object's id, unique within the process.
    pub id: ObjectId,
    /// The parent within the same process, if any. Never changes once set.
    pub parent_id: Option<ObjectId>,
    /// Strictly increasing per object: a higher version books, an equal one
    /// is a duplicate, a lower one is stale. The producer's status carries
    /// it and bumps it in the same write that changes anything the rules
    /// read (a state, an amount).
    pub version: u64,
    /// The accounting state.
    pub state: CreditNoteState,
    // TODO: the fields the accounting rules read (amounts in minor units as
    // `i128` with `#[serde(with = "basable_ledger::amount_string")]`, the
    // currency, the period, the counterparty). A desired state is a pure
    // function of these.
}

impl LedgerObject for CreditNoteSnapshot {
    const TYPE: &'static str = "credit_note";
    type State = CreditNoteState;

    fn process_id(&self) -> ProcessId {
        self.process_id
    }

    fn object_id(&self) -> ObjectId {
        self.id
    }

    fn parent_id(&self) -> Option<ObjectId> {
        self.parent_id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn state(&self) -> CreditNoteState {
        self.state
    }
}

/// The accounting states of `payment`, as `billing` reports them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentState {
    Pending,
    Settled,
    Reversed,
}

/// A snapshot of `payment`, sent by `billing` and booked by the ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSnapshot {
    /// The business process the object belongs to: a root object's own id.
    /// The objects of one process book atomically.
    pub process_id: ProcessId,
    /// The object's id, unique within the process.
    pub id: ObjectId,
    /// The parent within the same process, if any. Never changes once set.
    pub parent_id: Option<ObjectId>,
    /// Strictly increasing per object: a higher version books, an equal one
    /// is a duplicate, a lower one is stale. The producer's status carries
    /// it and bumps it in the same write that changes anything the rules
    /// read (a state, an amount).
    pub version: u64,
    /// The accounting state.
    pub state: PaymentState,
    // TODO: the fields the accounting rules read (amounts in minor units as
    // `i128` with `#[serde(with = "basable_ledger::amount_string")]`, the
    // currency, the period, the counterparty). A desired state is a pure
    // function of these.
}

impl LedgerObject for PaymentSnapshot {
    const TYPE: &'static str = "payment";
    type State = PaymentState;

    fn process_id(&self) -> ProcessId {
        self.process_id
    }

    fn object_id(&self) -> ObjectId {
        self.id
    }

    fn parent_id(&self) -> Option<ObjectId> {
        self.parent_id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn state(&self) -> PaymentState {
        self.state
    }
}

/// The accounting states of `vat_return`, as `billing` reports them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VatReturnState {
    Filed,
}

/// A snapshot of `vat_return`, sent by `billing` and booked by the ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatReturnSnapshot {
    /// The business process the object belongs to: a root object's own id.
    /// The objects of one process book atomically.
    pub process_id: ProcessId,
    /// The object's id, unique within the process.
    pub id: ObjectId,
    /// The parent within the same process, if any. Never changes once set.
    pub parent_id: Option<ObjectId>,
    /// Strictly increasing per object: a higher version books, an equal one
    /// is a duplicate, a lower one is stale. The producer's status carries
    /// it and bumps it in the same write that changes anything the rules
    /// read (a state, an amount).
    pub version: u64,
    /// The accounting state.
    pub state: VatReturnState,
    // TODO: the fields the accounting rules read (amounts in minor units as
    // `i128` with `#[serde(with = "basable_ledger::amount_string")]`, the
    // currency, the period, the counterparty). A desired state is a pure
    // function of these.
}

impl LedgerObject for VatReturnSnapshot {
    const TYPE: &'static str = "vat_return";
    type State = VatReturnState;

    fn process_id(&self) -> ProcessId {
        self.process_id
    }

    fn object_id(&self) -> ObjectId {
        self.id
    }

    fn parent_id(&self) -> Option<ObjectId> {
        self.parent_id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn state(&self) -> VatReturnState {
        self.state
    }
}

/// The accounting states of `supplier_bill`, as `billing` reports them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupplierBillState {
    Approved,
    Voided,
}

/// A snapshot of `supplier_bill`, sent by `billing` and booked by the ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierBillSnapshot {
    /// The business process the object belongs to: a root object's own id.
    /// The objects of one process book atomically.
    pub process_id: ProcessId,
    /// The object's id, unique within the process.
    pub id: ObjectId,
    /// The parent within the same process, if any. Never changes once set.
    pub parent_id: Option<ObjectId>,
    /// Strictly increasing per object: a higher version books, an equal one
    /// is a duplicate, a lower one is stale. The producer's status carries
    /// it and bumps it in the same write that changes anything the rules
    /// read (a state, an amount).
    pub version: u64,
    /// The accounting state.
    pub state: SupplierBillState,
    // TODO: the fields the accounting rules read (amounts in minor units as
    // `i128` with `#[serde(with = "basable_ledger::amount_string")]`, the
    // currency, the period, the counterparty). A desired state is a pure
    // function of these.
}

impl LedgerObject for SupplierBillSnapshot {
    const TYPE: &'static str = "supplier_bill";
    type State = SupplierBillState;

    fn process_id(&self) -> ProcessId {
        self.process_id
    }

    fn object_id(&self) -> ObjectId {
        self.id
    }

    fn parent_id(&self) -> Option<ObjectId> {
        self.parent_id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn state(&self) -> SupplierBillState {
        self.state
    }
}
