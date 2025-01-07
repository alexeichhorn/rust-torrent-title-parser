use torrent_title_parser::parse_title;

#[test]
fn test_site_detection() {
    let test_cases = vec![
        ("The.Expanse.S05E02.1080p.AMZN.WEB.DDP5.1.x264-NTb[eztv.re].mp4", Some("eztv.re")),
        (
            "www.1TamilBlasters.lat - Thuritham (2023) [Tamil - 2K QHD AVC UNTOUCHED - x264 - AAC - 3.4GB - ESub].mkv",
            Some("www.1TamilBlasters.lat"),
        ),
        (
            "www.1TamilMV.world - Raja Vikramarka (2024) Tamil HQ HDRip - 400MB - x264 - AAC - ESub.mkv",
            Some("www.1TamilMV.world"),
        ),
        (
            "Anatomia De Grey - Temporada 19 [HDTV][Cap.1905][Castellano][www.AtomoHD.nu].avi",
            Some("www.AtomoHD.nu"),
        ),
        (
            "[HD-ELITE.NET] -  The.Art.Of.The.Steal.2014.DVDRip.XviD.Dual.Aud",
            Some("HD-ELITE.NET"),
        ),
        (
            "[ Torrent9.cz ] The.InBetween.S01E10.FiNAL.HDTV.XviD-EXTREME.avi",
            Some("Torrent9.cz"),
        ),
        (
            "Jurassic.World.Dominion.CUSTOM.EXTENDED.2022.2160p.MULTi.VF2.UHD.Blu-ray.REMUX.HDR.DoVi.HEVC.DTS-X.DTS-HDHRA.7.1-MOONLY.mkv",
            None,
        ),
        ("Last.Call.for.Istanbul.2023.1080p.NF.WEB-DL.DDP5.1.H.264.MKV.torrent", None),
    ];

    for (release_name, expected_site) in test_cases {
        let result = parse_title(release_name).unwrap();
        match expected_site {
            Some(site) => assert_eq!(result.site.as_deref(), Some(site), "Incorrect site detected for {}", release_name),
            None => assert!(result.site.is_none(), "Incorrectly detected site for {}", release_name),
        }
    }
}
