use codec::Decode;
use node_runtime::common::constraints::InputValidationLengthConstraint;
use node_runtime::{
    content_wg::{Channel, ChannelId, Principal, PrincipalId},
    primitives::{AccountId, BlockNumber, Credential},
    versioned_store::{Class, ClassId, Entity, EntityId},
    versioned_store_permissions::ClassPermissions,
    ContentWorkingGroupConfig, Runtime, VersionedStoreConfig, VersionedStorePermissionsConfig,
};
use serde::Deserialize;
use std::{fs, path::Path};

// Because of the way that the @joystream/types were implemented the getters for
// the string types return a `string` not the `Text` type so when we are serializing
// them to json we get a string rather than an array of bytes, so deserializing them
// is failing. So we are relying on parity codec encoding instead..
#[derive(Decode)]
struct ClassAndPermissions {
    class: Class,
    permissions: ClassPermissions<ClassId, Credential, u16, BlockNumber>,
}

#[derive(Decode)]
struct EntityAndMaintainer {
    entity: Entity,
    maintainer: Option<Credential>,
}

#[derive(Decode)]
struct ContentData {
    /// classes and their associted permissions
    classes: Vec<ClassAndPermissions>,
    /// entities and their associated maintainer
    entities: Vec<EntityAndMaintainer>,
    /// Media Channels
    channels: Vec<ChannelAndId>,
}

#[derive(Deserialize)]
struct EncodedClassAndPermissions {
    /// hex encoded Class
    class: String,
    /// hex encoded ClassPermissions<ClassId, Credential, u16, BlockNumber>,
    permissions: String,
}

impl EncodedClassAndPermissions {
    fn decode(&self) -> ClassAndPermissions {
        // hex string must not include '0x' prefix!
        let encoded_class =
            hex::decode(&self.class[2..].as_bytes()).expect("failed to parse class hex string");
        let encoded_permissions = hex::decode(&self.permissions[2..].as_bytes())
            .expect("failed to parse class permissions hex string");
        ClassAndPermissions {
            class: Decode::decode(&mut encoded_class.as_slice()).unwrap(),
            permissions: Decode::decode(&mut encoded_permissions.as_slice()).unwrap(),
        }
    }
}

#[derive(Deserialize)]
struct EncodedEntityAndMaintainer {
    /// hex encoded Entity
    entity: String,
    /// hex encoded Option<Credential>
    maintainer: Option<String>,
}

impl EncodedEntityAndMaintainer {
    fn decode(&self) -> EntityAndMaintainer {
        // hex string must not include '0x' prefix!
        let encoded_entity =
            hex::decode(&self.entity[2..].as_bytes()).expect("failed to parse entity hex string");
        let encoded_maintainer = self.maintainer.as_ref().map(|maintainer| {
            hex::decode(&maintainer[2..].as_bytes()).expect("failed to parse maintainer hex string")
        });
        EntityAndMaintainer {
            entity: Decode::decode(&mut encoded_entity.as_slice()).unwrap(),
            maintainer: encoded_maintainer
                .map(|maintainer| Decode::decode(&mut maintainer.as_slice()).unwrap()),
        }
    }
}

#[derive(Decode)]
struct ChannelAndId {
    id: ChannelId<Runtime>,
    channel: Channel<u64, AccountId, BlockNumber, PrincipalId<Runtime>>,
}

#[derive(Deserialize)]
struct EncodedChannelAndId {
    /// ChannelId number
    id: u64,
    /// hex encoded Channel
    channel: String,
}

impl EncodedChannelAndId {
    fn decode(&self) -> ChannelAndId {
        let id = self.id;
        let encoded_channel =
            hex::decode(&self.channel[2..].as_bytes()).expect("failed to parse channel hex string");
        ChannelAndId {
            id: id as ChannelId<Runtime>,
            channel: Decode::decode(&mut encoded_channel.as_slice()).unwrap(),
        }
    }
}

#[derive(Deserialize)]
struct EncodedContentData {
    /// classes and their associted permissions
    classes: Vec<EncodedClassAndPermissions>,
    /// entities and their associated maintainer
    entities: Vec<EncodedEntityAndMaintainer>,
    /// Media Channels
    channels: Vec<EncodedChannelAndId>,
}

fn parse_content_data(data_file: &Path) -> EncodedContentData {
    let data = fs::read_to_string(data_file).expect("Failed reading file");
    serde_json::from_str(&data).expect("failed parsing content data")
}

impl EncodedContentData {
    pub fn decode(&self) -> ContentData {
        ContentData {
            classes: self
                .classes
                .iter()
                .map(|class_and_perm| class_and_perm.decode())
                .collect(),
            entities: self
                .entities
                .iter()
                .map(|entities_and_maintainer| entities_and_maintainer.decode())
                .collect(),
            channels: self
                .channels
                .iter()
                .map(|channel_and_id| channel_and_id.decode())
                .collect(),
        }
    }
}

/// Generates a VersionedStoreConfig genesis config
/// with pre-populated classes and entities parsed from a json file serialized
/// as a ContentData struct.
pub fn versioned_store_config_from_json(data_file: &Path) -> VersionedStoreConfig {
    let content = parse_content_data(data_file).decode();
    let base_config = empty_versioned_store_config();
    let first_id = 1;

    let next_class_id: ClassId = content
        .classes
        .last()
        .map_or(first_id, |class_and_perm| class_and_perm.class.id + 1);
    assert_eq!(next_class_id, (content.classes.len() + 1) as ClassId);

    let next_entity_id: EntityId = content
        .entities
        .last()
        .map_or(first_id, |entity_and_maintainer| {
            entity_and_maintainer.entity.id + 1
        });

    VersionedStoreConfig {
        class_by_id: content
            .classes
            .into_iter()
            .map(|class_and_permissions| {
                (class_and_permissions.class.id, class_and_permissions.class)
            })
            .collect(),
        entity_by_id: content
            .entities
            .into_iter()
            .map(|entity_and_maintainer| {
                (
                    entity_and_maintainer.entity.id,
                    entity_and_maintainer.entity,
                )
            })
            .collect(),
        next_class_id,
        next_entity_id,
        ..base_config
    }
}

/// Generates basic empty VersionedStoreConfig genesis config
pub fn empty_versioned_store_config() -> VersionedStoreConfig {
    VersionedStoreConfig {
        class_by_id: vec![],
        entity_by_id: vec![],
        next_class_id: 1,
        next_entity_id: 1,
        property_name_constraint: InputValidationLengthConstraint::new(1, 99),
        property_description_constraint: InputValidationLengthConstraint::new(1, 999),
        class_name_constraint: InputValidationLengthConstraint::new(1, 99),
        class_description_constraint: InputValidationLengthConstraint::new(1, 999),
    }
}

/// Generates a basic empty VersionedStorePermissionsConfig genesis config
pub fn empty_versioned_store_permissions_config() -> VersionedStorePermissionsConfig {
    VersionedStorePermissionsConfig {
        class_permissions_by_class_id: vec![],
        entity_maintainer_by_entity_id: vec![],
    }
}

/// Generates a `VersionedStorePermissionsConfig` genesis config
/// pre-populated with permissions and entity maintainers parsed from
/// a json file serialized as a `ContentData` struct.
pub fn versioned_store_permissions_config_from_json(
    data_file: &Path,
) -> VersionedStorePermissionsConfig {
    let content = parse_content_data(data_file).decode();

    VersionedStorePermissionsConfig {
        class_permissions_by_class_id: content
            .classes
            .into_iter()
            .map(|class_and_perm| (class_and_perm.class.id, class_and_perm.permissions))
            .collect(),
        entity_maintainer_by_entity_id: content
            .entities
            .into_iter()
            .filter_map(|entity_and_maintainer| {
                entity_and_maintainer
                    .maintainer
                    .map(|maintainer| (entity_and_maintainer.entity.id, maintainer))
            })
            .collect(),
    }
}

/// Generates a basic `ContentWorkingGroupConfig` genesis config without any active curators
/// curator lead or channels.
pub fn empty_content_working_group_config() -> ContentWorkingGroupConfig {
    ContentWorkingGroupConfig {
        mint_capacity: 0,
        curator_opening_by_id: vec![],
        next_curator_opening_id: 0,
        curator_application_by_id: vec![],
        next_curator_application_id: 0,
        channel_by_id: vec![],
        next_channel_id: 1,
        channel_id_by_handle: vec![],
        curator_by_id: vec![],
        next_curator_id: 0,
        principal_by_id: vec![],
        next_principal_id: 0,
        channel_creation_enabled: true, // there is no extrinsic to change it so enabling at genesis
        unstaker_by_stake_id: vec![],
        channel_handle_constraint: InputValidationLengthConstraint::new(5, 20),
        channel_description_constraint: InputValidationLengthConstraint::new(1, 1024),
        opening_human_readable_text: InputValidationLengthConstraint::new(1, 2048),
        curator_application_human_readable_text: InputValidationLengthConstraint::new(1, 2048),
        curator_exit_rationale_text: InputValidationLengthConstraint::new(1, 2048),
        channel_avatar_constraint: InputValidationLengthConstraint::new(5, 1024),
        channel_banner_constraint: InputValidationLengthConstraint::new(5, 1024),
        channel_title_constraint: InputValidationLengthConstraint::new(5, 1024),
    }
}

/// Generates a `ContentWorkingGroupConfig` genesis config
/// pre-populated with channels and corresponding princial channel owners
/// parsed from a json file serialized as a `ContentData` struct
pub fn content_working_group_config_from_json(data_file: &Path) -> ContentWorkingGroupConfig {
    let content = parse_content_data(data_file).decode();
    let first_channel_id = 1;
    let first_principal_id = 0;

    let next_channel_id: ChannelId<Runtime> = content
        .channels
        .last()
        .map_or(first_channel_id, |channel_and_id| channel_and_id.id + 1);
    assert_eq!(
        next_channel_id,
        (content.channels.len() + 1) as ChannelId<Runtime>
    );

    let base_config = empty_content_working_group_config();

    ContentWorkingGroupConfig {
        channel_by_id: content
            .channels
            .iter()
            .enumerate()
            .map(|(ix, channel_and_id)| {
                (
                    channel_and_id.id,
                    Channel {
                        principal_id: first_principal_id + ix as PrincipalId<Runtime>,
                        ..channel_and_id.channel.clone()
                    },
                )
            })
            .collect(),
        next_channel_id,
        channel_id_by_handle: content
            .channels
            .iter()
            .map(|channel_and_id| (channel_and_id.channel.handle.clone(), channel_and_id.id))
            .collect(),
        principal_by_id: content
            .channels
            .iter()
            .enumerate()
            .map(|(ix, channel_and_id)| {
                (
                    first_principal_id + ix as PrincipalId<Runtime>,
                    Principal::ChannelOwner(channel_and_id.id),
                )
            })
            .collect(),
        next_principal_id: first_principal_id + content.channels.len() as PrincipalId<Runtime>,
        ..base_config
    }
}
