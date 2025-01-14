use torrent_title_parser::{parse_title, Network};

#[test]
fn test_network_detection() {
    let test_cases = vec![
        ("Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2", None, "Nocturnal Animals"),
        ("doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov", None, "doctor who"),
        (
            "The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN",
            Some(Network::AnimalPlanet),
            "The Vet Life",
        ),
        ("Gotham S03E17 XviD-AFG", None, "Gotham"),
        ("Jimmy Kimmel 2017 05 03 720p HDTV DD5 1 MPEG2-CTL", None, "Jimmy Kimmel"),
        (
            "[Anime Time] Re Zero kara Hajimeru Isekai Seikatsu (Season 2 Part 1) [1080p][HEVC10bit x265][Multi Sub]",
            None,
            "Re Zero kara Hajimeru Isekai Seikatsu",
        ),
        (
            "[naiyas] Fate Stay Night - Unlimited Blade Works Movie [BD 1080P HEVC10 QAACx2 Dual Audio]",
            None,
            "Fate Stay Night - Unlimited Blade Works Movie",
        ),
        (
            "Extraction.2020.720p.NF.WEB-DL.Dual.Atmos.5.1.x264-BonsaiHD",
            Some(Network::Netflix),
            "Extraction",
        ),
        (
            "Guilty (2020) NF Original 720p WEBRip [Hindi + English] AAC DD-5.1 ESub x264 - Shadow.mkv",
            Some(Network::Netflix),
            "Guilty",
        ),
        (
            "The.Bear.S03.COMPLETE.1080p.HULU.WEB.H264-SuccessfulCrab[TGx]",
            Some(Network::Hulu),
            "The Bear",
        ),
        (
            "Futurama.S08E03.How.the.West.Was.1010001.1080p.HULU.WEB-DL.DDP5.1.H.264-FLUX.mkv",
            Some(Network::Hulu),
            "Futurama",
        ),
        (
            "Amazon.Queen.2021.720p.AMZN.WEBRip.800MB.x264-GalaxyRG",
            Some(Network::Amazon),
            "Amazon Queen",
        ),
        (
            "The.Mummy.2017.1080p.AMZN.WEBRip.DD5.1.H.264-GalaxyRG",
            Some(Network::Amazon),
            "The Mummy",
        ),
    ];

    for (release_name, expected_network, expected_title) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(result.network, expected_network, "Failed network detection for {}", release_name);
        assert_eq!(result.title, expected_title, "Failed title detection for {}", release_name);
    }
}
