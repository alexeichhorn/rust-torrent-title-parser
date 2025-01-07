use torrent_title_parser::parse_title;

#[test]
fn test_proper_detection() {
    let test_cases = vec![
        ("Into the Badlands S02E07 PROPER 720p HDTV x264-W4F", true),
        ("Bossi-Reality-REAL PROPER-CDM-FLAC-1999-MAHOU", true),
        ("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", false),
    ];

    for (release_name, expected_proper) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(
            result.proper, expected_proper,
            "Expected 'proper' detection to be {} for {}",
            expected_proper, release_name
        );
    }
}
