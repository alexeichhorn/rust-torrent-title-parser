use torrent_title_parser::parse_title;

#[test]
fn test_dubbed_detection() {
    let test_cases = vec![
        ("Yo-Kai Watch S01E71 DUBBED 720p HDTV x264-W4F", true),
        (
            "[Golumpa] Kochoki - 11 (Kochoki - Wakaki Nobunaga) [English Dub] [FuniDub 720p x264 AAC] [MKV] [4FA0D898]",
            true,
        ),
        ("[Aomori-Raws] Juushinki Pandora (01-13) [Dubs & Subs]", true),
        ("[LostYears] Tsuredure Children (WEB 720p Hi10 AAC) [Dual-Audio]", true),
        ("[DB] Gamers! [Dual Audio 10bit 720p][HEVC-x265]", true),
        ("[DragsterPS] Yu-Gi-Oh! S02 [480p] [Multi-Audio] [Multi-Subs]", true),
        ("A Freira (2018) Dublado HD-TS 720p", true),
        ("Toy.Story.1080p.BluRay.x264-HD[Dubbing PL].mkv", true),
        ("Fame (1980) [DVDRip][Dual][Ac3][Eng-Spa]", true),
        (
            "[Hakata Ramen] Hoshiai No Sora (Stars Align) 01 [1080p][HEVC][x265][10bit][Dual-Subs] HR-DR",
            false,
        ),
        ("[IceBlue] Naruto (Season 01) - [Multi-Dub][Multi-Sub][HEVC 10Bits] 800p BD", true),
    ];

    for (input, expected_dubbed) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(
            result.dubbed, expected_dubbed,
            "Incorrect dubbed detection for {}: Got {:?}, expected {:?}",
            input, result.dubbed, expected_dubbed
        );
    }
}
