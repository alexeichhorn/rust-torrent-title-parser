use torrent_title_parser::parse_title;

#[test]
fn test_edition_detection() {
    let test_cases = vec![
        (
            "Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL",
            Some("Extended Edition"),
        ),
        (
            "Mary.Poppins.1964.50th.ANNIVERSARY.EDITION.REMUX.1080p.Bluray.AVC.DTS-HD.MA.5.1-LEGi0N",
            Some("Anniversary Edition"),
        ),
        (
            "The.Lord.of.the.Rings.The.Fellowship.of.the.Ring.2001.EXTENDED.2160p.UHD.BluRay.x265.10bit.HDR.TrueHD.7.1.Atmos-BOREDOR",
            Some("Extended Edition"),
        ),
        (
            "The.Lord.of.the.Rings.The.Motion.Picture.Trilogy.Extended.Editions.2001-2003.1080p.BluRay.x264.DTS-WiKi",
            Some("Extended Edition"),
        ),
        ("Better.Call.Saul.S03E04.CONVERT.720p.WEB.h264-TBS", None),
        (
            "The Fifth Element 1997 REMASTERED MULTi 1080p BluRay HDLight AC3 x264 Zone80",
            Some("Remastered"),
        ),
        ("Predator 1987 REMASTER MULTi 1080p BluRay x264 FiDELiO", Some("Remastered")),
        (
            "Have I Got News For You S53E02 EXTENDED 720p HDTV x264-QPEL",
            Some("Extended Edition"),
        ),
    ];

    for (input, expected_edition) in test_cases {
        let result = parse_title(input).unwrap();
        match expected_edition {
            Some(edition) => {
                assert_eq!(
                    result.edition.as_deref(),
                    Some(edition),
                    "Incorrect edition detection for {}: Got {:?}, expected {:?}",
                    input,
                    result.edition,
                    expected_edition
                );
            }
            None => assert!(
                result.edition.is_none(),
                "Incorrectly detected edition for {}: Got {:?}",
                input,
                result.edition
            ),
        }
    }
}
