use torrent_title_parser::parse_title;

#[test]
fn test_unrated_detection() {
    let test_cases = vec![
        ("Identity.Thief.2013.Vostfr.UNRATED.BluRay.720p.DTS.x264-Nenuko", true),
        (
            "Charlie.les.filles.lui.disent.merci.2007.UNCENSORED.TRUEFRENCH.DVDRiP.AC3.Libe",
            true,
        ),
        ("Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL", false),
    ];

    for (release_name, expected_unrated) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(
            result.unrated, expected_unrated,
            "Expected 'unrated' detection to be {} for {}",
            expected_unrated, release_name
        );
    }
}
