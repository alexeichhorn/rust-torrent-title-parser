use torrent_title_parser::parse_title;

#[test]
fn test_special_flags() {
    let test_cases = vec![
        ("Test.File.EXTENDED", "extended"),
        ("Test.File.REMASTERED", "remastered"),
        ("Test.File.PROPER", "proper"),
        ("Test.File.REPACK", "repack"),
        ("Test.File.RETAIL", "retail"),
        ("Test.File.UNRATED", "unrated"),
        ("Test.File.COMPLETE", "complete"),
    ];

    for (input, flag) in test_cases {
        let result = parse_title(input).unwrap();
        assert!(match flag {
            "extended" => result.extended,
            "remastered" => result.remastered,
            "proper" => result.proper,
            "repack" => result.repack,
            "retail" => result.retail,
            "unrated" => result.unrated,
            "complete" => result.complete,
            _ => false,
        });
    }
}

#[test]
fn test_edge_cases() {
    let test_cases = vec![
        // Empty title
        ("", ""),
        // Only metadata
        ("1080p.BluRay.x264", ""),
        // Multiple dots and spaces
        ("The...Show....S01E01", "The Show"),
        // Mixed case
        ("ThE.ShOw.S01E01", "ThE ShOw"),
        // Special characters
        ("The_Show_S01E01", "The Show"),
    ];

    for (input, expected_title) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.title, expected_title);
    }
}

#[test]
fn test_complex_titles() {
    let test_cases = vec![
        (
            "[HorribleSubs] My Hero Academia - S04E01 [1080p].mkv",
            "My Hero Academia",
            vec![4],
            vec![1],
        ),
        (
            "The Walking Dead S10E01-E02 PROPER 1080p AMZN WEB-DL DDP5.1 x264-NTb",
            "The Walking Dead",
            vec![10],
            vec![1, 2],
        ),
        (
            "Planet.Earth.II.S01.COMPLETE.UHD.BLURAY-COASTER",
            "Planet Earth II",
            vec![1],
            vec![],
        ),
    ];

    for (input, expected_title, expected_seasons, expected_episodes) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.title, expected_title);
        assert_eq!(result.seasons, expected_seasons);
        assert_eq!(result.episodes, expected_episodes);
    }
}
