use torrent_title_parser::parse_title;

#[test]
fn test_trash_detection() {
    let test_cases = vec![
        ("(Hi10)_Re_Zero_Shin_Henshuu-ban_-_02v2_(720p)_(DDY)_(72006E34).mkv", false),
        (
            "Anatomia De Grey - Temporada 19 [HDTV][Cap.1905][Castellano][www.AtomoHD.nu].avi",
            false,
        ),
        ("[SubsPlease] Fairy Tail - 100 Years Quest - 05 (1080p) [1107F3A9].mkv", false),
        ("Body.Cam.S08E07.1080p.WEB.h264-EDITH[EZTVx.to].mkv", false),
        ("Body Cam (2020) [1080p] [WEBRip] [5.1] [YTS] [YIFY]", false),
        ("Avengers Infinity War 2018 NEW PROPER 720p HD-CAM X264 HQ-CPG", true),
        ("Venom: Let There Be Carnage (2021) English 720p CAMRip [NO LOGO]", true),
        ("Oppenheimer (2023) NEW ENG 1080p HQ-CAM x264 AAC - HushRips", true),
        ("Hatyapuri 2022 1080p CAMRp Bengali AAC H264 [2GB] - HDWebMovies", true),
        (
            "Avengers: Infinity War (2018) 720p HQ New CAMRip Line Audios [Tamil + Telugu + Hindi + Eng] x264 1.2GB [Team TR]",
            true,
        ),
        ("Brave.2012.R5.DVDRip.XViD.LiNE-UNiQUE", true),
        ("Guardians of the Galaxy (CamRip / 2014)", true),
        ("Guardians of the Galaxy (2014) 1080p BluRay 5.1 DTS-HD MA 7.1 [YTS] [YIFY]", false),
        ("抓娃娃 Successor.2024.TC1080P.国语中字", true),
    ];

    for (release_name, expected_trash) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(result.trash, expected_trash, "Failed for {}", release_name);
    }
}
