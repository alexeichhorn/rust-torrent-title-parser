use torrent_title_parser::{parse_title, Quality};

#[test]
fn test_quality_detection() {
    let test_cases = vec![
        ("Nocturnal Animals 2016 VFF 1080p BluRay DTS HEVC-HD2", Quality::BluRay),
        ("doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov", Quality::HDTV),
        ("Rebecca.1940.720p.HDTVRip.HDCLUB", Quality::HDTVRip),
        ("Gossip Girl - 1ª Temporada. (SAT-Rip)", Quality::SATRip),
        ("A Stable Life S01E01 DVDRip x264-Ltu", Quality::DVDRip),
        (
            "The Vet Life S02E01 Dunk-A-Doctor 1080p ANPL WEB-DL AAC2 0 H 264-RTN",
            Quality::WebDL,
        ),
        ("Brown Nation S01E05 1080p WEBRip x264-JAWN", Quality::WebRip),
        ("Star Wars The Last Jedi 2017 TeleSync AAC x264-MiniMe", Quality::TeleSync),
        ("The.Shape.of.Water.2017.DVDScr.XVID.AC3.HQ.Hive-CM8", Quality::SCR),
        (
            "Cloudy With A Chance Of Meatballs 2 2013 720p PPVRip x264 AAC-FooKaS",
            Quality::PPVRip,
        ),
        ("The.OA.1x08.L.Io.Invisibile.ITA.WEBMux.x264-UBi.mkv", Quality::WebMux),
        (
            "[UsifRenegade] Cardcaptor Sakura [BD][Remastered][1080p][HEVC_10Bit][Dual] + Movies",
            Quality::BDRip,
        ),
        (
            "[UsifRenegade] Cardcaptor Sakura - 54 [BD-RM][1080p][x265_10Bit][Dual_AAC].mkv",
            Quality::BDRip,
        ),
        ("Elvis & Nixon (MicroHD-1080p)", Quality::HDRip),
        ("Bohemian Rhapsody 2018.2160p.UHDrip.x265.HDR.DD+.5.1-DTOne", Quality::UHDRip),
        ("Blade.Runner.2049.2017.4K.UltraHD.BluRay.2160p.x264.TrueHD.Atmos", Quality::BluRay),
        ("Terminator.Dark.Fate.2019.2160p.UHD.BluRay.X265.10bit.HDR.TrueHD", Quality::BluRay),
        ("When We Were Boys 2013 BD Rip x264 titohmr", Quality::BDRip),
        ("Key.and.Peele.s03e09.720p.web.dl.mrlss.sujaidr (pimprg)", Quality::WebDL),
        ("Godzilla 2014 HDTS HC XVID AC3 ACAB", Quality::TeleSync),
        (
            "Harry Potter And The Half Blood Prince 2009 telesync aac -- king",
            Quality::TeleSync,
        ),
        ("Capitao.America.2.TS.BrunoG", Quality::TeleSync),
        (
            "Star Trek TS-Screener Spanish Alta-Calidad 2da Version 2009 - Me",
            Quality::TeleSync,
        ),
        (
            "Solo: A Star Wars Story (2018) English 720p TC x264 900MBTEAM TR",
            Quality::TeleCine,
        ),
        ("Alita Battle Angel 2019 720p HDTC-1XBET", Quality::TeleCine),
        ("My.Super.Ex.Girlfriend.FRENCH.TELECINE.XViD-VCDFRV", Quality::TeleCine),
        ("You're Next (2013) cam XVID", Quality::Cam),
        ("Shes the one_2013(camrip)__TOPSIDER [email protected]", Quality::Cam),
        ("Blair Witch 2016 HDCAM UnKnOwN", Quality::Cam),
        ("Thor : Love and Thunder (2022) Hindi HQCAM x264 AAC - QRips.mkv", Quality::Cam),
        (
            "Avatar The Way of Water (2022) 1080p HQ S-Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv",
            Quality::Cam,
        ),
        (
            "Avatar The Way of Water (2022) 1080p S Print Dual Audio [Hindi   English] x264 AAC HC-Esub - CineVood.mkv",
            Quality::Cam,
        ),
        ("Good Deeds 2012 SCR XViD-KiNGDOM", Quality::SCR),
        ("Genova DVD-Screener Spanish 2008", Quality::SCR),
        ("El Albergue Rojo BR-Screener Spanish 2007", Quality::SCR),
        ("The.Mysteries.of.Pittsburgh.LIMITED.SCREENER.XviD-COALiTiON [NOR", Quality::SCR),
        ("El.curioso.caso.de.benjamin.button-BRScreener-[EspaDivx.com].rar", Quality::SCR),
        (
            "Thor- Love and Thunder (2022) Original Hindi Dubbed 1080p HQ PreDVD Rip x264 AAC [1.7 GB]- CineVood.mkv",
            Quality::SCR,
        ),
        (
            "Black Panther Wakanda Forever 2022 Hindi 1080p PDVDRip x264 AAC CineVood.mkv",
            Quality::SCR,
        ),
        ("Vampire in Vegas (2009) NL Subs DVDR DivXNL-Team", Quality::DVD),
        (
            "Звонок из прошлого / Kol / The Call (2020) WEB-DLRip | ViruseProject",
            Quality::WebDLRip,
        ),
        (
            "La nube (2020) [BluRay Rip][AC3 5.1 Castellano][www.maxitorrent.com]",
            Quality::BRRip,
        ),
        (
            "Joker.2019.2160p.BluRay.REMUX.HEVC.DTS-HD.MA.TrueHD.7.1.Atmos-FGT",
            Quality::BluRayRemux,
        ),
        (
            "Warcraft 2016 1080p Blu-ray Remux AVC TrueHD Atmos-KRaLiMaRKo",
            Quality::BluRayRemux,
        ),
        ("Joker.2019.UHD.BluRay.2160p.TrueHD.Atmos.7.1.HEVC.REMUX-JAT", Quality::BluRayRemux),
        (
            "Spider-Man No Way Home.2022.REMUX.1080p.Bluray.DTS-HD.MA.5.1.AVC-EVO[TGx]",
            Quality::BluRayRemux,
        ),
        ("Son of God 2014 HDR BDRemux 1080p.mkv", Quality::BluRayRemux),
        (
            "Peter Rabbit 2 [4K UHDremux][2160p][HDR10][DTS-HD 5.1 Castellano-TrueHD 7.1-Ingles+Subs][ES-EN]",
            Quality::BluRayRemux,
        ),
        (
            "Snatch cerdos y diamantes [4KUHDremux 2160p][Castellano AC3 5.1-Ingles TrueHD 7.1+Subs]",
            Quality::BluRayRemux,
        ),
        ("Троя / Troy [2004 HDDVDRip-AVC] Dub + Original + Sub]", Quality::DVDRip),
        ("Структура момента (Расим Исмайлов) [1980, Драма, VHSRip]", Quality::VHSRip),
        ("Мужчины без женщин (Альгимантас Видугирис) [1981, Драма, VHS]", Quality::VHS),
        ("Преферанс по пятницам (Игорь Шешуков) [1984, Детектив, DVB]", Quality::HDTV),
        ("Соперницы (Алексей Дмитриев) [1929, драма, WEB-DLRip]", Quality::WebDLRip),
        ("Dragon Blade (2015) HDTSRip Exclusive", Quality::TeleSync),
        ("Criminal (2016) Hindi Dubbed HDTCRip", Quality::TeleCine),
        ("Avatar La Voie de l'eau.FRENCH.CAMHD.H264.AAC", Quality::Cam),
        (
            "www.1TamilBlasters.link - Indian 2 (2024) [Tamil - 1080p Proper HQ PRE-HDRip - x264 - AAC - 6.3GB - HQ Real Audio].mkv",
            Quality::SCR,
        ),
    ];

    for (release_name, expected_quality) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(
            result.quality,
            Some(expected_quality),
            "Quality detection failed for {}",
            release_name
        );
    }
}
