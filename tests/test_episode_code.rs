use torrent_title_parser::parse_title;

#[test]
fn test_episode_code_detection() {
    let test_cases = vec![
        (
            "[Golumpa] Fairy Tail - 214 [FuniDub 720p x264 AAC] [5E46AC39].mkv",
            Some("5E46AC39"),
        ),
        ("[Exiled-Destiny]_Tokyo_Underground_Ep02v2_(41858470).mkv", Some("41858470")),
        (
            "[ACX]El_Cazador_de_la_Bruja_-_19_-_A_Man_Who_Protects_[SSJ_Saiyan_Elite]_[9E199846].mkv",
            Some("9E199846"),
        ),
        ("[CBM]_Medaka_Box_-_11_-_This_Is_the_End!!_[720p]_[436E0E90]", Some("436E0E90")),
        (
            "Gankutsuou.-.The.Count.Of.Monte.Cristo[2005].-.04.-.[720p.BD.HEVC.x265].[FLAC].[Jd].[DHD].[b6e6e648].mkv",
            Some("B6E6E648"),
        ),
        (
            "[D0ugyB0y] Nanatsu no Taizai Fundo no Shinpan - 01 (1080p WEB NF x264 AAC[9CC04E06]).mkv",
            Some("9CC04E06"),
        ),
        // Negative test case
        ("Lost.[Perdidos].6x05.HDTV.XviD.[www.DivxTotaL.com].avi", None),
    ];

    for (input, expected_episode_code) in test_cases {
        let result = parse_title(input).unwrap();
        match expected_episode_code {
            Some(code) => {
                assert_eq!(
                    result.episode_code.as_deref(),
                    Some(code),
                    "Incorrect episode code detected for {}: Got {:?}, expected {:?}",
                    input,
                    result.episode_code,
                    expected_episode_code
                );
            }
            None => assert!(
                result.episode_code.is_none(),
                "Incorrectly detected episode code for {}: Got {:?}",
                input,
                result.episode_code
            ),
        }
    }
}
