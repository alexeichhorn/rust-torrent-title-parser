use torrent_title_parser::{parse_title, Codec, ParsedTitle, Quality};

#[test]
fn test_random_sports_parse() {
    let test_cases = vec![
        (
            "UFC.239.PPV.Jones.Vs.Santos.HDTV.x264-PUNCH[TGx]",
            ParsedTitle {
                title: "UFC 239 Jones Vs Santos".to_string(),
                quality: Some(Quality::HDTV),
                codec: Some(Codec::Avc),
                group: Some("PUNCH".to_string()),
                ppv: true,
                ..Default::default()
            },
        ),
        (
            "UFC.Fight.Night.158.Cowboy.vs.Gaethje.WEB.x264-PUNCH[TGx]",
            ParsedTitle {
                title: "UFC Fight Night 158 Cowboy vs Gaethje".to_string(),
                quality: Some(Quality::Web),
                codec: Some(Codec::Avc),
                group: Some("PUNCH".to_string()),
                ppv: true,
                ..Default::default()
            },
        ),
        (
            "UFC 226 PPV Miocic vs Cormier HDTV x264-Ebi [TJET]",
            ParsedTitle {
                title: "UFC 226 Miocic vs Cormier".to_string(),
                quality: Some(Quality::HDTV),
                codec: Some(Codec::Avc),
                ppv: true,
                ..Default::default()
            },
        ),
    ];

    for (release_name, expected) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(result, expected, "Failed for {}", release_name);
    }
}
