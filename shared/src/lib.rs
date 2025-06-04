pub mod pencas {
    use crate::{
        matches::{Difficulty, Match, MatchType},
        participants::Participant,
        scoring_rules::{self, ClubMatchOutcome, ScoringRule, apply_scoring_result},
        teams::Team,
    };

    //Aggregate Root
    pub struct Penca {
        participants: Vec<Participant>,
        wildcards: Vec<Team>,
        matches: Vec<Match>,
        format: PencaFormat,
        status: PencaStatus,
    }

    impl Penca {
        pub fn new(format: PencaFormat) -> Self {
            Self {
                participants: vec![],
                wildcards: vec![],
                matches: vec![],
                format,
                status: PencaStatus::Open,
            }
        }

        pub fn add_participant(&mut self, participant: Participant) {
            self.participants.push(participant);
        }

        pub fn add_match(&mut self, match_: Match) {
            self.matches.push(match_);
        }

        pub fn update(&mut self) {
            for match_ in &mut self.matches {
                if !match_.played {
                    break;
                }
                match &match_.match_type {
                    MatchType::Club => {
                        for participant in &mut self.participants {
                            if participant.teams_id.contains(&match_.home_team_id) {
                                if self
                                    .wildcards
                                    .iter()
                                    .any(|team| team.id == match_.away_team_id)
                                {
                                    participant.total_score += apply_scoring_result(
                                        ScoringRule::ClubMatchResult(ClubMatchOutcome::VsWildcard(
                                            Box::new(ClubMatchOutcome::Win),
                                        )),
                                    );
                                } else {
                                    participant.total_score += apply_scoring_result(
                                        ScoringRule::ClubMatchResult(ClubMatchOutcome::Win),
                                    );
                                }
                            }
                            if participant.teams_id.contains(&match_.away_team_id) {
                                if self
                                    .wildcards
                                    .iter()
                                    .any(|team| team.id == match_.home_team_id)
                                {
                                    participant.total_score += apply_scoring_result(
                                        ScoringRule::ClubMatchResult(ClubMatchOutcome::VsWildcard(
                                            Box::new(ClubMatchOutcome::Loss),
                                        )),
                                    );
                                } else {
                                    participant.total_score += apply_scoring_result(
                                        ScoringRule::ClubMatchResult(ClubMatchOutcome::Loss),
                                    );
                                }
                            }
                        }
                    }
                    MatchType::Predicted {
                        level,
                        selected_player,
                    } => match level {
                        Difficulty::Simple => {}
                        Difficulty::Complex => {}
                        Difficulty::Special => {}
                    },
                }
            }
        }
    }

    pub enum PencaFormat {
        Traditional,
        ClubMode,
        MixMode,
    }

    pub enum PencaStatus {
        Open,
        Closed,
        InProgress,
        Finished,
    }
}

pub mod participants {
    use crate::participant_id::ParticipantId;
    use crate::player_id::PlayerId;
    use crate::teams_id::TeamId;

    pub struct Participant {
        pub id: ParticipantId,
        pub name: String,
        pub teams_id: [TeamId; 2],
        pub friend_player_id: PlayerId,
        pub division: Division,
        pub total_score: i32,
        pub matches_played: u8,
        pub country: String,
    }
    impl Participant {
        pub fn new(
            name: String,
            country: String,
            teams_id: [TeamId; 2],
            friend_player_id: PlayerId,
            division: Division,
            total_score: i32,
            matches_played: u8,
        ) -> Self {
            Self {
                id: ParticipantId::new(),
                name,
                teams_id,
                friend_player_id,
                division,
                total_score,
                matches_played,
                country,
            }
        }
    }
    pub enum Division {
        A,
        B,
        C,
    }
}

pub mod participant_id {
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct ParticipantId(Uuid);
    impl ParticipantId {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
    }
}

pub mod teams {
    use crate::{leagues_id::LeagueId, teams_id::TeamId};
    pub struct Team {
        pub id: TeamId,
        pub name: String,
        pub league_id: LeagueId,
    }
    impl Team {
        pub fn new(name: String, league_id: LeagueId) -> Self {
            Self {
                id: TeamId::new(),
                name,
                league_id,
            }
        }
    }
}

pub mod teams_id {
    use uuid::Uuid;

    #[derive(Clone, PartialEq)]
    pub struct TeamId(Uuid);
    impl TeamId {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
    }
}

pub mod leagues {
    use crate::leagues_id::LeagueId;

    pub struct League {
        pub id: LeagueId,
        name: String,
    }

    impl League {
        pub fn new(name: String) -> Self {
            Self {
                id: LeagueId::new(),
                name,
            }
        }
    }
}

pub mod leagues_id {
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct LeagueId(Uuid);

    impl LeagueId {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
    }
}

pub mod player {
    use crate::{player_id::PlayerId, teams_id::TeamId};

    pub struct Player {
        pub id: PlayerId,
        name: String,
        current_team_id: TeamId,
        goals: u32,
        self_goals: u32,
        assists: u32,
        yellow_cards: u32,
        red_cards: u32,
    }
    impl Player {
        pub fn new(name: String, current_team_id: TeamId) -> Self {
            Self {
                id: PlayerId::new(),
                name,
                current_team_id,
                goals: 0,
                self_goals: 0,
                assists: 0,
                yellow_cards: 0,
                red_cards: 0,
            }
        }
    }
}

pub mod player_id {
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct PlayerId(Uuid);

    impl PlayerId {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
    }
}
pub mod match_repository {
    use crate::{matches::Match, matches_id::MatchId};

    pub trait MatchRepository {
        fn get_by_id(&self, id: &MatchId) -> Result<Option<Match>, String>;
        fn save(&mut self, game_match: Match) -> Result<(), String>;
    }
}
pub mod matches {
    use crate::{matches_id::MatchId, player::Player, teams_id::TeamId};
    use chrono::NaiveDateTime;
    pub struct Match {
        pub home_team_id: TeamId,
        pub away_team_id: TeamId,
        pub home_goals: u8,
        pub away_goals: u8,
        pub date: NaiveDateTime,
        pub is_friendly: bool,
        pub match_type: MatchType,
        pub id: MatchId,
        pub played: bool,
    }

    impl Match {
        pub fn new(
            home_team_id: TeamId,
            away_team_id: TeamId,
            date: NaiveDateTime,
            is_friendly: bool,
            match_type: MatchType,
        ) -> Self {
            Self {
                home_team_id,
                away_team_id,
                home_goals: 0,
                away_goals: 0,
                date,
                is_friendly,
                match_type,
                id: MatchId::new(),
                played: false,
            }
        }
        pub fn insert_result(&mut self, home_goals: u8, away_goals: u8) {
            self.home_goals = home_goals;
            self.away_goals = away_goals;
            self.played = true;
        }
    }

    pub enum MatchType {
        Club,
        Predicted {
            level: Difficulty,
            selected_player: Option<Player>,
        },
    }

    pub enum Difficulty {
        Simple,
        Complex,
        Special,
    }
}

pub mod matches_id {
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct MatchId(Uuid);

    impl MatchId {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
    }
}
pub mod predictions {
    use crate::{matches_id::MatchId, participant_id::ParticipantId};

    pub struct Prediction {
        participant_id: ParticipantId,
        match_id: MatchId,
        expected_result: MatchResult,
        predicted_home_goals: u8,
        predicted_away_goals: u8,
    }

    pub enum MatchResult {
        Win,
        Draw,
        Loss,
        WinByLargeMargin,
        LossByLargeMargin,
    }
}

pub mod scoring_rules {
    pub enum ScoringRule {
        ClubMatchResult(ClubMatchOutcome),
        FriendPlayerEvent(FriendPlayerEvent),
        PredictedMatchResult(PredictionOutcome),
        SelectedPlayerEvent(SelectedPlayerEvent),
    }

    pub enum ClubMatchOutcome {
        Win,
        Loss,
        Draw,
        WinByLargeMargin,
        LossByLargeMargin,
        VsWildcard(Box<ClubMatchOutcome>),
    }
    pub enum FriendPlayerEvent {
        Goal,
        Assist,
    }
    pub enum PredictionOutcome {
        ExactSimple,
        GeneralSimple,
        ExactComplex,
        GeneralComplex,
        ExactSpecial,
        GeneralSpecial,
    }
    pub enum SelectedPlayerEvent {
        Goal,
        Assist,
        YellowCard,
        RedCard,
        OwnGoal,
        MissedPenalty,
    }
    pub fn apply_scoring_result(rule: ScoringRule) -> i32 {
        match rule {
            ScoringRule::ClubMatchResult(outcome) => match outcome {
                ClubMatchOutcome::Win => 3,
                ClubMatchOutcome::WinByLargeMargin => 4,
                ClubMatchOutcome::Draw => 1,
                ClubMatchOutcome::Loss => 0,
                ClubMatchOutcome::LossByLargeMargin => -1,
                ClubMatchOutcome::VsWildcard(inner) => match *inner {
                    ClubMatchOutcome::Win => 6,
                    ClubMatchOutcome::WinByLargeMargin => 8,
                    ClubMatchOutcome::Draw => 2,
                    ClubMatchOutcome::Loss => -3,
                    ClubMatchOutcome::LossByLargeMargin => -4,
                    _ => 0,
                },
            },
            ScoringRule::FriendPlayerEvent(event) => match event {
                FriendPlayerEvent::Goal => 2,
                FriendPlayerEvent::Assist => 1,
            },
            ScoringRule::PredictedMatchResult(po) => match po {
                PredictionOutcome::ExactSimple => 10,
                PredictionOutcome::GeneralSimple => 3,
                PredictionOutcome::ExactComplex => 15,
                PredictionOutcome::GeneralComplex => 6,
                PredictionOutcome::ExactSpecial => 20,
                PredictionOutcome::GeneralSpecial => 9,
            },
            ScoringRule::SelectedPlayerEvent(ev) => match ev {
                SelectedPlayerEvent::Goal => 4,
                SelectedPlayerEvent::Assist => 2,
                SelectedPlayerEvent::YellowCard => -1,
                SelectedPlayerEvent::RedCard => -2,
                SelectedPlayerEvent::OwnGoal => -4,
                SelectedPlayerEvent::MissedPenalty => -4,
            },
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use std::clone;

    use chrono::NaiveDate;

    use crate::{
        leagues::League,
        matches, matches_id,
        participants::{self, Participant},
        pencas::{Penca, PencaFormat},
        player::Player,
        teams::Team,
    };

    #[test]
    fn e2e_full() {
        let mut penca = Penca::new(PencaFormat::MixMode);
        let league1 = League::new("Champions League".to_string());
        let mundial = League::new("Mundial femenino".to_string());
        let real_team = Team::new("Real Madrid".to_string(), league1.id.clone());
        let barca_team = Team::new("Barcelona".to_string(), league1.id.clone());
        let france_team = Team::new("Francia".to_string(), mundial.id.clone());
        let argentina_team = Team::new("Argentina".to_string(), mundial.id.clone());
        let player1 = Player::new("Federico Valverde".to_string(), real_team.id.clone());
        let player2 = Player::new("Lionel Messi".to_string(), argentina_team.id.clone());
        let participant1 = Participant::new(
            "Diego".to_string(),
            "Uruguay".to_string(),
            [real_team.id.clone(), france_team.id.clone()],
            player1.id.clone(),
            participants::Division::A,
            0,
            0,
        );
        let participant2 = Participant::new(
            "Diego2".to_string(),
            "Argentina".to_string(),
            [barca_team.id.clone(), argentina_team.id.clone()],
            player2.id.clone(),
            participants::Division::A,
            0,
            0,
        );
        penca.add_participant(participant1.id.clone());
        penca.add_participant(participant2.id.clone());
        let mut match_m = matches::Match::new(
            argentina_team.id.clone(),
            france_team.id.clone(),
            chrono::NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
                chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ),
            false,
            matches::MatchType::Club,
        );
        let mut match_c = matches::Match::new(
            real_team.id.clone(),
            barca_team.id.clone(),
            chrono::NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
                chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ),
            false,
            matches::MatchType::Club,
        );
        penca.add_match(match_c.id.clone());
        penca.add_match(match_m.id.clone());
        match_c.insert_result(2, 1);
        match_m.insert_result(1, 0);
        penca.update();
    }
}
