use torrent_title_parser::parse_title;

#[test]
fn test_parsed_output() {
    let test_case = "[Golumpa] Fairy Tail - 214 [FuniDub 720p x264 AAC] [5E46AC39]";
    let result = parse_title(test_case).unwrap();
    assert!(result.title.len() > 0);
    assert!(result.episode_code.is_some());
    assert!(result.resolution.is_some());
    assert!(result.codec.is_some());
    assert!(!result.audio.is_empty());
}

#[test]
fn test_basic_parsed() {
    let test_case = "The.Matrix.1999.1080p.BluRay.x264";
    let result = parse_title(test_case).unwrap();
    assert_eq!(result.title, "The Matrix");
    assert_eq!(result.resolution, Some("1080p".to_string()));
    assert_eq!(result.year, Some(1999));
    assert_eq!(result.quality, Some("BluRay".to_string()));
    assert_eq!(result.codec, Some("avc".to_string()));
}

#[test]
fn test_season_parser() {
    let test_cases = vec![
        ("Archer.S02.1080p.BluRay.DTSMA.AVC.Remux", vec![2]),
        ("The Simpsons S01E01 1080p BluRay x265 HEVC 10bit AAC 5.1 Tigole", vec![1]),
        ("[F-D] Fairy Tail Season 1 - 6 + Extras [480P][Dual-Audio]", vec![1, 2, 3, 4, 5, 6]),
        ("House MD All Seasons (1-8) 720p Ultra-Compressed", vec![1, 2, 3, 4, 5, 6, 7, 8]),
        ("Bleach 10º Temporada - 215 ao 220 - [DB-BR]", vec![10]),
        ("Lost.[Perdidos].6x05.HDTV.XviD.[www.DivxTotaL.com]", vec![6]),
        ("4-13 Cursed (HD)", vec![4]),
        ("Dragon Ball Z Movie - 09 - Bojack Unbound - 1080p BluRay x264 DTS 5.1 -DDR", vec![]), // Correct. This should not match, its a movie.
        ("BoJack Horseman [06x01-08 of 16] (2019-2020) WEB-DLRip 720p", vec![6]),
        ("[HR] Boku no Hero Academia 87 (S4-24) [1080p HEVC Multi-Subs] HR-GZ", vec![4]),
        ("The Simpsons S28E21 720p HDTV x264-AVS", vec![28]),
    ];

    for (test_case, expected) in test_cases {
        let result = parse_title(test_case).unwrap();
        assert_eq!(result.seasons, expected, "Failed for {}", test_case);
    }
}

#[test]
fn test_episode_code() {
    let test_case = "[Golumpa] Fairy Tail - 214 [FuniDub 720p x264 AAC] [5E46AC39]";
    let result = parse_title(test_case).unwrap();
    assert_eq!(result.episode_code, Some("5E46AC39".to_string()));
}
