use torrent_title_parser::parse_title;

#[test]
fn test_container_detection() {
    let test_cases = vec![
        ("Kevin Hart What Now (2016) 1080p BluRay x265 6ch -Dtech mkv", "mkv"),
        ("The Gorburger Show S01E05 AAC MP4-Mobile", "mp4"),
        ("[req]Night of the Lepus (1972) DVDRip XviD avi", "avi"),
    ];

    for (input, expected_container) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(
            result.container.as_deref(),
            Some(expected_container),
            "Incorrect container detected for {}: Got {:?}, expected {:?}",
            input,
            result.container,
            expected_container
        );
    }
}
