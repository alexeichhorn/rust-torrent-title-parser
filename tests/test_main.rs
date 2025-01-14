use torrent_title_parser::{parse_title, Codec, Language, ParsedTitle};

#[test]
fn test_main_parsing() {
    let test_cases = vec![
        (
            "sons.of.anarchy.s05e10.480p.BluRay.x264-GAnGSteR",
            ParsedTitle {
                title: "sons of anarchy".to_string(),
                resolution: Some("480p".to_string()),
                seasons: vec![5],
                episodes: vec![10],
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Avc),
                group: Some("GAnGSteR".to_string()),
                ..Default::default()
            },
        ),
        (
            "Color.Of.Night.Unrated.DC.VostFR.BRrip.x264",
            ParsedTitle {
                title: "Color Of Night".to_string(),
                unrated: true,
                languages: vec![Language::French],
                quality: Some("BRRip".to_string()),
                codec: Some(Codec::Avc),
                ..Default::default()
            },
        ),
        (
            "Da Vinci Code DVDRip",
            ParsedTitle {
                title: "Da Vinci Code".to_string(),
                quality: Some("DVDRip".to_string()),
                ..Default::default()
            },
        ),
        (
            "Some.girls.1998.DVDRip",
            ParsedTitle {
                title: "Some girls".to_string(),
                quality: Some("DVDRip".to_string()),
                year: Some(1998),
                ..Default::default()
            },
        ),
        (
            "Ecrit.Dans.Le.Ciel.1954.MULTI.DVDRIP.x264.AC3-gismo65",
            ParsedTitle {
                title: "Ecrit Dans Le Ciel".to_string(),
                quality: Some("DVDRip".to_string()),
                year: Some(1954),
                dubbed: true,
                codec: Some(Codec::Avc),
                audio: vec!["AC3".to_string()],
                group: Some("gismo65".to_string()),
                ..Default::default()
            },
        ),
        (
            "2019 After The Fall Of New York 1983 REMASTERED BDRip x264-GHOULS",
            ParsedTitle {
                title: "2019 After The Fall Of New York".to_string(),
                quality: Some("BDRip".to_string()),
                edition: Some("Remastered".to_string()),
                year: Some(1983),
                codec: Some(Codec::Avc),
                group: Some("GHOULS".to_string()),
                ..Default::default()
            },
        ),
        (
            "Ghost In The Shell 2017 720p HC HDRip X264 AC3-EVO",
            ParsedTitle {
                title: "Ghost In The Shell".to_string(),
                quality: Some("HDRip".to_string()),
                hardcoded: true,
                year: Some(2017),
                resolution: Some("720p".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["AC3".to_string()],
                group: Some("EVO".to_string()),
                ..Default::default()
            },
        ),
        (
            "Rogue One 2016 1080p BluRay x264-SPARKS",
            ParsedTitle {
                title: "Rogue One".to_string(),
                quality: Some("BluRay".to_string()),
                year: Some(2016),
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Avc),
                group: Some("SPARKS".to_string()),
                ..Default::default()
            },
        ),
        (
            "Desperation 2006 Multi Pal DvdR9-TBW1973",
            ParsedTitle {
                title: "Desperation".to_string(),
                quality: Some("DVD".to_string()),
                year: Some(2006),
                dubbed: true,
                region: Some("R9".to_string()),
                group: Some("TBW1973".to_string()),
                ..Default::default()
            },
        ),
        (
            "Maman, j'ai raté l'avion 1990 VFI 1080p BluRay DTS x265-HTG",
            ParsedTitle {
                title: "Maman, j'ai raté l'avion".to_string(),
                quality: Some("BluRay".to_string()),
                year: Some(1990),
                audio: vec!["DTS Lossy".to_string()],
                resolution: Some("1080p".to_string()),
                languages: vec![Language::French],
                codec: Some(Codec::Hevc),
                group: Some("HTG".to_string()),
                ..Default::default()
            },
        ),
        (
            "Game of Thrones - The Complete Season 3 [HDTV]",
            ParsedTitle {
                title: "Game of Thrones".to_string(),
                seasons: vec![3],
                quality: Some("HDTV".to_string()),
                ..Default::default()
            },
        ),
        (
            "The Sopranos: The Complete Series (Season 1,2,3,4,5&6) + Extras",
            ParsedTitle {
                title: "The Sopranos".to_string(),
                seasons: vec![1, 2, 3, 4, 5, 6],
                complete: true,
                ..Default::default()
            },
        ),
        (
            "Skins Season S01-S07 COMPLETE UK Soundtrack 720p WEB-DL",
            ParsedTitle {
                title: "Skins".to_string(),
                seasons: vec![1, 2, 3, 4, 5, 6, 7],
                resolution: Some("720p".to_string()),
                quality: Some("WEB-DL".to_string()),
                ..Default::default()
            },
        ),
        (
            "Futurama.COMPLETE.S01-S07.720p.BluRay.x265-HETeam",
            ParsedTitle {
                title: "Futurama".to_string(),
                seasons: vec![1, 2, 3, 4, 5, 6, 7],
                resolution: Some("720p".to_string()),
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Hevc),
                group: Some("HETeam".to_string()),
                ..Default::default()
            },
        ),
        (
            "You.[Uncut].S01.SweSub.1080p.x264-Justiso",
            ParsedTitle {
                title: "You".to_string(),
                edition: Some("Uncut".to_string()),
                seasons: vec![1],
                languages: vec![Language::Swedish],
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Avc),
                group: Some("Justiso".to_string()),
                ..Default::default()
            },
        ),
        (
            "Stephen Colbert 2019 10 25 Eddie Murphy 480p x264-mSD [eztv]",
            ParsedTitle {
                title: "Stephen Colbert".to_string(),
                date: Some("2019-10-25".to_string()),
                resolution: Some("480p".to_string()),
                codec: Some(Codec::Avc),
                ..Default::default()
            },
        ),
        (
            "House MD Season 7 Complete MKV",
            ParsedTitle {
                title: "House MD".to_string(),
                seasons: vec![7],
                container: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "2008 The Incredible Hulk Feature Film.mp4",
            ParsedTitle {
                title: "The Incredible Hulk Feature Film".to_string(),
                year: Some(2008),
                container: Some("mp4".to_string()),
                extension: Some("mp4".to_string()),
                ..Default::default()
            },
        ),
        (
            "【4月/悠哈璃羽字幕社】[UHA-WINGS][不要输！恶之军团][Makeruna!! Aku no Gundan!][04][1080p AVC_AAC][简繁外挂][sc_tc]",
            ParsedTitle {
                title: "Makeruna!! Aku no Gundan!".to_string(),
                episodes: vec![4],
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["AAC".to_string()],
                languages: vec![Language::Chinese],
                trash: true,
                ..Default::default()
            },
        ),
        (
            "[GM-Team][国漫][西行纪之集结篇][The Westward Ⅱ][2019][17][AVC][GB][1080P]",
            ParsedTitle {
                title: "The Westward Ⅱ".to_string(),
                year: Some(2019),
                episodes: vec![17],
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Avc),
                group: Some("GM-Team".to_string()),
                languages: vec![Language::Chinese],
                ..Default::default()
            },
        ),
        (
            "Черное зеркало / Black Mirror / Сезон 4 / Серии 1-6 (6) [2017, США, WEBRip 1080p] MVO + Eng Sub",
            ParsedTitle {
                title: "Black Mirror".to_string(),
                year: Some(2017),
                seasons: vec![4],
                episodes: vec![1, 2, 3, 4, 5, 6],
                languages: vec![Language::English, Language::Russian],
                resolution: Some("1080p".to_string()),
                quality: Some("WEBRip".to_string()),
                subbed: true,
                ..Default::default()
            },
        ),
        (
            "[neoHEVC] Student Council's Discretion / Seitokai no Ichizon [Season 1] [BD 1080p x265 HEVC AAC]",
            ParsedTitle {
                title: "Student Council's Discretion / Seitokai no Ichizon".to_string(),
                seasons: vec![1],
                resolution: Some("1080p".to_string()),
                quality: Some("BDRip".to_string()),
                audio: vec!["AAC".to_string()],
                codec: Some(Codec::Hevc),
                group: Some("neoHEVC".to_string()),
                ..Default::default()
            },
        ),
        (
            "[Commie] Chihayafuru 3 - 21 [BD 720p AAC] [5F1911ED].mkv",
            ParsedTitle {
                title: "Chihayafuru 3".to_string(),
                episodes: vec![21],
                resolution: Some("720p".to_string()),
                quality: Some("BDRip".to_string()),
                audio: vec!["AAC".to_string()],
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                episode_code: Some("5F1911ED".to_string()),
                group: Some("Commie".to_string()),
                ..Default::default()
            },
        ),
        (
            "[DVDRip-ITA]The Fast and the Furious: Tokyo Drift [CR-Bt]",
            ParsedTitle {
                title: "The Fast and the Furious: Tokyo Drift".to_string(),
                quality: Some("DVDRip".to_string()),
                languages: vec![Language::Italian],
                ..Default::default()
            },
        ),
        (
            "[BluRay Rip 720p ITA AC3 - ENG AC3 SUB] Hostel[2005]-LIFE[ultimafrontiera]",
            ParsedTitle {
                title: "Hostel".to_string(),
                year: Some(2005),
                resolution: Some("720p".to_string()),
                quality: Some("BRRip".to_string()),
                audio: vec!["AC3".to_string()],
                languages: vec![Language::English, Language::Italian],
                group: Some("LIFE".to_string()),
                subbed: true,
                ..Default::default()
            },
        ),
        (
            "[OFFICIAL ENG SUB] Soul Land Episode 121-125 [1080p][Soft Sub][Web-DL][Douluo Dalu][斗罗大陆]",
            ParsedTitle {
                title: "Soul Land".to_string(),
                episodes: vec![121, 122, 123, 124, 125],
                languages: vec![Language::English, Language::Chinese],
                resolution: Some("1080p".to_string()),
                quality: Some("WEB-DL".to_string()),
                subbed: true,
                ..Default::default()
            },
        ),
        (
            "[720p] The God of Highschool Season 1",
            ParsedTitle {
                title: "The God of Highschool".to_string(),
                seasons: vec![1],
                resolution: Some("720p".to_string()),
                ..Default::default()
            },
        ),
        (
            "Heidi Audio Latino DVDRip [cap. 3 Al 18]",
            ParsedTitle {
                title: "Heidi".to_string(),
                episodes: vec![3],
                quality: Some("DVDRip".to_string()),
                languages: vec![Language::LatinAmericanSpanish],
                ..Default::default()
            },
        ),
        (
            "Anatomia De Grey - Temporada 19 [HDTV][Castellano][www.AtomoHD.nu].avi",
            ParsedTitle {
                title: "Anatomia De Grey".to_string(),
                seasons: vec![19],
                container: Some("avi".to_string()),
                extension: Some("avi".to_string()),
                languages: vec![Language::Spanish],
                quality: Some("HDTV".to_string()),
                site: Some("www.AtomoHD.nu".to_string()),
                ..Default::default()
            },
        ),
        (
            "Sprint.2024.S01.COMPLETE.1080p.WEB.h264-EDITH[TGx]",
            ParsedTitle {
                title: "Sprint".to_string(),
                year: Some(2024),
                seasons: vec![1],
                quality: Some("WEB".to_string()),
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Avc),
                group: Some("EDITH".to_string()),
                scene: true,
                ..Default::default()
            },
        ),
        (
            "Madame Web 2024 UHD BluRay 2160p TrueHD Atmos 7 1 DV HEVC REMUX-FraMeSToR",
            ParsedTitle {
                title: "Madame Web".to_string(),
                year: Some(2024),
                quality: Some("BluRay REMUX".to_string()),
                resolution: Some("2160p".to_string()),
                channels: vec!["7.1".to_string()],
                audio: vec!["Atmos".to_string(), "TrueHD".to_string()],
                codec: Some(Codec::Hevc),
                hdr: vec!["DV".to_string()],
                group: Some("FraMeSToR".to_string()),
                ..Default::default()
            },
        ),
        (
            "The.Witcher.US.S01.INTERNAL.1080p.WEB.x264-STRiFE",
            ParsedTitle {
                title: "The Witcher US".to_string(),
                seasons: vec![1],
                quality: Some("WEB".to_string()),
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Avc),
                group: Some("STRiFE".to_string()),
                scene: true,
                ..Default::default()
            },
        ),
        (
            "Madame Web (2024) 1080p HINDI ENGLISH 10bit AMZN WEBRip DDP5 1 x265 HEVC - PSA Shadow",
            ParsedTitle {
                title: "Madame Web".to_string(),
                year: Some(2024),
                languages: vec![Language::English, Language::Hindi],
                quality: Some("WEBRip".to_string()),
                resolution: Some("1080p".to_string()),
                bit_depth: Some("10bit".to_string()),
                audio: vec!["Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                codec: Some(Codec::Hevc),
                network: Some("Amazon".to_string()),
                ..Default::default()
            },
        ),
        (
            "The Simpsons S01E01 1080p BluRay x265 HEVC 10bit AAC 5.1 Tigole",
            ParsedTitle {
                title: "The Simpsons".to_string(),
                seasons: vec![1],
                episodes: vec![1],
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Hevc),
                bit_depth: Some("10bit".to_string()),
                audio: vec!["AC3".to_string(), "AAC".to_string()],
                channels: vec!["5.1".to_string()],
                ..Default::default()
            },
        ),
        (
            "[DB]_Bleach_264_[012073FE].avi",
            ParsedTitle {
                title: "Bleach".to_string(),
                container: Some("avi".to_string()),
                extension: Some("avi".to_string()),
                episode_code: Some("012073FE".to_string()),
                episodes: vec![264],
                group: Some("DB".to_string()),
                ..Default::default()
            },
        ),
        (
            "[SubsPlease] One Piece - 1111 (480p) [2E05E658].mkv",
            ParsedTitle {
                title: "One Piece".to_string(),
                container: Some("mkv".to_string()),
                resolution: Some("480p".to_string()),
                extension: Some("mkv".to_string()),
                episode_code: Some("2E05E658".to_string()),
                episodes: vec![1111],
                group: Some("SubsPlease".to_string()),
                ..Default::default()
            },
        ),
        (
            "One Piece S01E1056 VOSTFR 1080p WEB x264 AAC -Tsundere-Raws (CR) mkv",
            ParsedTitle {
                title: "One Piece".to_string(),
                seasons: vec![1],
                episodes: vec![1056],
                languages: vec![Language::French],
                container: Some("mkv".to_string()),
                resolution: Some("1080p".to_string()),
                scene: true,
                quality: Some("WEB".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["AAC".to_string()],
                ..Default::default()
            },
        ),
        (
            "Mary.Poppins.1964.50th.ANNIVERSARY.EDITION.REMUX.1080p.Bluray.AVC.DTS-HD.MA.5.1-LEGi0N",
            ParsedTitle {
                title: "Mary Poppins".to_string(),
                year: Some(1964),
                edition: Some("Anniversary Edition".to_string()),
                quality: Some("BluRay REMUX".to_string()),
                resolution: Some("1080p".to_string()),
                audio: vec!["DTS Lossless".to_string()],
                channels: vec!["5.1".to_string()],
                codec: Some(Codec::Avc),
                group: Some("LEGi0N".to_string()),
                ..Default::default()
            },
        ),
        (
            "The.Lord.of.the.Rings.The.Fellowship.of.the.Ring.2001.EXTENDED.2160p.UHD.BluRay.x265.10bit.HDR.TrueHD.7.1.Atmos-BOREDOR",
            ParsedTitle {
                title: "The Lord of the Rings The Fellowship of the Ring".to_string(),
                year: Some(2001),
                resolution: Some("2160p".to_string()),
                edition: Some("Extended Edition".to_string()),
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Hevc),
                bit_depth: Some("10bit".to_string()),
                audio: vec!["Atmos".to_string(), "TrueHD".to_string()],
                channels: vec!["7.1".to_string()],
                hdr: vec!["HDR".to_string()],
                group: Some("BOREDOR".to_string()),
                ..Default::default()
            },
        ),
        (
            "Escaflowne (2000) (BDRip 1896x1048p x265 HEVC TrueHD, FLACx3, AC3 5.1x2+2.0x3)(Triple Audio)[sxales].mkv",
            ParsedTitle {
                title: "Escaflowne".to_string(),
                year: Some(2000),
                quality: Some("BDRip".to_string()),
                codec: Some(Codec::Hevc),
                resolution: Some("1896x1048p".to_string()),
                audio: vec!["TrueHD".to_string(), "FLAC".to_string(), "AC3".to_string()],
                channels: vec!["5.1".to_string()],
                dubbed: true,
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "[www.1TamilMV.pics]_The.Great.Indian.Suicide.2023.Tamil.TRUE.WEB-DL.4K.SDR.HEVC.(DD+5.1.384Kbps.&.AAC).3.2GB.ESub.mkv",
            ParsedTitle {
                title: "The Great Indian Suicide".to_string(),
                year: Some(2023),
                languages: vec![Language::English, Language::Tamil],
                quality: Some("WEB-DL".to_string()),
                resolution: Some("2160p".to_string()),
                hdr: vec!["SDR".to_string()],
                codec: Some(Codec::Hevc),
                site: Some("www.1TamilMV.pics".to_string()),
                size: Some("3.2GB".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                bitrate: Some("384kbps".to_string()),
                audio: vec!["TrueHD".to_string(), "Dolby Digital Plus".to_string(), "AAC".to_string()],
                channels: vec!["5.1".to_string()],
                ..Default::default()
            },
        ),
        (
            "www.5MovieRulz.show - Khel Khel Mein (2024) 1080p Hindi DVDScr - x264 - AAC - 2.3GB.mkv",
            ParsedTitle {
                title: "Khel Khel Mein".to_string(),
                year: Some(2024),
                languages: vec![Language::Hindi],
                quality: Some("SCR".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["AAC".to_string()],
                resolution: Some("1080p".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                size: Some("2.3GB".to_string()),
                site: Some("www.5MovieRulz.show".to_string()),
                trash: true,
                ..Default::default()
            },
        ),
        (
            "超能警探.Memorist.S01E01.2160p.WEB-DL.H265.AAC-FLTTH.mkv",
            ParsedTitle {
                title: "Memorist".to_string(),
                seasons: vec![1],
                episodes: vec![1],
                languages: vec![Language::Chinese],
                quality: Some("WEB-DL".to_string()),
                codec: Some(Codec::Hevc),
                audio: vec!["AAC".to_string()],
                resolution: Some("2160p".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                group: Some("FLTTH".to_string()),
                ..Default::default()
            },
        ),
        (
            "Futurama.S08E03.How.the.West.Was.1010001.1080p.HULU.WEB-DL.DDP5.1.H.264-FLUX.mkv",
            ParsedTitle {
                title: "Futurama".to_string(),
                seasons: vec![8],
                episodes: vec![3],
                network: Some("Hulu".to_string()),
                codec: Some(Codec::Avc),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                audio: vec!["Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                quality: Some("WEB-DL".to_string()),
                resolution: Some("1080p".to_string()),
                group: Some("FLUX".to_string()),
                ..Default::default()
            },
        ),
        (
            "V.H.S.2 [2013] 1080p BDRip x265 DTS-HD MA 5.1 Kira [SEV].mkv",
            ParsedTitle {
                title: "V H S 2".to_string(),
                year: Some(2013),
                quality: Some("BDRip".to_string()),
                codec: Some(Codec::Hevc),
                audio: vec!["DTS Lossless".to_string()],
                channels: vec!["5.1".to_string()],
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                resolution: Some("1080p".to_string()),
                ..Default::default()
            },
        ),
        (
            "{WWW.BLUDV.TV} Love, Death & Robots - 1ª Temporada Completa 2019 (1080p) Acesse o ORIGINAL WWW.BLUDV.TV",
            ParsedTitle {
                title: "Love, Death & Robots".to_string(),
                seasons: vec![1],
                languages: vec![Language::Spanish],
                resolution: Some("1080p".to_string()),
                year: Some(2019),
                complete: true,
                site: Some("WWW.BLUDV.TV".to_string()),
                trash: true,
                ..Default::default()
            },
        ),
        (
            "www.MovCr.to - Bikram Yogi, Guru, Predator (2019) 720p WEB_DL x264 ESubs [Dual Audio]-[Hindi + Eng] - 950MB - MovCr.mkv",
            ParsedTitle {
                title: "Bikram Yogi, Guru, Predator".to_string(),
                year: Some(2019),
                languages: vec![Language::English, Language::Hindi],
                quality: Some("WEB-DL".to_string()),
                resolution: Some("720p".to_string()),
                codec: Some(Codec::Avc),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                site: Some("www.MovCr.to".to_string()),
                dubbed: true,
                group: Some("MovCr".to_string()),
                size: Some("950MB".to_string()),
                ..Default::default()
            },
        ),
        (
            "28.days.2000.1080p.bluray.x264-mimic.mkv",
            ParsedTitle {
                title: "28 days".to_string(),
                year: Some(2000),
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Avc),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                group: Some("mimic".to_string()),
                ..Default::default()
            },
        ),
        (
            "4.20.Massacre.2018.1080p.BluRay.x264.AAC-[YTS.MX].mp4",
            ParsedTitle {
                title: "4 20 Massacre".to_string(),
                year: Some(2018),
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["AAC".to_string()],
                container: Some("mp4".to_string()),
                extension: Some("mp4".to_string()),
                site: Some("YTS.MX".to_string()),
                ..Default::default()
            },
        ),
        (
            "inside.out.2.2024.d.ru.ua.ts.1o8op.mkv",
            ParsedTitle {
                title: "inside out 2".to_string(),
                year: Some(2024),
                quality: Some("TeleSync".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                languages: vec![Language::Russian],
                trash: true,
                ..Default::default()
            },
        ),
        (
            "I.S.S.2023.P.WEB-DL.1O8Op.mkv",
            ParsedTitle {
                title: "I S S".to_string(),
                year: Some(2023),
                quality: Some("WEB-DL".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "Skazka.2022.Pa.WEB-DL.1O8Op.mkv",
            ParsedTitle {
                title: "Skazka".to_string(),
                year: Some(2022),
                quality: Some("WEB-DL".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "Spider-Man.Across.the.Spider-Verse.2023.Dt.WEBRip.1O8Op.mkv",
            ParsedTitle {
                title: "Spider-Man Across the Spider-Verse".to_string(),
                year: Some(2023),
                quality: Some("WEBRip".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "Civil.War.2024.D.WEB-DL.1O8Op.mkv",
            ParsedTitle {
                title: "Civil War".to_string(),
                year: Some(2024),
                quality: Some("WEB-DL".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "Dune.Part.Two.2024.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX[TGx]",
            ParsedTitle {
                title: "Dune Part Two".to_string(),
                year: Some(2024),
                resolution: Some("2160p".to_string()),
                quality: Some("WEB-DL".to_string()),
                codec: Some(Codec::Hevc),
                audio: vec!["Dolby Digital Plus".to_string(), "Atmos".to_string()],
                channels: vec!["5.1".to_string()],
                group: Some("FLUX".to_string()),
                hdr: vec!["DV".to_string(), "HDR".to_string()],
                ..Default::default()
            },
        ),
        (
            "Saw.3D.2010.1080p.ITA-ENG.BluRay.x265.AAC-V3SP4EV3R.mkv",
            ParsedTitle {
                title: "Saw 3D".to_string(),
                year: Some(2010),
                languages: vec![Language::English, Language::Italian],
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Hevc),
                audio: vec!["AAC".to_string()],
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                group: Some("V3SP4EV3R".to_string()),
                ..Default::default()
            },
        ),
        (
            "Dead Before Dawn 3D (2012) [3D.BLU-RAY] [1080p 3D] [BluRay] [HSBS] [YTS.MX]",
            ParsedTitle {
                title: "Dead Before Dawn 3D".to_string(),
                year: Some(2012),
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                is_3d: true,
                ..Default::default()
            },
        ),
        (
            "Wonder.Woman.1984.2020.3D.1080p.BluRay.x264-SURCODE[rarbg]",
            ParsedTitle {
                title: "Wonder Woman 1984".to_string(),
                year: Some(2020),
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Avc),
                group: Some("SURCODE".to_string()),
                scene: true,
                is_3d: true,
                ..Default::default()
            },
        ),
        (
            "The.Last.of.Us.S01E08.1080p.WEB.H264-CAKES[TGx]",
            ParsedTitle {
                title: "The Last of Us".to_string(),
                seasons: vec![1],
                episodes: vec![8],
                resolution: Some("1080p".to_string()),
                quality: Some("WEB".to_string()),
                codec: Some(Codec::Avc),
                group: Some("CAKES".to_string()),
                scene: true,
                ..Default::default()
            },
        ),
        (
            "The.Office.UK.S01.1080P.BLURAY.REMUX.AVC.DD5.1-NOGRP",
            ParsedTitle {
                title: "The Office UK".to_string(),
                seasons: vec![1],
                quality: Some("BluRay REMUX".to_string()),
                resolution: Some("1080p".to_string()),
                audio: vec!["Dolby Digital".to_string()],
                channels: vec!["5.1".to_string()],
                codec: Some(Codec::Avc),
                group: Some("NOGRP".to_string()),
                languages: vec![],
                ..Default::default()
            },
        ),
        (
            "The.Office.US.S01-09.COMPLETE.SERIES.1080P.BLURAY.X265-HIQVE",
            ParsedTitle {
                title: "The Office US".to_string(),
                seasons: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                quality: Some("BluRay".to_string()),
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Hevc),
                group: Some("HIQVE".to_string()),
                complete: true,
                languages: vec![],
                ..Default::default()
            },
        ),
        (
            "Hard Knocks 2001 S23E01 1080p MAX WEB-DL DDP2 0 x264-NTb[EZTVx.to].mkv",
            ParsedTitle {
                title: "Hard Knocks".to_string(),
                year: Some(2001),
                seasons: vec![23],
                episodes: vec![1],
                quality: Some("WEB-DL".to_string()),
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["Dolby Digital Plus".to_string()],
                group: Some("NTb".to_string()),
                extension: Some("mkv".to_string()),
                container: Some("mkv".to_string()),
                site: Some("EZTVx.to".to_string()),
                ..Default::default()
            },
        ),
        (
            "Fallout.S01E03.The.Head.2160p.DV.HDR10Plus.Ai-Enhanced.H265.DDP.5.1.MULTI.RIFE.4.15v2-60fps-DirtyHippie.mkv",
            ParsedTitle {
                title: "Fallout".to_string(),
                seasons: vec![1],
                episodes: vec![3],
                resolution: Some("2160p".to_string()),
                codec: Some(Codec::Hevc),
                audio: vec!["AC3".to_string(), "Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                group: Some("DirtyHippie".to_string()),
                container: Some("mkv".to_string()),
                dubbed: true,
                extension: Some("mkv".to_string()),
                hdr: vec!["DV".to_string(), "HDR10+".to_string()],
                upscaled: true,
                ..Default::default()
            },
        ),
        (
            "BoJack Horseman [06x01-08 of 16] (2019-2020) WEB-DLRip 720p",
            ParsedTitle {
                title: "BoJack Horseman".to_string(),
                seasons: vec![6],
                episodes: vec![1, 2, 3, 4, 5, 6, 7, 8],
                resolution: Some("720p".to_string()),
                quality: Some("WEB-DLRip".to_string()),
                complete: true,
                ..Default::default()
            },
        ),
        (
            "Трон: Наследие / TRON: Legacy (2010) WEB-DL 1080p | D | Open Matte",
            ParsedTitle {
                title: "TRON: Legacy".to_string(),
                year: Some(2010),
                languages: vec![Language::Russian],
                resolution: Some("1080p".to_string()),
                quality: Some("WEB-DL".to_string()),
                ..Default::default()
            },
        ),
        (
            "Wentworth.S08E06.PDTV.AAC2.0.x264-BTN",
            ParsedTitle {
                title: "Wentworth".to_string(),
                seasons: vec![8],
                episodes: vec![6],
                quality: Some("PDTV".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["AAC".to_string()],
                group: Some("BTN".to_string()),
                ..Default::default()
            },
        ),
        (
            "www.1Tamilblasters.co - Guardians of the Galaxy Vol. 3 (2023) [4K IMAX UHD HEVC - BDRip - [Tam + Mal + Tel + Hin + Eng] - x264 - DDP5.1 (192Kbps) - 8.3GB - ESub].mkv",
            ParsedTitle {
                title: "Guardians of the Galaxy Vol. 3".to_string(),
                year: Some(2023),
                languages: vec![Language::English, Language::Hindi, Language::Telugu, Language::Tamil, Language::Malayalam],
                quality: Some("BDRip".to_string()),
                codec: Some(Codec::Hevc),
                audio: vec!["Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                resolution: Some("2160p".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                site: Some("www.1Tamilblasters.co".to_string()),
                bitrate: Some("192kbps".to_string()),
                edition: Some("IMAX".to_string()),
                size: Some("8.3GB".to_string()),
                ..Default::default()
            },
        ),
        (
            "【高清影视之家发布 www.hdbthd.com】奥本海默 杜比视界版本 高码版 国英多音轨 中文字幕 .oppenheimer.2023.2160p.hq.web-dl.h265.dv.ddp5.1.2audio-dreamhd",
            ParsedTitle {
                title: "高清影视之家发布".to_string(),
                year: Some(2023),
                languages: vec![Language::Chinese],
                quality: Some("WEB-DL".to_string()),
                codec: Some(Codec::Hevc),
                audio: vec!["Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                resolution: Some("2160p".to_string()),
                site: Some("www.hdbthd.com".to_string()),
                group: Some("dreamhd".to_string()),
                hdr: vec!["DV".to_string()],
                trash: true,
                ..Default::default()
            },
        ),
        (
            "Venom (2018) HD-TS 720p Hindi Dubbed (Clean Audio) x264",
            ParsedTitle {
                title: "Venom".to_string(),
                year: Some(2018),
                languages: vec![Language::Hindi],
                quality: Some("TeleSync".to_string()),
                resolution: Some("720p".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["HQ Clean Audio".to_string()],
                dubbed: true,
                trash: true,
                ..Default::default()
            },
        ),
        (
            "www.Tamilblasters.party - The Wheel of Time (2021) Season 01 EP(01-08) [720p HQ HDRip - [Tam + Tel + Hin] - DDP5.1 - x264 - 2.7GB - ESubs]",
            ParsedTitle {
                title: "The Wheel of Time".to_string(),
                year: Some(2021),
                seasons: vec![1],
                episodes: vec![1, 2, 3, 4, 5, 6, 7, 8],
                languages: vec![Language::Hindi, Language::Telugu, Language::Tamil],
                quality: Some("HDRip".to_string()),
                resolution: Some("720p".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                site: Some("www.Tamilblasters.party".to_string()),
                size: Some("2.7GB".to_string()),
                trash: true,
                ..Default::default()
            },
        ),
        (
            "The.Walking.Dead.S06E07.SUBFRENCH.HDTV.x264-AMB3R.mkv",
            ParsedTitle {
                title: "The Walking Dead".to_string(),
                seasons: vec![6],
                episodes: vec![7],
                languages: vec![Language::French],
                quality: Some("HDTV".to_string()),
                codec: Some(Codec::Avc),
                group: Some("AMB3R".to_string()),
                extension: Some("mkv".to_string()),
                container: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "The Walking Dead S05E03 720p Remux x264-ASAP[ettv]",
            ParsedTitle {
                title: "The Walking Dead".to_string(),
                seasons: vec![5],
                episodes: vec![3],
                quality: Some("REMUX".to_string()),
                resolution: Some("720p".to_string()),
                codec: Some(Codec::Avc),
                group: Some("ASAP".to_string()),
                ..Default::default()
            },
        ),
        (
            "www.TamilBlasters.vip - Shang-Chi (2021) [720p BDRip - [Tamil + Telugu + Hindi + Eng] - x264 - DDP5.1 (192 Kbps) - 1.4GB - ESubs].mkv",
            ParsedTitle {
                title: "Shang-Chi".to_string(),
                year: Some(2021),
                languages: vec![Language::English, Language::Hindi, Language::Telugu, Language::Tamil],
                quality: Some("BDRip".to_string()),
                resolution: Some("720p".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                site: Some("www.TamilBlasters.vip".to_string()),
                size: Some("1.4GB".to_string()),
                extension: Some("mkv".to_string()),
                container: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "Game of Thrones 1ª a 8ª Temporada Completa [720p-1080p] [BluRay] [DUAL]",
            ParsedTitle {
                title: "Game of Thrones".to_string(),
                seasons: vec![1, 2, 3, 4, 5, 6, 7, 8],
                languages: vec![Language::Spanish],
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                complete: true,
                dubbed: true,
                ..Default::default()
            },
        ),
        (
            "Kill.2024.REPACK.1080p.AMZN.WEB-DL.DDP5.1.Atmos.H.264-XEBEC.mkv",
            ParsedTitle {
                title: "Kill".to_string(),
                year: Some(2024),
                resolution: Some("1080p".to_string()),
                quality: Some("WEB-DL".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["Dolby Digital Plus".to_string(), "Atmos".to_string()],
                channels: vec!["5.1".to_string()],
                group: Some("XEBEC".to_string()),
                container: Some("mkv".to_string()),
                extension: Some("mkv".to_string()),
                network: Some("Amazon".to_string()),
                repack: true,
                ..Default::default()
            },
        ),
        (
            "Mad.Max.Fury.Road.2015.1080p.BluRay.DDP5.1.x265.10bit-GalaxyRG265[TGx]",
            ParsedTitle {
                title: "Mad Max Fury Road".to_string(),
                year: Some(2015),
                resolution: Some("1080p".to_string()),
                codec: Some(Codec::Hevc),
                bit_depth: Some("10bit".to_string()),
                audio: vec!["Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                group: Some("GalaxyRG265".to_string()),
                quality: Some("BluRay".to_string()),
                ..Default::default()
            },
        ),
        (
            "Властелин колец: Кольца власти (S1E1-8 of 8) / The Lord of the Rings: The Rings of Power (2022) WEB-DL",
            ParsedTitle {
                title: "Властелин колец: Кольца власти".to_string(),
                year: Some(2022),
                seasons: vec![1],
                episodes: vec![1, 2, 3, 4, 5, 6, 7, 8],
                languages: vec![Language::Russian],
                quality: Some("WEB-DL".to_string()),
                ..Default::default()
            },
        ),
        (
            "抓娃娃 Successor.2024.TC1080P.国语中字",
            ParsedTitle {
                title: "Successor".to_string(),
                year: Some(2024),
                languages: vec![Language::Chinese],
                resolution: Some("1080p".to_string()),
                quality: Some("TeleCine".to_string()),
                trash: true,
                ..Default::default()
            },
        ),
        (
            "True.Detective.S03E02.720p.WEB.x265-MiNX[eztv].mkv",
            ParsedTitle {
                title: "True Detective".to_string(),
                seasons: vec![3],
                episodes: vec![2],
                resolution: Some("720p".to_string()),
                scene: true,
                quality: Some("WEB".to_string()),
                codec: Some(Codec::Hevc),
                group: Some("MiNX".to_string()),
                extension: Some("mkv".to_string()),
                container: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "True.Grit.1969.720p.WEB.x265-MiNX[eztv].mkv",
            ParsedTitle {
                title: "True Grit".to_string(),
                year: Some(1969),
                resolution: Some("720p".to_string()),
                scene: true,
                quality: Some("WEB".to_string()),
                codec: Some(Codec::Hevc),
                group: Some("MiNX".to_string()),
                extension: Some("mkv".to_string()),
                container: Some("mkv".to_string()),
                ..Default::default()
            },
        ),
        (
            "Free Samples (2012) [BluRay] [1080p] [YTS.AM]",
            ParsedTitle {
                title: "Free Samples".to_string(),
                year: Some(2012),
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                ..Default::default()
            },
        ),
        (
            "Trailer Park Boys S01-S10 + Movies + Specials + Extras [Ultimate Collection]-CAPTAiN",
            ParsedTitle {
                title: "Trailer Park Boys".to_string(),
                seasons: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                complete: true,
                group: Some("CAPTAiN".to_string()),
                ..Default::default()
            },
        ),
        (
            "Adbhut (2024) Hindi 1080p HDTVRip x264 AAC 5.1 [2.2GB] - QRips",
            ParsedTitle {
                title: "Adbhut".to_string(),
                year: Some(2024),
                languages: vec![Language::Hindi],
                resolution: Some("1080p".to_string()),
                quality: Some("HDTVRip".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["AC3".to_string(), "AAC".to_string()],
                channels: vec!["5.1".to_string()],
                group: Some("QRips".to_string()),
                size: Some("2.2GB".to_string()),
                ..Default::default()
            },
        ),
        (
            "Blood Diamond (2006) 1080p BluRay H264 DolbyD 5 1 + nickarad mp4",
            ParsedTitle {
                title: "Blood Diamond".to_string(),
                year: Some(2006),
                resolution: Some("1080p".to_string()),
                quality: Some("BluRay".to_string()),
                codec: Some(Codec::Avc),
                audio: vec!["Dolby Digital".to_string()],
                channels: vec!["5.1".to_string()],
                container: Some("mp4".to_string()),
                ..Default::default()
            },
        ),
        (
            "The Lockerbie Bombing (2013) Documentary HDTVRIP",
            ParsedTitle {
                title: "The Lockerbie Bombing".to_string(),
                year: Some(2013),
                documentary: true,
                quality: Some("HDTVRip".to_string()),
                ..Default::default()
            },
        ),
        (
            "STEVE.martin.a.documentary.in.2.pieces.S01.COMPLETE.1080p.WEB.H264-SuccessfulCrab[TGx]",
            ParsedTitle {
                title: "STEVE martin a documentary in 2 pieces".to_string(),
                seasons: vec![1],
                quality: Some("WEB".to_string()),
                codec: Some(Codec::Avc),
                group: Some("SuccessfulCrab".to_string()),
                resolution: Some("1080p".to_string()),
                documentary: true,
                scene: true,
                ..Default::default()
            },
        ),
        (
            "The New Frontier S01E10 720p WEB H264-INFLATE[eztv] mkv",
            ParsedTitle {
                title: "The New Frontier".to_string(),
                seasons: vec![1],
                episodes: vec![10],
                quality: Some("WEB".to_string()),
                container: Some("mkv".to_string()),
                codec: Some(Codec::Avc),
                group: Some("INFLATE".to_string()),
                resolution: Some("720p".to_string()),
                scene: true,
                ..Default::default()
            },
        ),
        (
            "[BEST-TORRENTS.COM] The.Penguin.S01E07.MULTi.1080p.AMZN.WEB-DL.H264.DDP5.1.Atmos-K83",
            ParsedTitle {
                title: "The Penguin".to_string(),
                seasons: vec![1],
                episodes: vec![7],
                resolution: Some("1080p".to_string()),
                quality: Some("WEB-DL".to_string()),
                network: Some("Amazon".to_string()),
                codec: Some(Codec::Avc),
                dubbed: true,
                audio: vec!["Dolby Digital Plus".to_string(), "Atmos".to_string()],
                channels: vec!["5.1".to_string()],
                site: Some("BEST-TORRENTS.COM".to_string()),
                ..Default::default()
            },
        ),
        (
            "[ Torrent911.my ] The.Penguin.S01E07.FRENCH.WEBRip.x264.mp4",
            ParsedTitle {
                title: "The Penguin".to_string(),
                seasons: vec![1],
                episodes: vec![7],
                languages: vec![Language::French],
                quality: Some("WEBRip".to_string()),
                codec: Some(Codec::Avc),
                site: Some("Torrent911.my".to_string()),
                container: Some("mp4".to_string()),
                extension: Some("mp4".to_string()),
                ..Default::default()
            },
        ),
        (
            "The.O.C.Seasons.01-04.AMZN.1080p.10bit.x265.hevc-Bearfish",
            ParsedTitle {
                title: "The O C".to_string(),
                seasons: vec![1, 2, 3, 4],
                resolution: Some("1080p".to_string()),
                network: Some("Amazon".to_string()),
                codec: Some(Codec::Hevc),
                bit_depth: Some("10bit".to_string()),
                group: Some("Bearfish".to_string()),
                ..Default::default()
            },
        ),
        (
            "The Adam Project 2022 2160p NF WEB-DL DDP 5 1 Atmos DoVi HDR HEVC-SiC mkv",
            ParsedTitle {
                title: "The Adam Project".to_string(),
                year: Some(2022),
                resolution: Some("2160p".to_string()),
                quality: Some("WEB-DL".to_string()),
                network: Some("Netflix".to_string()),
                codec: Some(Codec::Hevc),
                container: Some("mkv".to_string()),
                audio: vec!["Atmos".to_string(), "Dolby Digital Plus".to_string()],
                channels: vec!["5.1".to_string()],
                hdr: vec!["DV".to_string(), "HDR".to_string()],
                ..Default::default()
            },
        )
    ];

    for (release_name, expected_output) in test_cases {
        let result = parse_title(release_name).unwrap();
        assert_eq!(result, expected_output, "Failed for {}", release_name);
    }
}
