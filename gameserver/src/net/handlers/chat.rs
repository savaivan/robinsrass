use crate::{
    net::{
        tools::{JsonData, MainCharacter},
        PlayerSession,
    },
    util::cur_timestamp_ms,
};

use super::*;

pub async fn on_nhfajfplkep(session: &mut PlayerSession, _: &Nhfajfplkep) -> Result<()> {
    session
        .send(
            CMD_GET_FRIEND_LIST_INFO_SC_RSP,
            Pbfkjlnnnim {
                amgpdgdnlgd: vec![Leldmbjfheh {
                    gpgdedmpjla: PlayingState::None.into(),
                    cfmiklhjmle: Some(Alcoeanikil::default()),
                    hiljemhhhnk: String::from("RobinSR"),
                    kmclngophda: Some(Hcdpijbnijp {
                        uid: 727,
                        mbdjcknimop: 3,
                        igmaomgegaj: 1,
                        gjlfhjlijon: 201008,
                        jpajpffgnbi: 220005,
                        level: 70,
                        nickname: String::from("Server"),
                        plmbeaaegak: vec![AssistSimpleInfo {
                            avatar_id: 1008,
                            level: 70,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    gjdiplfecfa: true,
                }],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_komknbijgpi(session: &mut PlayerSession, _: &Komknbijgpi) -> Result<()> {
    session
        .send(
            CMD_GET_PRIVATE_CHAT_HISTORY_SC_RSP,
            Ooibcglpnac {
                pgofeopnpbm: vec![Bpifmdladdn {
                    bdjoneohhpj: MsgType::CustomText.into(),
                    phhhfhobhmk: cur_timestamp_ms(),
                    fbelgjfhbkh: ":motorized_wheelchair:".to_string(),
                    nokipdbhglc: 727,
                    ..Default::default()
                }],
                fjbkleaflam: 727,
                oligkfnjkma: 1337,
                ..Default::default()
            },
        )
        .await
}

// RecvMsgCsReq
pub async fn on_bgfjcbbfiek(session: &mut PlayerSession, body: &Bgfjcbbfiek) -> Result<()> {
    let mut json = JsonData::load().await;
    if let Some((cmd, args)) = parse_command(&body.moiplammfad) {
        match cmd {
            "sync" => {
                sync_player(session, json).await?;
                session
                    .send(
                        CMD_REVC_MSG_SC_NOTIFY,
                        Kifdjbodlcc {
                            pofomobijdg: body.pofomobijdg,
                            kjdhmhgjdmc: body.kjdhmhgjdmc,
                            moiplammfad: String::from("Inventory Synced"),
                            bdjoneohhpj: body.bdjoneohhpj,
                            aljhmlmnmhp: 727,
                            djefnoaonkc: 1337,
                        },
                    )
                    .await?;
            }
            "mc" => {
                let mc = MainCharacter::from(
                    args.first()
                        .unwrap_or(&"")
                        .parse::<u32>()
                        .unwrap_or(json.main_character as u32),
                );

                json.main_character = mc;
                json.save().await;

                session
                    .send(
                        CMD_GET_HERO_BASIC_TYPE_INFO_SC_RSP,
                        GetHeroBasicTypeInfoScRsp {
                            retcode: 0,
                            gender: mc.get_gender().into(),
                            cur_basic_type: mc.get_type().into(),
                            basic_type_info_list: vec![HeroBasicTypeInfo {
                                basic_type: mc.get_type().into(),
                                ..Default::default()
                            }],
                            ..Default::default()
                        },
                    )
                    .await?;

                sync_player(session, json).await?;

                session
                    .send(
                        CMD_REVC_MSG_SC_NOTIFY,
                        Kifdjbodlcc {
                            pofomobijdg: body.pofomobijdg,
                            kjdhmhgjdmc: body.kjdhmhgjdmc,
                            moiplammfad: format!("Set MC to: {mc:#?}"),
                            bdjoneohhpj: body.bdjoneohhpj,
                            aljhmlmnmhp: 727,
                            djefnoaonkc: 1337,
                        },
                    )
                    .await?;
            }
            _ => {}
        }
    }

    session
        .send(
            CMD_SEND_MSG_SC_RSP,
            Jhickbdnnii {
                retcode: 0,
                end_time: 0,
            },
        )
        .await
}

fn parse_command(command: &str) -> Option<(&str, Vec<&str>)> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return Option::None;
    }

    Some((parts[0], parts[1..].to_vec()))
}

async fn sync_player(session: &mut PlayerSession, json: JsonData) -> Result<()> {
    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Eckkajafean {
                lminpcphbfp: (2000..3500).collect(),
                oglioehgbal: (1..2000).collect(),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Eckkajafean {
                fihplpphfme: Some(Abcekhjbnmp {
                    avatar_list: json
                        .avatars
                        .values()
                        .map(|avatar| avatar.to_avatar_proto(Option::None, vec![]))
                        .collect::<Vec<_>>(),
                }),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Eckkajafean {
                relic_list: json.relics.iter().map(|v| v.to_relic_proto()).collect(),
                equipment_list: json
                    .lightcones
                    .iter()
                    .map(|v| v.to_equipment_proto())
                    .collect(),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Eckkajafean {
                fihplpphfme: Some(Abcekhjbnmp {
                    avatar_list: json
                        .avatars
                        .values()
                        .map(|avatar| {
                            avatar.to_avatar_proto(
                                json.lightcones
                                    .iter()
                                    .find(|v| v.equip_avatar == avatar.avatar_id),
                                json.relics
                                    .iter()
                                    .filter(|v| v.equip_avatar == avatar.avatar_id)
                                    .collect(),
                            )
                        })
                        .collect(),
                }),
                ..Default::default()
            },
        )
        .await
}
