use super::{Activation, BuffRemove, StateChange, Team};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ArcDPS Combat event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CombatEvent {
    pub time: u64,
    pub src_agent: usize,
    pub dst_agent: usize,
    pub value: i32,
    pub buff_dmg: i32,
    pub overstack_value: u32,
    pub skill_id: u32,
    pub src_instance_id: u16,
    pub dst_instance_id: u16,
    pub src_master_instance_id: u16,
    pub dst_master_instance_id: u16,
    pub iff: Team,
    pub buff: u8,
    pub result: u8,
    pub is_activation: Activation,
    pub is_buff_remove: BuffRemove,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: StateChange,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_off_cycle: u8,
    pub pad61: u8,
    pub pad62: u8,
    pub pad63: u8,
    pub pad64: u8,
}

impl From<&RawCombatEvent> for CombatEvent {
    fn from(raw: &RawCombatEvent) -> Self {
        Self {
            time: raw.time,
            src_agent: raw.src_agent,
            dst_agent: raw.dst_agent,
            value: raw.value,
            buff_dmg: raw.buff_dmg,
            overstack_value: raw.overstack_value,
            skill_id: raw.skill_id,
            src_instance_id: raw.src_instance_id,
            dst_instance_id: raw.dst_instance_id,
            src_master_instance_id: raw.src_master_instance_id,
            dst_master_instance_id: raw.dst_master_instance_id,
            iff: raw.iff.into(),
            buff: raw.buff,
            result: raw.result,
            is_activation: raw.is_activation.into(),
            is_buff_remove: raw.is_buff_remove.into(),
            is_ninety: raw.is_ninety,
            is_fifty: raw.is_fifty,
            is_moving: raw.is_moving,
            is_statechange: raw.is_statechange.into(),
            is_flanking: raw.is_flanking,
            is_shields: raw.is_shields,
            is_off_cycle: raw.is_off_cycle,
            pad61: raw.pad61,
            pad62: raw.pad62,
            pad63: raw.pad63,
            pad64: raw.pad64,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RawCombatEvent {
    pub time: u64,
    pub src_agent: usize,
    pub dst_agent: usize,
    pub value: i32,
    pub buff_dmg: i32,
    pub overstack_value: u32,
    pub skill_id: u32,
    pub src_instance_id: u16,
    pub dst_instance_id: u16,
    pub src_master_instance_id: u16,
    pub dst_master_instance_id: u16,
    pub iff: u8,
    pub buff: u8,
    pub result: u8,
    pub is_activation: u8,
    pub is_buff_remove: u8,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: u8,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_off_cycle: u8,
    pub pad61: u8,
    pub pad62: u8,
    pub pad63: u8,
    pub pad64: u8,
}

impl From<CombatEvent> for RawCombatEvent {
    fn from(event: CombatEvent) -> Self {
        Self {
            time: event.time,
            src_agent: event.src_agent,
            dst_agent: event.dst_agent,
            value: event.value,
            buff_dmg: event.buff_dmg,
            overstack_value: event.overstack_value,
            skill_id: event.skill_id,
            src_instance_id: event.src_instance_id,
            dst_instance_id: event.dst_instance_id,
            src_master_instance_id: event.src_master_instance_id,
            dst_master_instance_id: event.dst_master_instance_id,
            iff: event.iff as u8,
            buff: event.buff,
            result: event.result,
            is_activation: event.is_activation as u8,
            is_buff_remove: event.is_buff_remove as u8,
            is_ninety: event.is_ninety,
            is_fifty: event.is_fifty,
            is_moving: event.is_moving,
            is_statechange: event.is_statechange as u8,
            is_flanking: event.is_flanking,
            is_shields: event.is_shields,
            is_off_cycle: event.is_off_cycle,
            pad61: event.pad61,
            pad62: event.pad62,
            pad63: event.pad63,
            pad64: event.pad64,
        }
    }
}
