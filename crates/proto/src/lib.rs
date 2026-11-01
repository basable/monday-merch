//! Generated protobuf types (buffa, under `proto`) and Connect service stubs
//! (under `connect`), mounted from the build's output tree one module pair
//! per proto package. Edit `proto/**/*.proto`, never these modules. The
//! scaffolder adds a nanoservice's package between the markers.

#![allow(missing_docs, clippy::all)]

pub mod proto {
    pub mod health {
        pub mod v1 {
            include!("../generated/health_v1/buffa/health.v1.rs");
        }
    }
    pub mod basable {
        pub mod config {
            pub mod v1 {
                include!("../generated/basable_config_v1/buffa/basable.config.v1.rs");
            }
        }
    }
    // basable:proto-modules-begin
    pub mod account {
        pub mod v1 {
            include!("../generated/account_v1/buffa/account.v1.rs");
        }
    }
    pub mod catalog {
        pub mod v1 {
            include!("../generated/catalog_v1/buffa/catalog.v1.rs");
        }
    }
    pub mod design {
        pub mod v1 {
            include!("../generated/design_v1/buffa/design.v1.rs");
        }
    }
    pub mod order {
        pub mod v1 {
            include!("../generated/order_v1/buffa/order.v1.rs");
        }
    }
    pub mod fulfilment {
        pub mod v1 {
            include!("../generated/fulfilment_v1/buffa/fulfilment.v1.rs");
        }
    }
    pub mod billing {
        pub mod v1 {
            include!("../generated/billing_v1/buffa/billing.v1.rs");
        }
    }
    pub mod notifier {
        pub mod v1 {
            include!("../generated/notifier_v1/buffa/notifier.v1.rs");
        }
    }
    pub mod inbox {
        pub mod v1 {
            include!("../generated/inbox_v1/buffa/inbox.v1.rs");
        }
    }
    // basable:proto-modules-end
}

pub mod connect {
    pub mod health {
        pub mod v1 {
            include!("../generated/health_v1/connect/health.v1.rs");
        }
    }
    // basable:connect-modules-begin
    pub mod account {
        pub mod v1 {
            include!("../generated/account_v1/connect/account.v1.rs");
        }
    }
    pub mod catalog {
        pub mod v1 {
            include!("../generated/catalog_v1/connect/catalog.v1.rs");
        }
    }
    pub mod design {
        pub mod v1 {
            include!("../generated/design_v1/connect/design.v1.rs");
        }
    }
    pub mod order {
        pub mod v1 {
            include!("../generated/order_v1/connect/order.v1.rs");
        }
    }
    pub mod fulfilment {
        pub mod v1 {
            include!("../generated/fulfilment_v1/connect/fulfilment.v1.rs");
        }
    }
    pub mod billing {
        pub mod v1 {
            include!("../generated/billing_v1/connect/billing.v1.rs");
        }
    }
    pub mod inbox {
        pub mod v1 {
            include!("../generated/inbox_v1/connect/inbox.v1.rs");
        }
    }
    // basable:connect-modules-end
}
