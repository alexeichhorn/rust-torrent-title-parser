use torrent_title_parser::parse_title;

#[test]
fn test_hdr_detection() {
    let test_cases = vec![
        ("The.Mandalorian.S01E06.4K.HDR.2160p 4.42GB", vec!["HDR"]),
        (
            "Spider-Man - Complete Movie Collection (2002-2022) 1080p.HEVC.HDR10.1920x800.x265. DTS-HD",
            vec!["HDR"],
        ),
        (
            "Bullet.Train.2022.2160p.AMZN.WEB-DL.x265.10bit.HDR10Plus.DDP5.1-SMURF",
            vec!["HDR10+"],
        ),
        ("Belle (2021) 2160p 10bit 4KLight DOLBY VISION BluRay DDP 7.1 x265-QTZ", vec!["DV"]),
        (
            "Андор / Andor [01x01-03 из 12] (2022) WEB-DL-HEVC 2160p | 4K | Dolby Vision TV | NewComers, HDRezka Studio",
            vec!["DV"],
        ),
        ("АBullet.Train.2022.2160p.WEB-DL.DDP5.1.DV.MKV.x265-NOGRP", vec!["DV"]),
        ("Bullet.Train.2022.2160p.WEB-DL.DoVi.DD5.1.HEVC-EVO[TGx]", vec!["DV"]),
        (
            "Спайдерхед / Spiderhead (2022) WEB-DL-HEVC 2160p | 4K | HDR | Dolby Vision Profile 8 | P | NewComers, Jaskier",
            vec!["DV", "HDR"],
        ),
        (
            "House.of.the.Dragon.S01E07.2160p.10bit.HDR.DV.WEBRip.6CH.x265.HEVC-PSA",
            vec!["DV", "HDR"],
        ),
        (
            "Флешбэк / Memory (2022) WEB-DL-HEVC 2160p | 4K | HDR | HDR10+ | Dolby Vision Profile 8 | Pazl Voice",
            vec!["DV", "HDR10+", "HDR"],
        ),
    ];

    for (input, expected_hdr) in test_cases {
        let result = parse_title(input).unwrap();
        assert_eq!(
            result.hdr, expected_hdr,
            "Incorrect HDR tags detected for {}: Got {:?}, expected {:?}",
            input, result.hdr, expected_hdr
        );
    }
}
