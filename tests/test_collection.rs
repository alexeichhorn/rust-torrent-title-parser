use torrent_title_parser::parse_title;

#[test]
fn test_complete_collection_detection() {
    let test_cases = vec![
        ("[Furi] Avatar - The Last Airbender [720p] (Full 3 Seasons + Extr", true, None),
        ("Harry.Potter.Complete.Collection.2001-2011.1080p.BluRay.DTS-ETRG", true, None),
        ("Game of Thrones All 7 Seasons 1080p ~∞~ .HakunaMaKoko", true, None),
        (
            "Avatar: The Last Airbender Full Series 720p",
            true,
            Some("Avatar: The Last Airbender"),
        ),
        ("Dora the Explorer - Ultimate Collection", true, None),
        ("Mr Bean Complete Pack (Animated, Tv series, 2 Movies) DVDRIP (WA", true, None),
        ("American Pie - Complete set (8 movies) 720p mkv - YIFY", true, None),
        ("Charlie Chaplin - Complete Filmography (87 movies)", true, None),
        ("Monster High Movies Complete 2014", true, None),
        ("Harry Potter All Movies Collection 2001-2011 720p Dual KartiKing", true, None),
        ("The Clint Eastwood Movie Collection", true, None),
        ("Clint Eastwood Collection - 15 HD Movies", true, None),
        ("Official  IMDb  Top  250  Movies  Collection  6/17/2011", true, None),
        ("The Texas Chainsaw Massacre Collection (1974-2017) BDRip 1080p", true, None),
        ("Snabba.Cash.I-II.Duology.2010-2012.1080p.BluRay.x264.anoXmous", true, None),
        ("Star Wars Original Trilogy 1977-1983 Despecialized 720p", true, None),
        ("The.Wong.Kar-Wai.Quadrology.1990-2004.1080p.BluRay.x264.AAC.5.1-", true, None),
        ("Lethal.Weapon.Quadrilogy.1987-1992.1080p.BluRay.x264.anoXmous", true, None),
        ("X-Men.Tetralogy.BRRip.XviD.AC3.RoSubbed-playXD", true, None),
        ("Mission.Impossible.Pentalogy.1996-2015.1080p.BluRay.x264.AAC.5.1", true, None),
        ("Mission.Impossible.Hexalogy.1996-2018.SweSub.1080p.x264-Justiso", true, None),
        ("American.Pie.Heptalogy.SWESUB.DVDRip.XviD-BaZZe", true, Some("American Pie")),
        ("The Exorcist 1, 2, 3, 4, 5 - Complete Horror Anthology 1973-2005", true, None),
        ("Harry.Potter.Complete.Saga. I - VIII .1080p.Bluray.x264.anoXmous", true, None),
        (
            "[Erai-raws] Ninja Collection - 05 [720p][Multiple Subtitle].mkv",
            true,
            Some("Ninja Collection"),
        ),
        (
            "Furiosa - A Mad Max Saga (2024) 2160p H265 HDR10 D V iTA EnG AC3 5 1 Sub iTA EnG NUiTA NUEnG AsPiDe-MIRCrew mkv",
            true,
            Some("Furiosa - A Mad Max Saga"),
        ),
        (
            "[Judas] Vinland Saga (Season 2) [1080p][HEVC x265 10bit][Multi-Subs]",
            true,
            Some("Vinland Saga"),
        ),
    ];

    for (input, expected_complete, expected_title) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(
            result.complete, expected_complete,
            "Incorrect 'complete' detection for {}: Got {:?}, expected {:?}",
            input, result.complete, expected_complete
        );
        if let Some(title) = expected_title {
            assert_eq!(
                result.title, title,
                "Incorrect title detected for {}: Got {:?}, expected {:?}",
                input, result.title, title
            );
        }
    }
}
