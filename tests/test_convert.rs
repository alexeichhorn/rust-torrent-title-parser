use torrent_title_parser::parse_title;

#[test]
fn test_convert_detection() {
    let test_cases = vec![
        ("Better.Call.Saul.S03E04.CONVERT.720p.WEB.h264-TBS", true),
        ("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", false),
    ];

    for (input, expected_convert) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(
            result.convert, expected_convert,
            "Incorrect convert detection for {}: Got {:?}, expected {:?}",
            input, result.convert, expected_convert
        );
    }
}
