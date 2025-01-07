use torrent_title_parser::parse_title;

#[test]
fn test_region_detection() {
    let test_cases = vec![
        ("Welcome to New York 2014 R5 XviD AC3-SUPERFAST", Some("R5"), true),
        ("[Coalgirls]_Code_Geass_R2_06_(1920x1080_Blu-ray_FLAC)_[F8C7FE25].mkv", None, false),
    ];

    for (release_name, expected_region, should_have_region) in test_cases {
        let result = parse_title(release_name).unwrap();
        if should_have_region {
            assert_eq!(
                result.region.as_deref(),
                expected_region,
                "Expected region to be {:?} for {}",
                expected_region,
                release_name
            );
        } else {
            assert!(result.region.is_none(), "Region should not be detected in {}", release_name);
        }
    }
}
