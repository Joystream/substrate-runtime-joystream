use codec::Decode;
use joystream_node_runtime::{
    common::storage::StorageObjectOwner, data_directory::*, ChannelId, ContentId, DAOId,
    DataDirectoryConfig, MemberId, Runtime,
};
use serde::Deserialize;
use std::{fs, path::Path};

// Because of the way that the @joystream/types were implemented the getters for
// the string types return a `string` not the `Text` type so when we are serializing
// them to json we get a string rather than an array of bytes, so deserializing them
// is failing. So we are relying on parity codec encoding instead..
#[derive(Decode)]
struct Content {
    content_id: ContentId,
    data_object: DataObject<Runtime>,
    storage_object_owner: StorageObjectOwner<MemberId, ChannelId, DAOId>,
    voucher: Voucher,
}

#[derive(Decode)]
struct ContentData {
    /// DataObject(s) and ContentId
    data_objects: Vec<Content>,
    voucher_size_limit_upper_bound: u64,
    voucher_objects_limit_upper_bound: u64,
    global_voucher: Voucher,
    default_voucher: Voucher,
    uploading_blocked: bool,
}

#[derive(Deserialize)]
struct EncodedContent {
    /// hex encoded ContentId
    content_id: String,
    /// hex encoded DataObject<Runtime>
    data_object: String,
    /// hex encoded StorageObjectOwner
    storage_object_owner: String,
    /// hex encoded Voucher
    voucher: String,
}

impl EncodedContent {
    fn decode(&self) -> Content {
        // hex string must not include '0x' prefix!
        let encoded_content_id = hex::decode(&self.content_id[2..].as_bytes())
            .expect("failed to parse content_id hex string");
        let encoded_data_object = hex::decode(&self.data_object[2..].as_bytes())
            .expect("failed to parse data_object hex string");
        let encoded_storage_object_owner = hex::decode(&self.storage_object_owner[2..].as_bytes())
            .expect("failed to parse content_id hex string");
        let encoded_voucher = hex::decode(&self.voucher[2..].as_bytes())
            .expect("failed to parse data_object hex string");
        Content {
            content_id: Decode::decode(&mut encoded_content_id.as_slice()).unwrap(),
            data_object: Decode::decode(&mut encoded_data_object.as_slice()).unwrap(),
            storage_object_owner: Decode::decode(&mut encoded_storage_object_owner.as_slice())
                .unwrap(),
            voucher: Decode::decode(&mut encoded_voucher.as_slice()).unwrap(),
        }
    }
}

#[derive(Deserialize)]
struct EncodedContentData {
    /// EncodedContent
    data_objects: Vec<EncodedContent>,
    /// hex encoded VoucherSizeLimitUpperBound
    voucher_size_limit_upper_bound: String,
    /// hex encoded VoucherObjectsLimitUpperBound
    voucher_objects_limit_upper_bound: String,
    /// hex encoded GlobalVoucher
    global_voucher: String,
    /// hex encoded DefaultVoucher
    default_voucher: String,
    /// hex encoded UploadingBlocked flag
    uploading_blocked: String,
}

fn parse_content_data(data_file: &Path) -> EncodedContentData {
    let data = fs::read_to_string(data_file).expect("Failed reading file");
    serde_json::from_str(&data).expect("failed parsing content data")
}

impl EncodedContentData {
    pub fn decode(&self) -> ContentData {
        ContentData {
            data_objects: self
                .data_objects
                .iter()
                .map(|data_objects| data_objects.decode())
                .collect(),
            voucher_size_limit_upper_bound: {
                let encoded_voucher_size_limit_upper_bound =
                    hex::decode(&self.voucher_size_limit_upper_bound[2..].as_bytes())
                        .expect("failed to parse data_object hex string");

                Decode::decode(&mut encoded_voucher_size_limit_upper_bound.as_slice()).unwrap()
            },
            voucher_objects_limit_upper_bound: {
                let encoded_voucher_objects_limit_upper_bound =
                    hex::decode(&self.voucher_objects_limit_upper_bound[2..].as_bytes())
                        .expect("failed to parse data_object hex string");

                Decode::decode(&mut encoded_voucher_objects_limit_upper_bound.as_slice()).unwrap()
            },
            global_voucher: {
                let encoded_global_voucher = hex::decode(&self.global_voucher[2..].as_bytes())
                    .expect("failed to parse data_object hex string");

                Decode::decode(&mut encoded_global_voucher.as_slice()).unwrap()
            },
            default_voucher: {
                let encoded_default_voucher = hex::decode(&self.default_voucher[2..].as_bytes())
                    .expect("failed to parse data_object hex string");

                Decode::decode(&mut encoded_default_voucher.as_slice()).unwrap()
            },
            uploading_blocked: {
                let encoded_uploading_blocked =
                    hex::decode(&self.uploading_blocked[2..].as_bytes())
                        .expect("failed to parse data_object hex string");

                Decode::decode(&mut encoded_uploading_blocked.as_slice()).unwrap()
            },
        }
    }
}

/// Generates a basic empty `DataDirectoryConfig` genesis config
pub fn empty_data_directory_config() -> DataDirectoryConfig {
    DataDirectoryConfig {
        data_object_by_content_id: vec![],
        vouchers: vec![],
        voucher_size_limit_upper_bound: DEFAULT_VOUCHER_SIZE_LIMIT_UPPER_BOUND,
        voucher_objects_limit_upper_bound: DEFAULT_VOUCHER_OBJECTS_LIMIT_UPPER_BOUND,
        global_voucher: DEFAULT_GLOBAL_VOUCHER,
        default_voucher: DEFAULT_VOUCHER,
        uploading_blocked: DEFAULT_UPLOADING_BLOCKED_STATUS,
    }
}

/// Generates a `DataDirectoryConfig` genesis config
/// pre-populated with data objects and known content ids parsed from
/// a json file serialized as a `ContentData` struct
pub fn data_directory_config_from_json(data_file: &Path) -> DataDirectoryConfig {
    let content = parse_content_data(data_file).decode();

    DataDirectoryConfig {
        data_object_by_content_id: content
            .data_objects
            .iter()
            .map(|object| (object.content_id, object.data_object.clone()))
            .collect(),
        vouchers: content
            .data_objects
            .iter()
            .map(|object| (object.storage_object_owner.clone(), object.voucher))
            .collect(),
        voucher_size_limit_upper_bound: content.voucher_size_limit_upper_bound,
        voucher_objects_limit_upper_bound: content.voucher_objects_limit_upper_bound,
        global_voucher: content.global_voucher,
        default_voucher: content.default_voucher,
        uploading_blocked: content.uploading_blocked,
    }
}
