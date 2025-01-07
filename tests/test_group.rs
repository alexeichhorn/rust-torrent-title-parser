use torrent_title_parser::parse_title;

#[test]
fn test_group_detection() {
    let test_cases = vec![
        ("Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2", Some("HD2")),
        ("Gold 2016 1080p BluRay DTS-HD MA 5 1 x264-HDH", Some("HDH")),
        ("Hercules (2014) 1080p BrRip H264 - YIFY", Some("YIFY")),
        ("The.Expanse.S05E02.720p.WEB.x264-Worldmkv.mkv", Some("Worldmkv")),
        ("The.Expanse.S05E02.PROPER.720p.WEB.h264-KOGi[rartv]", Some("KOGi")),
        ("The.Expanse.S05E02.1080p.AMZN.WEB.DDP5.1.x264-NTb[eztv.re].mp4", Some("NTb")),
        ("Western - L'homme qui n'a pas d'étoile-1955.Multi.DVD9", None),
        ("Power (2014) - S02E03.mp4", None),
        ("Power (2014) - S02E03", None),
        ("3-Nen D-Gumi Glass no Kamen - 13", None),
        ("3-Nen D-Gumi Glass no Kamen - Ep13", None),
        ("[AnimeRG] One Punch Man - 09 [720p].mkv", Some("AnimeRG")),
        ("[Mazui]_Hyouka_-_03_[DF5E813A].mkv", Some("Mazui")),
        ("[H3] Hunter x Hunter - 38 [1280x720] [x264]", Some("H3")),
        ("[KNK E MMS Fansubs] Nisekoi - 20 Final [PT-BR].mkv", Some("KNK E MMS Fansubs")),
        (
            "[ToonsHub] JUJUTSU KAISEN - S02E01 (Japanese 2160p x264 AAC) [Multi-Subs].mkv",
            Some("ToonsHub"),
        ),
        ("[HD-ELITE.NET] -  The.Art.Of.The.Steal.2014.DVDRip.XviD.Dual.Aud", None),
        ("[Russ]Lords.Of.London.2014.XviD.H264.AC3-BladeBDP", Some("BladeBDP")),
        (
            "Jujutsu Kaisen S02E01 2160p WEB H.265 AAC -Tsundere-Raws (B-Global).mkv",
            Some("B-Global"),
        ),
        ("[DVD-RIP] Kaavalan (2011) Sruthi XVID [700Mb] [TCHellRaiser]", None),
        ("the-x-files-502.mkv", None),
        ("[ Torrent9.cz ] The.InBetween.S01E10.FiNAL.HDTV.XviD-EXTREME.avi", Some("EXTREME")),
    ];

    for (input, expected_group) in test_cases {
        let result = parse_title(input).unwrap();
        match expected_group {
            Some(group) => {
                assert_eq!(
                    result.group.as_deref(),
                    Some(group),
                    "Incorrect group detected for {}: Got {:?}, expected {:?}",
                    input,
                    result.group,
                    expected_group
                );
            }
            None => assert!(
                result.group.is_none(),
                "Incorrectly detected group for {}: Got {:?}",
                input,
                result.group
            ),
        }
    }
}
