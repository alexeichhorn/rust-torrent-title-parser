use torrent_title_parser::parse_title;

#[test]
fn test_hardcoded_detection() {
    let test_cases = vec![
        ("Ghost In The Shell 2017 1080p HC HDRip X264 AC3-EVO", true),
        ("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", false),
    ];

    for (input, expected_hardcoded) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(
            result.hardcoded, expected_hardcoded,
            "Incorrect hardcoded detection for {}: Got {:?}, expected {:?}",
            input, result.hardcoded, expected_hardcoded
        );
    }
}
