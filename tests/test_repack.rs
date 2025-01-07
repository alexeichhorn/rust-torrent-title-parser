use torrent_title_parser::parse_title;

#[test]
fn test_repack_detection() {
    let test_cases = vec![
        ("Silicon Valley S04E03 REPACK HDTV x264-SVA", true),
        ("Expedition Unknown S03E14 Corsicas Nazi Treasure RERIP 720p HDTV x264-W4F", true),
        ("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", false),
    ];

    for (release_name, expected_repack) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(
            result.repack, expected_repack,
            "Expected 'repack' detection to be {} for {}",
            expected_repack, release_name
        );
    }
}
