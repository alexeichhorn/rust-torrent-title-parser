use torrent_title_parser::parse_title;

#[test]
fn test_extras_detection() {
    let test_cases = vec![
        (
            "Madame Web 2024 1080p WEBRip 1400MB DD 5.1 x264 Sample-GalaxyRG[TGx]",
            Some(vec!["Sample"]),
        ),
        ("Madame Web Sample 2024 1080p WEBRip 1400MB DD 5.1 x264-GalaxyRG[TGx]", None),
        (
            "Madame Web Sample 1080p WEBRip 1400MB DD 5.1 x264-GalaxyRG[TGx]",
            Some(vec!["Sample"]),
        ),
        (
            "AVATAR.Featurette.Creating.the.World.of.Pandora.1080p.H264.ITA.AC3.ENGAAC.PappaMux.mkv",
            Some(vec!["Featurette"]),
        ),
    ];

    for (input, expected_extras) in test_cases {
        let result = parse_title(input).unwrap();
        match expected_extras {
            Some(extras) => {
                let expected = extras.clone();
                assert_eq!(
                    result.extras, extras,
                    "Incorrect extras detection for {}: Got {:?}, expected {:?}",
                    input, result.extras, expected
                );
            }
            None => assert!(
                result.extras.is_empty(),
                "Incorrectly detected extras for {}: Got {:?}",
                input,
                result.extras
            ),
        }
    }
}
