use statusforge_backend::shared::utils;

#[test]
fn test_uuid_v4_generates_valid_uuid() {
    let uuid = utils::uuid_v4();
    assert!(uuid::Uuid::parse_str(&uuid).is_ok());
}

#[test]
fn test_uuid_v4_generates_unique_uuids() {
    let uuid1 = utils::uuid_v4();
    let uuid2 = utils::uuid_v4();
    assert_ne!(uuid1, uuid2);
}

#[test]
fn test_uuid_v4_format() {
    let uuid = utils::uuid_v4();
    assert_eq!(uuid.len(), 36);
    assert!(uuid.contains('-'));
}
