use crate::net::tools::{self, AvatarJson, JsonData};

use super::*;

pub async fn on_get_all_lineup_data_cs_req(
    session: &mut PlayerSession,
    _body: &GetAllLineupDataCsReq,
) -> Result<()> {
    let player = tools::JsonData::load().await;
    let lineup = LineupInfo {
        extra_lineup_type: ExtraLineupType::LineupNone.into(),
        name: "Squad 1".to_string(),
        njjbfegnhjc: 5,
        bpkggopoppf: 5,
        avatar_list: AvatarJson::to_lineup_avatars(&player),
        ..Default::default()
    };

    session
        .send(
            CMD_GET_ALL_LINEUP_DATA_SC_RSP,
            GetAllLineupDataScRsp {
                lineup_list: vec![lineup],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_cur_lineup_data_cs_req(
    session: &mut PlayerSession,
    _body: &GetCurLineupDataCsReq,
) -> Result<()> {
    let player = tools::JsonData::load().await;
    let mut lineup = LineupInfo {
        extra_lineup_type: ExtraLineupType::LineupNone.into(),
        name: "Squad 1".to_string(),
        njjbfegnhjc: 5,
        bpkggopoppf: 5,
        ..Default::default()
    };

    let avatar_ids = player
        .avatars
        .values()
        .map(|v| v.avatar_id)
        .collect::<Vec<_>>();

    let mut avatars = player
        .lineups
        .iter()
        .filter(|(_slot, v)| v > &&0 && avatar_ids.contains(v))
        .map(|(slot, avatar_id)| {
            player
                .avatars
                .get(avatar_id)
                .unwrap()
                .to_lineup_avatar_proto(*slot)
        })
        .collect::<Vec<LineupAvatar>>();

    lineup.avatar_list.append(&mut avatars);

    session
        .send(
            CMD_GET_CUR_LINEUP_DATA_SC_RSP,
            GetCurLineupDataScRsp {
                lineup: Some(lineup),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_join_lineup_cs_req(
    session: &mut PlayerSession,
    body: &JoinLineupCsReq,
) -> Result<()> {
    // update lineups
    // TODO: FIX THESE SHIT
    {
        let mut player = tools::JsonData::load().await;
        let lineups = &mut player.lineups;
        lineups.insert(
            body.slot,
            if body.base_avatar_id > 8000 {
                player.main_character as u32
            } else {
                body.base_avatar_id
            },
        );
        player.save_lineup().await;
    }

    {
        let player = tools::JsonData::load().await;

        refresh_lineup(session, &player).await?;
    }

    session
        .send(CMD_JOIN_LINEUP_SC_RSP, JoinLineupScRsp::default())
        .await
}

pub async fn on_replace_lineup_cs_req(
    _session: &mut PlayerSession,
    req: &ReplaceLineupCsReq,
) -> Result<()> {
    {
        let mut player = tools::JsonData::load().await;

        let lineups = &mut player.lineups;
        for (slot, avatar_id) in &mut *lineups {
            if let Some(lineup) = req.jkifflmenfn.get(*slot as usize) {
                *avatar_id = if lineup.id > 8000 {
                    player.main_character as u32
                } else {
                    lineup.id
                };
            } else {
                *avatar_id = 0;
            }
        }
        player.save_lineup().await;
    }

    {
        let player = tools::JsonData::load().await;

        refresh_lineup(_session, &player).await?;
    }

    _session
        .send(CMD_JOIN_LINEUP_SC_RSP, JoinLineupScRsp::default())
        .await
}

pub async fn on_quit_lineup_cs_req(
    _session: &mut PlayerSession,
    _: &QuitLineupCsReq,
) -> Result<()> {
    _session
        .send(CMD_JOIN_LINEUP_SC_RSP, JoinLineupScRsp::default())
        .await
}

async fn refresh_lineup(sess: &mut PlayerSession, player: &JsonData) -> Result<()> {
    let lineup = LineupInfo {
        extra_lineup_type: ExtraLineupType::LineupNone.into(),
        name: "Squad 1".to_string(),
        avatar_list: AvatarJson::to_lineup_avatars(player),
        njjbfegnhjc: 5,
        bpkggopoppf: 5,
        ..Default::default()
    };

    sess.send(
        CMD_SCENE_GROUP_REFRESH_SC_NOTIFY,
        Ljihfeagpcl {
            kpfomkdmoce: vec![Jnofbbanolk {
                group_id: 0,
                state: 0,
                kppckepfpce: 0,
                fiiciciambe: player
                    .lineups
                    .iter()
                    .map(|(idx, v)| Gffbkjofnad {
                        fimallpbobk: 0,
                        mggfjbdchjh: 0,
                        glalelmdamm: Some(SceneEntityInfo {
                            actor: Some(SceneActorInfo {
                                avatar_type: AvatarType::AvatarFormalType.into(),
                                base_avatar_id: *v,
                                map_layer: 0,
                                uid: 1337,
                            }),
                            entity_id: idx + 1,
                            group_id: 0,
                            inst_id: 0,
                            ..Default::default()
                        }),
                    })
                    .collect(),
            }],
        },
    )
    .await?;

    sess.send(
        CMD_SYNC_LINEUP_NOTIFY,
        SyncLineupNotify {
            lineup: Some(lineup),
            reason_list: vec![],
        },
    )
    .await
}

pub async fn on_change_lineup_leader_cs_req(
    session: &mut PlayerSession,
    body: &ChangeLineupLeaderCsReq,
) -> Result<()> {
    session
        .send(
            CMD_CHANGE_LINEUP_LEADER_SC_RSP,
            ChangeLineupLeaderScRsp {
                slot: body.slot,
                retcode: 0,
            },
        )
        .await
}
