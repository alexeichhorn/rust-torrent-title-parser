use torrent_title_parser::parse_title;

#[test]
fn test_language_detection() {
    let test_cases = vec![
        ("Test.File.FRENCH", vec!["french"]),
        ("Test.File.MULTI.FRENCH", vec!["french"]),
        ("Test.File.VOSTFR", vec!["french"]),
        ("Test.File.SUBFRENCH", vec!["french"]),
        ("Test.File.GERMAN", vec!["german"]),
        ("Test.File.SPANISH", vec!["spanish"]),
        ("Test.File.ITALIAN", vec!["italian"]),
        ("Test.File.RUSSIAN", vec!["russian"]),
        ("Test.File.JAPANESE", vec!["japanese"]),
        ("Test.File.KOREAN", vec!["korean"]),
        ("Test.File.CHINESE", vec!["chinese"]),
        ("Test.File.MULTI.FRENCH.GERMAN", vec!["french", "german"]),
    ];

    for (input, expected) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.languages, expected);
    }
}

#[test]
fn test_subbed_dubbed() {
    let test_cases = vec![
        ("Test.File.SUBBED", true, false),
        ("Test.File.DUBBED", false, true),
        ("Test.File.VOSTFR", true, false),
        ("Test.File.DUB", false, true),
        ("Test.File.SUB", true, false),
    ];

    for (input, expected_subbed, expected_dubbed) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.subbed, expected_subbed);
        assert_eq!(result.dubbed, expected_dubbed);
    }
}
