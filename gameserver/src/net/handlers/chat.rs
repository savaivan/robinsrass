use crate::{
    net::{
        tools::{JsonData, MainCharacter},
        PlayerSession,
    },
    util::cur_timestamp_ms,
};

use super::*;

pub async fn on_jhfffmnkcbf(session: &mut PlayerSession, _: &Jhfffmnkcbf) -> Result<()> {
    session
        .send(
            CMD_GET_FRIEND_LIST_INFO_SC_RSP,
            Fmonbbgkfpp {
                nffnphfnbph: vec![Pbbkojapeaj {
                    jkmlmffobmi: PlayingState::None.into(),
                    eejlmfccjnm: Some(Bhkoekppbaf::default()),
                    ddnklblgmaa: String::from("RobinSR"),
                    mkabddponma: Some(Mionicmdpaa {
                        uid: 727,
                        platform: 3,
                        mjbmlkimpnn: 1,
                        dgojjmfnomj: 201008,
                        kpiphlhfiib: 220005,
                        level: 70,
                        nickname: String::from("Server"),
                        oemkmkkhkde: vec![AssistSimpleInfo {
                            avatar_id: 1008,
                            level: 70,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    niekopdjolm: true,
                }],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_pignjacjgdl(session: &mut PlayerSession, _: &Pignjacjgdl) -> Result<()> {
    session
        .send(
            CMD_GET_PRIVATE_CHAT_HISTORY_SC_RSP,
            Hachagdjilp {
                fepangjbmpa: vec![
                    Gnbekokccfc {
                        mgmicgabebd: MsgType::CustomText.into(),
                        hkkkjdbgiao: cur_timestamp_ms(),
                        anjfbjikchf: "'sync'".to_string(),
                        befmkobhohp: 727,
                        ..Default::default()
                    },
                    Gnbekokccfc {
                        mgmicgabebd: MsgType::CustomText.into(),
                        hkkkjdbgiao: cur_timestamp_ms(),
                        anjfbjikchf: "'mc {mc_id}' mc_id can be set from 8001 to 8006".to_string(),
                        befmkobhohp: 727,
                        ..Default::default()
                    },
                    Gnbekokccfc {
                        mgmicgabebd: MsgType::CustomText.into(),
                        hkkkjdbgiao: cur_timestamp_ms(),
                        anjfbjikchf: "available command:".to_string(),
                        befmkobhohp: 727,
                        ..Default::default()
                    },
                ],
                cibgdjekbja: 727,  // from
                ienomggikon: 1337, // to
                ..Default::default()
            },
        )
        .await
}

// RecvMsgCsReq
pub async fn on_dgaiigecbee(session: &mut PlayerSession, body: &Dgaiigecbee) -> Result<()> {
    let mut json = JsonData::load().await;

    if let Some((cmd, args)) = parse_command(&body.ajlhdpcjand) {
        match cmd {
            "sync" => {
                sync_player(session, json).await?;
                session
                    .send(
                        CMD_REVC_MSG_SC_NOTIFY,
                        Klonpheafip {
                            ggadmjhlomj: body.ggadmjhlomj.clone(),
                            kokadficdfb: body.kokadficdfb,
                            ajlhdpcjand: String::from("Inventory Synced"),
                            mgmicgabebd: body.mgmicgabebd,
                            ghojifhngmc: 727,  // from
                            cmmildghfnl: 1337, // to
                            nmfepfoojic: body.nmfepfoojic,
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
                        Klonpheafip {
                            ggadmjhlomj: body.ggadmjhlomj.clone(),
                            kokadficdfb: body.kokadficdfb,
                            ajlhdpcjand: format!("Set MC to: {mc:#?}"),
                            mgmicgabebd: body.mgmicgabebd,
                            ghojifhngmc: 727,
                            cmmildghfnl: 1337,
                            nmfepfoojic: body.nmfepfoojic,
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
            Ckcjblcacof {
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
            Pkbehgpoein {
                ipnkigfoikl: (2000..3500).collect(),
                idgakomjiio: (1..2000).collect(),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Pkbehgpoein {
                enfnppagfpp: Some(Gkjoiapbbne {
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
            Pkbehgpoein {
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
            Pkbehgpoein {
                enfnppagfpp: Some(Gkjoiapbbne {
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
