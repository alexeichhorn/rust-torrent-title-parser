use torrent_title_parser::parse_title;

#[test]
fn test_retail_detection() {
    let test_cases = vec![
        ("MONSTER HIGH: ELECTRIFIED (2017) Retail PAL DVD9 [EAGLE]", true),
        ("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", false),
    ];

    for (release_name, expected_retail) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(
            result.retail, expected_retail,
            "Expected 'retail' detection to be {} for {}",
            expected_retail, release_name
        );
    }
}
