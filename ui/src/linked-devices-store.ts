import { AgentPubKey } from '@holochain/client';
import {
	collectionSignal,
	liveLinksSignal,
	pipe,
	uniquify,
} from '@tnesh-stack/signals';
import { HashType, MemoHoloHashMap, retype } from '@tnesh-stack/utils';

import { LinkedDevicesConfig, defaultLinkedDevicesConfig } from './config.js';
import { LinkedDevicesClient } from './linked-devices-client.js';

export class LinkedDevicesStore {
	constructor(
		public client: LinkedDevicesClient,
		public config: LinkedDevicesConfig = defaultLinkedDevicesConfig,
	) {
		// At startup, clear all the cap grants that might have been left over from an unfinished link agent process
		this.client.clearLinkDevices();
	}

	/** Linked Devices for Agent */

	linkedDevicesForAgent = new MemoHoloHashMap((agent: AgentPubKey) =>
		pipe(
			liveLinksSignal(
				this.client,
				agent,
				() => this.client.getLinkedDevicesForAgent(agent),
				'AgentToLinkedDevices',
			),
			links => uniquify(links.map(l => retype(l.target, HashType.AGENT))),
		),
	);

	/** Link agents */
	linkingAgents = collectionSignal(
		this.client,
		() => this.client.getLinkingAgents(),
		'LinkingAgents',
		1000,
	);
}
