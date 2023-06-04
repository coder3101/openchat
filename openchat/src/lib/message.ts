export interface ChatMessage {
	timestamp: number;
	body: string;
	author: string;
}

export interface ChatEvent {
	name: 'Joined' | 'Left';
	identifier: string;
	timestamp: number;
}

export let isChatMessage = (obj: any): obj is ChatMessage => {
	return obj.timestamp !== undefined && obj.body !== undefined && obj.author !== undefined;
};

export let isChatEvent = (obj: any): obj is ChatEvent => {
	return obj.timestamp !== undefined && obj.name !== undefined && obj.identifier !== undefined;
};
