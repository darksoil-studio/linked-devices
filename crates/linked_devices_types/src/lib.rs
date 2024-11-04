use hdi::prelude::*;

mod are_agents_linked;
pub use are_agents_linked::are_agents_linked;

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct AgentToLinkedDevicesLinkTag(pub Vec<LinkedDevicesProof>);

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkedDevicesProof {
    pub linked_devices: LinkedDevices,
    pub signatures: Vec<Signature>,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkedDevices {
    pub agents: Vec<AgentPubKey>,
    pub timestamp: Timestamp,
}

pub const AGENT_TO_LINKED_DEVICES_LINK_INDEX: u8 = 0;

pub fn validate_agents_have_linked_devices(
    agents_with_chain_tops: &Vec<(AgentPubKey, ActionHash)>,
    linked_devices_integrity_zome_name: ZomeName,
) -> ExternResult<ValidateCallbackResult> {
    let dna_info = dna_info()?;

    let Some(linked_devices_integrity_zome_index) = dna_info
        .zome_names
        .into_iter()
        .position(|z| z.eq(&linked_devices_integrity_zome_name))
    else {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Unreachable: there is no 'linked_devices' integrity zome in this DNA",
        )));
    };

    validate_agents_have_linked_devices_with_zome_index(
        agents_with_chain_tops,
        ZomeIndex::new(linked_devices_integrity_zome_index as u8),
    )
}

pub fn validate_agents_have_linked_devices_with_zome_index(
    agents_with_chain_tops: &Vec<(AgentPubKey, ActionHash)>,
    linked_devices_integrity_zome_index: ZomeIndex,
) -> ExternResult<ValidateCallbackResult> {
    let proofs: Vec<LinkedDevicesProof> =
        get_all_linked_devices_proofs(agents_with_chain_tops, linked_devices_integrity_zome_index)?
            .into_iter()
            .collect();

    for (agent_a, _chain_top) in agents_with_chain_tops {
        for (agent_b, _) in agents_with_chain_tops {
            if agent_a.eq(&agent_b) {
                continue;
            };

            let devices_are_linked = are_agents_linked(&agent_a, &agent_b, &proofs);
            if !devices_are_linked {
                return Ok(ValidateCallbackResult::Invalid(format!(
                    "Agents are haven't linked devices: {agent_a} {agent_b}"
                )));
            }
        }
    }

    Ok(ValidateCallbackResult::Valid)
}

fn get_all_linked_devices_proofs(
    agents_with_chain_tops: &Vec<(AgentPubKey, ActionHash)>,
    linked_devices_integrity_zome_index: ZomeIndex,
) -> ExternResult<BTreeSet<LinkedDevicesProof>> {
    let mut result = BTreeSet::new();
    for (agent, chain_top) in agents_with_chain_tops {
        let activity = must_get_agent_activity(agent.clone(), ChainFilter::new(chain_top.clone()))?;
        let tags = filter_agent_to_linked_devices_tags(
            &activity,
            linked_devices_integrity_zome_index.clone(),
        )?;

        for tag in tags {
            for proof in tag.0 {
                result.insert(proof);
            }
        }
    }
    Ok(result)
}

pub fn validate_agent_has_linked_device(
    agent: &AgentPubKey,
    chain_top: &ActionHash,
    linked_device: &AgentPubKey,
    linked_devices_integrity_zome_name: ZomeName,
) -> ExternResult<ValidateCallbackResult> {
    let dna_info = dna_info()?;

    let Some(linked_devices_integrity_zome_index) = dna_info
        .zome_names
        .into_iter()
        .position(|z| z.eq(&linked_devices_integrity_zome_name))
    else {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Unreachable: there is no 'linked_devices' integrity zome in this DNA",
        )));
    };

    validate_agent_has_linked_device_with_zome_index(
        agent,
        chain_top,
        linked_device,
        ZomeIndex::new(linked_devices_integrity_zome_index as u8),
    )
}

pub fn validate_agent_has_linked_device_with_zome_index(
    agent: &AgentPubKey,
    chain_top: &ActionHash,
    linked_device: &AgentPubKey,
    linked_devices_integrity_zome_index: ZomeIndex,
) -> ExternResult<ValidateCallbackResult> {
    let activity = must_get_agent_activity(agent.clone(), ChainFilter::new(chain_top.clone()))?;
    let linked = has_linked_device(
        agent,
        &activity,
        linked_device.clone(),
        linked_devices_integrity_zome_index,
    )?;
    match linked {
        true => Ok(ValidateCallbackResult::Valid),
        false => Ok(ValidateCallbackResult::Invalid(format!(
            "Agent has not linked the given device at the given chain top"
        ))),
    }
}

/** Utility functions */
fn filter_agent_to_linked_devices_tags(
    activity: &Vec<RegisterAgentActivity>,
    linked_devices_integrity_zome_index: ZomeIndex,
) -> ExternResult<Vec<AgentToLinkedDevicesLinkTag>> {
    let agent_to_linked_devices_tags: Vec<AgentToLinkedDevicesLinkTag> = activity
        .iter()
        .filter_map(|activity| match &activity.action.hashed.content {
            Action::CreateLink(create_link) => Some(create_link.clone()),
            _ => None,
        })
        .filter(|create_link| {
            create_link.zome_index == linked_devices_integrity_zome_index
                && create_link.link_type.0 == AGENT_TO_LINKED_DEVICES_LINK_INDEX
        })
        .map(|create_link| {
            let bytes =
                SerializedBytes::from(UnsafeBytes::from(create_link.tag.into_inner().clone()));
            let tag =
                AgentToLinkedDevicesLinkTag::try_from(bytes).map_err(|err| wasm_error!(err))?;
            Ok(tag)
        })
        .collect::<ExternResult<Vec<AgentToLinkedDevicesLinkTag>>>()?;
    Ok(agent_to_linked_devices_tags)
}

fn has_linked_device(
    agent: &AgentPubKey,
    activity: &Vec<RegisterAgentActivity>,
    linked_device: AgentPubKey,
    linked_devices_integrity_zome_index: ZomeIndex,
) -> ExternResult<bool> {
    let agent_to_linked_devices_tags: Vec<AgentToLinkedDevicesLinkTag> =
        filter_agent_to_linked_devices_tags(activity, linked_devices_integrity_zome_index)?;

    let agent_to_linked_device = agent_to_linked_devices_tags
        .into_iter()
        .find(|tag| are_agents_linked(&agent, &linked_device, &tag.0));

    Ok(agent_to_linked_device.is_some())
}
