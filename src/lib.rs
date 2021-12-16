pub mod managers;
pub mod services;
pub mod token;

pub mod api {
    pub mod catalog {
        tonic::include_proto!("catalog");
        pub const CATALOG_FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("catalog_descriptor");
    }

    pub mod idp {
        tonic::include_proto!("idp");
        pub const IDP_FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("idp_descriptor");
    }
}
