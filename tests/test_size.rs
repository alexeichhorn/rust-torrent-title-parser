use torrent_title_parser::parse_title;

#[test]
fn test_size_detection() {
    let test_cases = vec![
        ("www.1TamilBlasters.lat - Thuritham (2023) [Tamil - 2K QHD AVC UNTOUCHED - x264 - AAC - 3.4GB - ESub].mkv", Some("3.4GB")),
        ("www.1TamilMV.world - Raja Vikramarka (2024) Tamil HQ HDRip - 400MB - x264 - AAC - ESub.mkv", Some("400MB")),
        ("www.1TamilMV.cz - Maharaja (2024) TRUE WEB-DL - 1080p HQ - AVC - (DD+5.1 - 640Kbps) [Tam + Tel + Hin + Mal + Kan] - 8.4GB - ESub.mkv", Some("8.4GB")),
        ("The.Walking.Dead.S06E07.SUBFRENCH.HDTV.x264-AMB3R.mkv", None),
    ];

    for (release_name, expected_size) in test_cases {
        let result = parse_title(release_name).unwrap();
        match expected_size {
            Some(size) => assert_eq!(result.size.as_deref(), Some(size), "Incorrect size detected for {}", release_name),
            None => assert!(result.size.is_none(), "Incorrectly detected size for {}", release_name),
        }
    }
}
