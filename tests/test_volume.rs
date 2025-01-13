use torrent_title_parser::parse_title;

#[test]
fn test_volume_detection() {
    let test_cases = vec![
        ("[MTBB] Sword Art Onlineː Alicization - Volume 2 (BD 1080p)", (vec![2])),
        (
            "[Neutrinome] Sword Art Online Alicization Vol.2 - VOSTFR [1080p BDRemux] + DDL",
            (vec![2]),
        ),
        ("[Mr. Kimiko] Oh My Goddess! - Vol. 7 [Kobo][2048px][CBZ]", vec![7]),
        ("[MTBB] Cross Game - Volume 1-3 (WEB 720p)", vec![1, 2, 3]),
        ("PIXAR SHORT FILMS COLLECTION - VOLS. 1 & 2 + - BDrip 1080p", vec![1, 2]),
        (
            "Altair - A Record of Battles Vol. 01-08 (Digital) (danke-Empire)",
            (vec![1, 2, 3, 4, 5, 6, 7, 8]),
        ),
        ("Guardians of the Galaxy Vol. 2 (2017) 720p HDTC x264 MKVTV", (vec![])),
        ("Kill Bill: Vol. 1 (2003) BluRay 1080p 5.1CH x264 Ganool", (vec![])),
        ("[Valenciano] Aquarion EVOL - 22 [1080p][AV1 10bit][FLAC][Eng sub].mkv", (vec![])),
    ];

    for (release_name, expected_volumes) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(result.volumes, expected_volumes, "Volumes mismatch for {}", release_name);
    }
}
