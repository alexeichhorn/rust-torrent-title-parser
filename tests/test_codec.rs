use torrent_title_parser::{parse_title, Codec};

#[test]
fn test_codec_detection() {
    let test_cases = vec![
        ("Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2", Some(Codec::Hevc), None),
        ("doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov", Some(Codec::Avc), None),
        (
            "The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN",
            Some(Codec::Avc),
            None,
        ),
        ("Gotham S03E17 XviD-AFG", Some(Codec::Xvid), None),
        ("Jimmy Kimmel 2017 05 03 720p HDTV DD5 1 MPEG2-CTL", Some(Codec::Mpeg), None),
        (
            "[Anime Time] Re Zero kara Hajimeru Isekai Seikatsu (Season 2 Part 1) [1080p][HEVC10bit x265][Multi Sub]",
            Some(Codec::Hevc),
            Some("10bit"),
        ),
        (
            "[naiyas] Fate Stay Night - Unlimited Blade Works Movie [BD 1080P HEVC10 QAACx2 Dual Audio]",
            Some(Codec::Hevc),
            Some("10bit"),
        ),
        ("[DB]_Bleach_264_[012073FE].avi", None, None),
        ("[DB]_Bleach_265_[B4A04EC9].avi", None, None),
        (
            "Mad.Max.Fury.Road.2015.1080p.BluRay.DDP5.1.x265.10bit-GalaxyRG265[TGx]",
            Some(Codec::Hevc),
            Some("10bit"),
        ),
    ];

    for (input, expected_codec, expected_bit_depth) in test_cases {
        let result = parse_title(input).unwrap();
        match expected_codec {
            Some(codec) => {
                assert_eq!(
                    result.codec,
                    Some(codec),
                    "Incorrect codec detected for {}: Got {:?}, expected {:?}",
                    input,
                    result.codec,
                    expected_codec
                );
                if let Some(bit_depth) = expected_bit_depth {
                    assert_eq!(
                        result.bit_depth,
                        Some(bit_depth.to_string()),
                        "Incorrect bit depth detected for {}: Got {:?}, expected {:?}",
                        input,
                        result.bit_depth,
                        expected_bit_depth
                    );
                }
            }
            None => assert!(result.codec.is_none(), "Unexpected codec found: {:?} in {}", result.codec, input),
        }
    }
}
