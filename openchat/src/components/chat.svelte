<script lang="ts">
	import { Avatar, Button, Card, Heading, Hr, Input } from 'flowbite-svelte';
	import { isChatMessage, isChatEvent } from '$lib/message';
	import { createEventDispatcher } from 'svelte';

	export let handle: any;
	export let messages: any[];
    export let online: number;

	let msg: string = '';
	let send_message = () => {
		msg = msg.trim();
		if (msg != '') {
			dispatcher('send', { text: msg });
			msg = '';
		}
	};

	let dispatcher = createEventDispatcher();
</script>

<Card class="min-w-full h-full overflow-auto scroll-smooth">
	<div class="flex flex-col flex-auto overflow-auto scroll-smooth">
		<div
			id="messages"
			class="flex flex-col-reverse space-y-4 p-3 overflow-y-auto scrolling-touch scrollbar-hide"
		>
			{#each messages as message}
				{#if isChatMessage(message)}
					{#if message.author != handle}
						<div class="chat-message mt-4">
							<div class="flex items-end">
								<div class="flex flex-col text-xs max-w-xs mx-2 order-2 items-start">
									<div>
										<span
											class="px-4 py-2 rounded-lg inline-block rounded-bl-none bg-gray-300 text-gray-600"
											><b><i>{message.author}</i></b><Hr />
											{message.body}<br /><sub
												>Sent at {new Date(message.timestamp).toLocaleTimeString()}</sub
											></span
										>
									</div>
								</div>
								<Avatar border size="xs">{message.author[0]}</Avatar>
							</div>
						</div>
					{:else}
						<div class="chat-message mt-4">
							<div class="flex items-end justify-end">
								<div class="flex flex-col space-y-2 text-xs max-w-xs mx-2 order-1 items-end">
									<div>
										<span
											class="px-4 py-2 rounded-lg inline-block rounded-br-none bg-blue-600 text-white"
											><b><i>You</i></b><Hr />
											{message.body}<br /><sub
												>Sent at {new Date(message.timestamp).toLocaleTimeString()}</sub
											></span
										>
									</div>
								</div>
								<Avatar border size="xs" class="order-2">{message.author[0]}</Avatar>
							</div>
						</div>
					{/if}
				{:else if isChatEvent(message)}
					<Hr class="m-4"
						><div class="text-xs">
							{message.identifier}
							{message.name} at {new Date(message.timestamp).toLocaleTimeString()}
						</div></Hr
					>
				{/if}
			{/each}
		</div>
	</div>

	<Hr />
	<div class="sticky top-[100vh]">
		<div class="flex p-1">
			<Input
				type="text"
				bind:value={msg}
				placeholder="Type your message @{handle}, hit enter to send..."
				class="flex-auto mr-3"
				style="font-size: 16px;"
				on:keypress={(e) => {
					if (e.key == 'Enter') {
						send_message();
					}
				}}
			/>
			<Button on:click={send_message} color="primary">Send</Button>
		</div>
	</div>
</Card>
