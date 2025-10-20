use invidious::{ClientSync, ClientSyncTrait};

use crate::global::traits::SearchProviderTrait;

// used in `data.global`
/// Holds the crate::global::common client
#[derive(Clone)]
pub struct InvidiousClient(pub ClientSync);

impl InvidiousClient {
    pub fn new(instance: String) -> Self {
        Self(ClientSync::default().instance(instance))
    }
}

impl From<invidious::hidden::Storyboard> for crate::global::common::hidden::Storyboard {
    fn from(value: invidious::hidden::Storyboard) -> Self {
        Self {
            url: value.url,
            template_url: value.template_url,
            width: value.width,
            height: value.height,
            count: value.count,
            interval: value.interval,
            storyboard_width: value.storyboard_width,
            storyboard_height: value.storyboard_height,
            storyboard_count: value.storyboard_count,
        }
    }
}

impl From<invidious::CommonThumbnail> for crate::global::common::CommonThumbnail {
    fn from(value: invidious::CommonThumbnail) -> Self {
        Self {
            quality: value.quality,
            url: value.url,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<invidious::hidden::CountryCode> for crate::global::common::hidden::CountryCode {
    fn from(value: invidious::hidden::CountryCode) -> Self {
        match value {
            invidious::hidden::CountryCode::AD => Self::AD,
            invidious::hidden::CountryCode::AE => Self::AE,
            invidious::hidden::CountryCode::AF => Self::AF,
            invidious::hidden::CountryCode::AG => Self::AG,
            invidious::hidden::CountryCode::AI => Self::AI,
            invidious::hidden::CountryCode::AL => Self::AL,
            invidious::hidden::CountryCode::AM => Self::AM,
            invidious::hidden::CountryCode::AO => Self::AO,
            invidious::hidden::CountryCode::AQ => Self::AQ,
            invidious::hidden::CountryCode::AR => Self::AR,
            invidious::hidden::CountryCode::AS => Self::AS,
            invidious::hidden::CountryCode::AT => Self::AT,
            invidious::hidden::CountryCode::AU => Self::AU,
            invidious::hidden::CountryCode::AW => Self::AW,
            invidious::hidden::CountryCode::AX => Self::AX,
            invidious::hidden::CountryCode::AZ => Self::AZ,
            invidious::hidden::CountryCode::BA => Self::BA,
            invidious::hidden::CountryCode::BB => Self::BB,
            invidious::hidden::CountryCode::BD => Self::BD,
            invidious::hidden::CountryCode::BE => Self::BE,
            invidious::hidden::CountryCode::BF => Self::BF,
            invidious::hidden::CountryCode::BG => Self::BG,
            invidious::hidden::CountryCode::BH => Self::BH,
            invidious::hidden::CountryCode::BI => Self::BI,
            invidious::hidden::CountryCode::BJ => Self::BJ,
            invidious::hidden::CountryCode::BL => Self::BL,
            invidious::hidden::CountryCode::BM => Self::BM,
            invidious::hidden::CountryCode::BN => Self::BN,
            invidious::hidden::CountryCode::BO => Self::BO,
            invidious::hidden::CountryCode::BQ => Self::BQ,
            invidious::hidden::CountryCode::BR => Self::BR,
            invidious::hidden::CountryCode::BS => Self::BS,
            invidious::hidden::CountryCode::BT => Self::BT,
            invidious::hidden::CountryCode::BV => Self::BV,
            invidious::hidden::CountryCode::BW => Self::BW,
            invidious::hidden::CountryCode::BY => Self::BY,
            invidious::hidden::CountryCode::BZ => Self::BZ,
            invidious::hidden::CountryCode::CA => Self::CA,
            invidious::hidden::CountryCode::CC => Self::CC,
            invidious::hidden::CountryCode::CD => Self::CD,
            invidious::hidden::CountryCode::CF => Self::CF,
            invidious::hidden::CountryCode::CG => Self::CG,
            invidious::hidden::CountryCode::CH => Self::CH,
            invidious::hidden::CountryCode::CI => Self::CI,
            invidious::hidden::CountryCode::CK => Self::CK,
            invidious::hidden::CountryCode::CL => Self::CL,
            invidious::hidden::CountryCode::CM => Self::CM,
            invidious::hidden::CountryCode::CN => Self::CN,
            invidious::hidden::CountryCode::CO => Self::CO,
            invidious::hidden::CountryCode::CR => Self::CR,
            invidious::hidden::CountryCode::CU => Self::CU,
            invidious::hidden::CountryCode::CV => Self::CV,
            invidious::hidden::CountryCode::CW => Self::CW,
            invidious::hidden::CountryCode::CX => Self::CX,
            invidious::hidden::CountryCode::CY => Self::CY,
            invidious::hidden::CountryCode::CZ => Self::CZ,
            invidious::hidden::CountryCode::DE => Self::DE,
            invidious::hidden::CountryCode::DJ => Self::DJ,
            invidious::hidden::CountryCode::DK => Self::DK,
            invidious::hidden::CountryCode::DM => Self::DM,
            invidious::hidden::CountryCode::DO => Self::DO,
            invidious::hidden::CountryCode::DZ => Self::DZ,
            invidious::hidden::CountryCode::EC => Self::EC,
            invidious::hidden::CountryCode::EE => Self::EE,
            invidious::hidden::CountryCode::EG => Self::EG,
            invidious::hidden::CountryCode::EH => Self::EH,
            invidious::hidden::CountryCode::ER => Self::ER,
            invidious::hidden::CountryCode::ES => Self::ES,
            invidious::hidden::CountryCode::ET => Self::ET,
            invidious::hidden::CountryCode::FI => Self::FI,
            invidious::hidden::CountryCode::FJ => Self::FJ,
            invidious::hidden::CountryCode::FK => Self::FK,
            invidious::hidden::CountryCode::FM => Self::FM,
            invidious::hidden::CountryCode::FO => Self::FO,
            invidious::hidden::CountryCode::FR => Self::FR,
            invidious::hidden::CountryCode::GA => Self::GA,
            invidious::hidden::CountryCode::GB => Self::GB,
            invidious::hidden::CountryCode::GD => Self::GD,
            invidious::hidden::CountryCode::GE => Self::GE,
            invidious::hidden::CountryCode::GF => Self::GF,
            invidious::hidden::CountryCode::GG => Self::GG,
            invidious::hidden::CountryCode::GH => Self::GH,
            invidious::hidden::CountryCode::GI => Self::GI,
            invidious::hidden::CountryCode::GL => Self::GL,
            invidious::hidden::CountryCode::GM => Self::GM,
            invidious::hidden::CountryCode::GN => Self::GN,
            invidious::hidden::CountryCode::GP => Self::GP,
            invidious::hidden::CountryCode::GQ => Self::GQ,
            invidious::hidden::CountryCode::GR => Self::GR,
            invidious::hidden::CountryCode::GS => Self::GS,
            invidious::hidden::CountryCode::GT => Self::GT,
            invidious::hidden::CountryCode::GU => Self::GU,
            invidious::hidden::CountryCode::GW => Self::GW,
            invidious::hidden::CountryCode::GY => Self::GY,
            invidious::hidden::CountryCode::HK => Self::HK,
            invidious::hidden::CountryCode::HM => Self::HM,
            invidious::hidden::CountryCode::HN => Self::HN,
            invidious::hidden::CountryCode::HR => Self::HR,
            invidious::hidden::CountryCode::HT => Self::HT,
            invidious::hidden::CountryCode::HU => Self::HU,
            invidious::hidden::CountryCode::ID => Self::ID,
            invidious::hidden::CountryCode::IE => Self::IE,
            invidious::hidden::CountryCode::IL => Self::IL,
            invidious::hidden::CountryCode::IM => Self::IM,
            invidious::hidden::CountryCode::IN => Self::IN,
            invidious::hidden::CountryCode::IO => Self::IO,
            invidious::hidden::CountryCode::IQ => Self::IQ,
            invidious::hidden::CountryCode::IR => Self::IR,
            invidious::hidden::CountryCode::IS => Self::IS,
            invidious::hidden::CountryCode::IT => Self::IT,
            invidious::hidden::CountryCode::JE => Self::JE,
            invidious::hidden::CountryCode::JM => Self::JM,
            invidious::hidden::CountryCode::JO => Self::JO,
            invidious::hidden::CountryCode::JP => Self::JP,
            invidious::hidden::CountryCode::KE => Self::KE,
            invidious::hidden::CountryCode::KG => Self::KG,
            invidious::hidden::CountryCode::KH => Self::KH,
            invidious::hidden::CountryCode::KI => Self::KI,
            invidious::hidden::CountryCode::KM => Self::KM,
            invidious::hidden::CountryCode::KN => Self::KN,
            invidious::hidden::CountryCode::KP => Self::KP,
            invidious::hidden::CountryCode::KR => Self::KR,
            invidious::hidden::CountryCode::KW => Self::KW,
            invidious::hidden::CountryCode::KY => Self::KY,
            invidious::hidden::CountryCode::KZ => Self::KZ,
            invidious::hidden::CountryCode::LA => Self::LA,
            invidious::hidden::CountryCode::LB => Self::LB,
            invidious::hidden::CountryCode::LC => Self::LC,
            invidious::hidden::CountryCode::LI => Self::LI,
            invidious::hidden::CountryCode::LK => Self::LK,
            invidious::hidden::CountryCode::LR => Self::LR,
            invidious::hidden::CountryCode::LS => Self::LS,
            invidious::hidden::CountryCode::LT => Self::LT,
            invidious::hidden::CountryCode::LU => Self::LU,
            invidious::hidden::CountryCode::LV => Self::LV,
            invidious::hidden::CountryCode::LY => Self::LY,
            invidious::hidden::CountryCode::MA => Self::MA,
            invidious::hidden::CountryCode::MC => Self::MC,
            invidious::hidden::CountryCode::MD => Self::MD,
            invidious::hidden::CountryCode::ME => Self::ME,
            invidious::hidden::CountryCode::MF => Self::MF,
            invidious::hidden::CountryCode::MG => Self::MG,
            invidious::hidden::CountryCode::MH => Self::MH,
            invidious::hidden::CountryCode::MK => Self::MK,
            invidious::hidden::CountryCode::ML => Self::ML,
            invidious::hidden::CountryCode::MM => Self::MM,
            invidious::hidden::CountryCode::MN => Self::MN,
            invidious::hidden::CountryCode::MO => Self::MO,
            invidious::hidden::CountryCode::MP => Self::MP,
            invidious::hidden::CountryCode::MQ => Self::MQ,
            invidious::hidden::CountryCode::MR => Self::MR,
            invidious::hidden::CountryCode::MS => Self::MS,
            invidious::hidden::CountryCode::MT => Self::MT,
            invidious::hidden::CountryCode::MU => Self::MU,
            invidious::hidden::CountryCode::MV => Self::MV,
            invidious::hidden::CountryCode::MW => Self::MW,
            invidious::hidden::CountryCode::MX => Self::MX,
            invidious::hidden::CountryCode::MY => Self::MY,
            invidious::hidden::CountryCode::MZ => Self::MZ,
            invidious::hidden::CountryCode::NA => Self::NA,
            invidious::hidden::CountryCode::NC => Self::NC,
            invidious::hidden::CountryCode::NE => Self::NE,
            invidious::hidden::CountryCode::NF => Self::NF,
            invidious::hidden::CountryCode::NG => Self::NG,
            invidious::hidden::CountryCode::NI => Self::NI,
            invidious::hidden::CountryCode::NL => Self::NL,
            invidious::hidden::CountryCode::NO => Self::NO,
            invidious::hidden::CountryCode::NP => Self::NP,
            invidious::hidden::CountryCode::NR => Self::NR,
            invidious::hidden::CountryCode::NU => Self::NU,
            invidious::hidden::CountryCode::NZ => Self::NZ,
            invidious::hidden::CountryCode::OM => Self::OM,
            invidious::hidden::CountryCode::PA => Self::PA,
            invidious::hidden::CountryCode::PE => Self::PE,
            invidious::hidden::CountryCode::PF => Self::PF,
            invidious::hidden::CountryCode::PG => Self::PG,
            invidious::hidden::CountryCode::PH => Self::PH,
            invidious::hidden::CountryCode::PK => Self::PK,
            invidious::hidden::CountryCode::PL => Self::PL,
            invidious::hidden::CountryCode::PM => Self::PM,
            invidious::hidden::CountryCode::PN => Self::PN,
            invidious::hidden::CountryCode::PR => Self::PR,
            invidious::hidden::CountryCode::PS => Self::PS,
            invidious::hidden::CountryCode::PT => Self::PT,
            invidious::hidden::CountryCode::PW => Self::PW,
            invidious::hidden::CountryCode::PY => Self::PY,
            invidious::hidden::CountryCode::QA => Self::QA,
            invidious::hidden::CountryCode::RE => Self::RE,
            invidious::hidden::CountryCode::RO => Self::RO,
            invidious::hidden::CountryCode::RS => Self::RS,
            invidious::hidden::CountryCode::RU => Self::RU,
            invidious::hidden::CountryCode::RW => Self::RW,
            invidious::hidden::CountryCode::SA => Self::SA,
            invidious::hidden::CountryCode::SB => Self::SB,
            invidious::hidden::CountryCode::SC => Self::SC,
            invidious::hidden::CountryCode::SD => Self::SD,
            invidious::hidden::CountryCode::SE => Self::SE,
            invidious::hidden::CountryCode::SG => Self::SG,
            invidious::hidden::CountryCode::SH => Self::SH,
            invidious::hidden::CountryCode::SI => Self::SI,
            invidious::hidden::CountryCode::SJ => Self::SJ,
            invidious::hidden::CountryCode::SK => Self::SK,
            invidious::hidden::CountryCode::SL => Self::SL,
            invidious::hidden::CountryCode::SM => Self::SM,
            invidious::hidden::CountryCode::SN => Self::SN,
            invidious::hidden::CountryCode::SO => Self::SO,
            invidious::hidden::CountryCode::SR => Self::SR,
            invidious::hidden::CountryCode::SS => Self::SS,
            invidious::hidden::CountryCode::ST => Self::ST,
            invidious::hidden::CountryCode::SV => Self::SV,
            invidious::hidden::CountryCode::SX => Self::SX,
            invidious::hidden::CountryCode::SY => Self::SY,
            invidious::hidden::CountryCode::SZ => Self::SZ,
            invidious::hidden::CountryCode::TC => Self::TC,
            invidious::hidden::CountryCode::TD => Self::TD,
            invidious::hidden::CountryCode::TF => Self::TF,
            invidious::hidden::CountryCode::TG => Self::TG,
            invidious::hidden::CountryCode::TH => Self::TH,
            invidious::hidden::CountryCode::TJ => Self::TJ,
            invidious::hidden::CountryCode::TK => Self::TK,
            invidious::hidden::CountryCode::TL => Self::TL,
            invidious::hidden::CountryCode::TM => Self::TM,
            invidious::hidden::CountryCode::TN => Self::TN,
            invidious::hidden::CountryCode::TO => Self::TO,
            invidious::hidden::CountryCode::TR => Self::TR,
            invidious::hidden::CountryCode::TT => Self::TT,
            invidious::hidden::CountryCode::TV => Self::TV,
            invidious::hidden::CountryCode::TW => Self::TW,
            invidious::hidden::CountryCode::TZ => Self::TZ,
            invidious::hidden::CountryCode::UA => Self::UA,
            invidious::hidden::CountryCode::UG => Self::UG,
            invidious::hidden::CountryCode::UM => Self::UM,
            invidious::hidden::CountryCode::US => Self::US,
            invidious::hidden::CountryCode::UY => Self::UY,
            invidious::hidden::CountryCode::UZ => Self::UZ,
            invidious::hidden::CountryCode::VA => Self::VA,
            invidious::hidden::CountryCode::VC => Self::VC,
            invidious::hidden::CountryCode::VE => Self::VE,
            invidious::hidden::CountryCode::VG => Self::VG,
            invidious::hidden::CountryCode::VI => Self::VI,
            invidious::hidden::CountryCode::VN => Self::VN,
            invidious::hidden::CountryCode::VU => Self::VU,
            invidious::hidden::CountryCode::WF => Self::WF,
            invidious::hidden::CountryCode::WS => Self::WS,
            invidious::hidden::CountryCode::YE => Self::YE,
            invidious::hidden::CountryCode::YT => Self::YT,
            invidious::hidden::CountryCode::ZA => Self::ZA,
            invidious::hidden::CountryCode::ZM => Self::ZM,
            invidious::hidden::CountryCode::ZW => Self::ZW,
        }
    }
}

impl From<invidious::hidden::AdaptiveFormat> for crate::global::common::hidden::AdaptiveFormat {
    fn from(value: invidious::hidden::AdaptiveFormat) -> Self {
        Self {
            index: value.index,
            bitrate: value.bitrate,
            init: value.init,
            url: value.url,
            itag: value.itag,
            r#type: value.r#type,
            clen: value.clen,
            lmt: value.lmt,
            projection_type: value.projection_type,
            fps: value.fps,
            container: value.container,
            encoding: value.encoding,
            quality: value.quality,
            resolution: value.resolution,
            quality_label: value.quality_label,
            audio_quality: value.audio_quality,
            audio_sample_rate: value.audio_sample_rate,
            audio_channels: value.audio_channels,
        }
    }
}

impl From<invidious::CommonImage> for crate::global::common::CommonImage {
    fn from(value: invidious::CommonImage) -> Self {
        Self {
            url: value.url,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<invidious::hidden::FormatStream> for crate::global::common::hidden::FormatStream {
    fn from(value: invidious::hidden::FormatStream) -> Self {
        Self {
            url: value.url,
            itag: value.itag,
            r#type: value.r#type,
            quality: value.quality,
            container: value.container,
            encoding: value.encoding,
            quality_label: value.quality_label,
            resolution: value.resolution,
            size: value.size,
        }
    }
}

impl From<invidious::hidden::Caption> for crate::global::common::hidden::Caption {
    fn from(value: invidious::hidden::Caption) -> Self {
        Self {
            label: value.label,
            language: value.language,
            url: value.url,
        }
    }
}

impl From<invidious::hidden::VideoShort> for crate::global::common::hidden::VideoShort {
    fn from(value: invidious::hidden::VideoShort) -> Self {
        Self {
            id: value.id,
            title: value.title,
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
            author: value.author,
            length: value.length,
            views_text: value.views_text,
        }
    }
}

impl From<invidious::video::Video> for crate::global::common::video::Video {
    fn from(value: invidious::video::Video) -> Self {
        Self {
            r#type: value.r#type,
            title: value.title,
            id: value.id,
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
            storyboards: value.storyboards.into_iter().map(|v| v.into()).collect(),
            description: value.description,
            description_html: value.description_html,
            published: value.published,
            published_text: value.published_text,
            keywords: value.keywords,
            views: value.views,
            likes: value.likes,
            dislikes: value.dislikes,
            paid: value.paid,
            premium: value.premium,
            family_friendly: value.family_friendly,
            allowed_regions: value
                .allowed_regions
                .into_iter()
                .map(|v| v.into())
                .collect(),
            genre: value.genre,
            genre_url: value.genre_url,
            author: value.author,
            author_id: value.author_id,
            author_url: value.author_url,
            author_thumbnails: value
                .author_thumbnails
                .into_iter()
                .map(|v| v.into())
                .collect(),
            sub_count_text: value.sub_count_text,
            length: value.length,
            allow_ratings: value.allow_ratings,
            rating: value.rating,
            listed: value.listed,
            live: value.live,
            upcoming: value.upcoming,
            premiere_timestamp: value.premiere_timestamp,
            dash: value.dash,
            adaptive_formats: value
                .adaptive_formats
                .into_iter()
                .map(|v| v.into())
                .collect(),
            format_streams: value.format_streams.into_iter().map(|v| v.into()).collect(),
            captions: value.captions.into_iter().map(|v| v.into()).collect(),
            recommended_videos: value
                .recommended_videos
                .into_iter()
                .map(|v| v.into())
                .collect(),
        }
    }
}

impl From<invidious::CommonVideo> for crate::global::common::CommonVideo {
    fn from(value: invidious::CommonVideo) -> Self {
        Self {
            title: value.title,
            id: value.id,
            author: value.author,
            author_id: value.author_id,
            author_url: value.author_url,
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
            description: value.description,
            description_html: value.description_html,
            views: value.views,
            length: value.length,
            published: value.published,
            published_text: value.published_text,
            premiere_timestamp: value.premiere_timestamp,
            live: value.live,
            premium: value.premium,
            upcoming: value.upcoming,
        }
    }
}

impl From<invidious::CommonPlaylist> for crate::global::common::CommonPlaylist {
    fn from(value: invidious::CommonPlaylist) -> Self {
        Self {
            title: value.title,
            id: value.id,
            thumbnail: value.thumbnail,
            author: value.author,
            author_id: value.author_id,
            author_verified: value.author_verified,
            video_count: value.video_count,
            videos: value.videos.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl From<invidious::CommonPlaylistVideo> for crate::global::common::CommonPlaylistVideo {
    fn from(value: invidious::CommonPlaylistVideo) -> Self {
        Self {
            title: value.title,
            id: value.id,
            length: value.length,
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl From<invidious::CommonChannel> for crate::global::common::CommonChannel {
    fn from(value: invidious::CommonChannel) -> Self {
        Self {
            name: value.name,
            id: value.id,
            url: value.url,
            verified: value.verified,
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
            auto_generated: value.auto_generated,
            subscribers: value.subscribers,
            video_count: value.video_count,
            description: value.description,
            description_html: value.description_html,
        }
    }
}

impl From<invidious::hidden::SearchItem> for crate::global::common::hidden::SearchItem {
    fn from(value: invidious::hidden::SearchItem) -> Self {
        match value {
            invidious::hidden::SearchItem::Video(video) => Self::Video(video.into()),
            invidious::hidden::SearchItem::Playlist(pl) => Self::Playlist(pl.into()),
            invidious::hidden::SearchItem::Channel(ch) => Self::Channel(ch.into()),
        }
    }
}

impl From<invidious::channel::Channel> for crate::global::common::channel::Channel {
    fn from(value: invidious::channel::Channel) -> Self {
        Self {
            name: value.name,
            id: value.id,
            url: value.url,
            banners: value.banners.into_iter().map(|v| v.into()).collect(),
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
            subscribers: value.subscribers,
            total_views: value.total_views,
            joined: value.joined,
            auto_generated: value.auto_generated,
            family_friendly: value.family_friendly,
            description: value.description,
            description_html: value.description_html,
            allowed_regions: value
                .allowed_regions
                .into_iter()
                .map(|v| v.into())
                .collect(),
            latest_videos: value.lastest_videos.into_iter().map(|v| v.into()).collect(),
            related_channels: value
                .related_channels
                .into_iter()
                .map(|v| v.into())
                .collect(),
        }
    }
}

impl From<invidious::hidden::RelatedChannel> for crate::global::common::hidden::RelatedChannel {
    fn from(value: invidious::hidden::RelatedChannel) -> Self {
        Self {
            name: value.name,
            id: value.id,
            url: value.url,
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl From<invidious::hidden::PopularItem> for crate::global::common::hidden::PopularItem {
    fn from(value: invidious::hidden::PopularItem) -> Self {
        Self {
            r#type: value.r#type,
            title: value.title,
            id: value.id,
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
            length: value.length,
            views: value.views,
            author: value.author,
            author_id: value.author_id,
            author_url: value.author_url,
            published: value.published,
            published_text: value.published_text,
        }
    }
}

impl From<invidious::universal::Playlist> for crate::global::common::universal::Playlist {
    fn from(value: invidious::universal::Playlist) -> Self {
        Self {
            title: value.title,
            id: value.id,
            thumbnail: value.thumbnail,
            author: value.author,
            author_id: value.author_id,
            author_thumbnails: value
                .author_thumbnails
                .into_iter()
                .map(|v| v.into())
                .collect(),
            description: value.description,
            description_html: value.description_html,
            video_count: value.video_count,
            views: value.views,
            updated: value.updated,
            listed: value.listed,
            videos: value.videos.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl From<invidious::hidden::PlaylistItem> for crate::global::common::hidden::PlaylistItem {
    fn from(value: invidious::hidden::PlaylistItem) -> Self {
        Self {
            title: value.title,
            id: value.id,
            author: value.author,
            author_id: value.author_id,
            author_url: value.author_url,
            thumbnails: value.thumbnails.into_iter().map(|v| v.into()).collect(),
            index: value.index,
            length: value.length,
        }
    }
}

impl SearchProviderTrait for InvidiousClient {
    fn supports_video(&self) -> bool {
        true
    }

    fn video(
        &self,
        id: &str,
    ) -> Result<crate::global::common::video::Video, Box<dyn std::error::Error>> {
        match self.0.video(id, None) {
            Ok(k) => Ok(k.into()),
            Err(e) => Err(e.into()),
        }
    }

    fn supports_search(&self) -> bool {
        true
    }

    fn search(
        &self,
        filters: &crate::config::Search,
    ) -> Result<Vec<crate::global::common::hidden::SearchItem>, Box<dyn std::error::Error>> {
        match self.0.search(Some(&filters.to_string())) {
            Ok(k) => Ok(k.items.into_iter().map(|v| v.into()).collect()),
            Err(e) => Err(e.into()),
        }
    }

    fn supports_channel(&self) -> bool {
        true
    }

    fn channel(
        &self,
        id: &str,
    ) -> Result<crate::global::common::channel::Channel, Box<dyn std::error::Error>> {
        match self.0.channel(id, None) {
            Ok(k) => Ok(k.into()),
            Err(e) => Err(e.into()),
        }
    }

    fn supports_popular(&self) -> bool {
        true
    }

    fn popular(
        &self,
    ) -> Result<Vec<crate::global::common::hidden::PopularItem>, Box<dyn std::error::Error>> {
        match self.0.popular(None) {
            Ok(k) => Ok(k.items.into_iter().map(|v| v.into()).collect()),
            Err(e) => Err(e.into()),
        }
    }

    fn supports_trending(&self) -> bool {
        true
    }

    fn trending(
        &self,
    ) -> Result<Vec<crate::global::common::CommonVideo>, Box<dyn std::error::Error>> {
        match self.0.trending(None) {
            Ok(k) => Ok(k.videos.into_iter().map(|v| v.into()).collect()),
            Err(e) => Err(e.into()),
        }
    }

    fn supports_playlist(&self) -> bool {
        true
    }

    fn playlist(
        &self,
        id: &str,
    ) -> Result<crate::global::common::universal::Playlist, Box<dyn std::error::Error>> {
        match self.0.playlist(id, None) {
            Ok(k) => Ok(k.into()),
            Err(e) => Err(e.into()),
        }
    }

    fn supports_channel_videos(&self) -> bool {
        true
    }

    fn channel_videos(
        &self,
        id: &str,
    ) -> Result<Vec<crate::global::common::CommonVideo>, Box<dyn std::error::Error>> {
        match self.0.channel_videos(id, None) {
            Ok(k) => Ok(k.videos.into_iter().map(|v| v.into()).collect()),
            Err(e) => Err(e.into()),
        }
    }

    fn supports_channel_playlists(&self) -> bool {
        true
    }

    fn channel_playlists(
        &self,
        id: &str,
    ) -> Result<Vec<crate::global::common::CommonPlaylist>, Box<dyn std::error::Error>> {
        match self.0.channel_playlists(id, None) {
            Ok(k) => Ok(k.playlists.into_iter().map(|v| v.into()).collect()),
            Err(e) => Err(e.into()),
        }
    }
}
