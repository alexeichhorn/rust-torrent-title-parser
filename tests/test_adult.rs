use torrent_title_parser::parse_title;

#[test]
fn test_adult_content_detection() {
    let test_cases = vec![
        (
            "Wicked 24 02 23 Liz Jordan And Xxlayna Marie Phantasia XXX 1080p HEVC x265 PRT",
            true,
            "Wicked",
        ),
        (
            "Wicked.24.11.01.Liz.Jordan.It.Didnt.Have.To.End.This.Way.XXX.1080p.HEVC.x265.PRT.mp4",
            true,
            "Wicked",
        ),
        (
            "The.Sopranos.S04E01.For.All.Debts.Public.and.Private.480p.WEB-DL.x264-Sticky83.mkv",
            false,
            "The Sopranos",
        ),
    ];

    for (input, expected_adult, expected_title) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(
            result.adult, expected_adult,
            "Got {} instead of {} for title: {}",
            result.adult, expected_adult, input
        );
        assert_eq!(
            result.title, expected_title,
            "Got '{}' instead of '{}' for title: {}",
            result.title, expected_title, input
        );
    }
}
