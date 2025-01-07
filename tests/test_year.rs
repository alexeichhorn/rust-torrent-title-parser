use torrent_title_parser::parse_title;

#[test]
fn test_year_detection() {
    let test_cases = vec![
        ("Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO", Some(2014)),
        ("Hercules (2014) 1080p BrRip H264 - YIFY", Some(2014)),
        ("One Shot [2014] DVDRip XViD-ViCKY", Some(2014)),
        ("2012 2009 1080p BluRay x264 REPACK-METiS", Some(2009)),
        ("2008 The Incredible Hulk Feature Film.mp4", Some(2008)),
        ("Harry Potter All Movies Collection 2001-2011 720p Dual KartiKing", None),
        ("Empty Nest Season 1 (1988 - 89) fiveofseven", None),
        ("04. Practice Two (1324mb 1916x1080 50fps 1970kbps x265 deef).mkv", None),
        (
            "Anatomia De Grey - Temporada 19 [HDTV][Cap.1905][Castellano][www.AtomoHD.nu].avi",
            None,
        ),
        (
            "Wonder Woman 1984 (2020) [UHDRemux 2160p DoVi P8 Es-DTSHD AC3 En-AC3].mkv",
            Some(2020),
        ),
    ];

    for (release_name, expected_year) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(result.year, expected_year, "Failed for {}", release_name);
    }
}
