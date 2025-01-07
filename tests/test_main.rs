use torrent_title_parser::parse_title;

#[test]
fn test_resolution() {
    let test_cases = vec![
        ("Test.File.1080p", "1080p"),
        ("Test.File.720p", "720p"),
        ("Test.File.2160p", "2160p"),
        ("Test.File.4k", "2160p"),
    ];

    for (input, expected) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.resolution.as_deref(), Some(expected));
    }
}

#[test]
fn test_quality() {
    let test_cases = vec![
        ("Test.File.BluRay", "bluray"),
        ("Test.File.HDTV", "hdtv"),
        ("Test.File.WEB-DL", "webdl"),
        ("Test.File.BDRip", "bdrip"),
    ];

    for (input, expected) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.quality.as_deref(), Some(expected));
    }
}

#[test]
fn test_codec() {
    let test_cases = vec![
        ("Test.File.x264", "x264"),
        ("Test.File.h264", "h264"),
        ("Test.File.x265", "x265"),
        ("Test.File.HEVC", "hevc"),
    ];

    for (input, expected) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.codec.as_deref(), Some(expected));
    }
}

#[test]
fn test_audio() {
    let test_cases = vec![
        ("Test.File.AAC", vec!["aac"]),
        ("Test.File.DTS", vec!["dts"]),
        ("Test.File.AC3", vec!["ac3"]),
        ("Test.File.DTS.AC3", vec!["dts", "ac3"]),
    ];

    for (input, expected) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.audio, expected);
    }
}

#[test]
fn test_season_episode() {
    let test_cases = vec![
        ("Show.S01E01", (vec![1], vec![1])),
        ("Show.S1E1", (vec![1], vec![1])),
        ("Show.S01E01E02", (vec![1], vec![1, 2])),
        ("Show.S01.E01", (vec![1], vec![1])),
    ];

    for (input, (expected_seasons, expected_episodes)) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(result.seasons, expected_seasons);
        assert_eq!(result.episodes, expected_episodes);
    }
}

#[test]
fn test_complete_titles() {
    let test_cases = vec![
        (
            "The.Simpsons.S01E01.1080p.BluRay.x265.HEVC.10bit.AAC.5.1.Tigole",
            "The Simpsons",
            Some("1080p"),
            Some("bluray"),
            Some("x265"),
            vec!["aac"],
            vec!["5.1"],
            vec![1],
            vec![1],
            Some("Tigole"),
        ),
        (
            "Breaking.Bad.S01E01.720p.HDTV.x264-DIMENSION",
            "Breaking Bad",
            Some("720p"),
            Some("hdtv"),
            Some("x264"),
            vec![],
            vec![],
            vec![1],
            vec![1],
            Some("DIMENSION"),
        ),
    ];

    for (
        input,
        expected_title,
        expected_resolution,
        expected_quality,
        expected_codec,
        expected_audio,
        expected_channels,
        expected_seasons,
        expected_episodes,
        expected_group,
    ) in test_cases
    {
        let result = parse_title(input).unwrap();
        assert_eq!(result.title, expected_title);
        assert_eq!(result.resolution.as_deref(), expected_resolution);
        assert_eq!(result.quality.as_deref(), expected_quality);
        assert_eq!(result.codec.as_deref(), expected_codec);
        assert_eq!(result.audio, expected_audio);
        assert_eq!(result.channels, expected_channels);
        assert_eq!(result.seasons, expected_seasons);
        assert_eq!(result.episodes, expected_episodes);
        assert_eq!(result.group.as_deref(), expected_group);
    }
}
