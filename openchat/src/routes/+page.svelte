<script lang="ts">
	import {
		Breadcrumb,
		Input,
		BreadcrumbItem,
		Heading,
		Label,
		Button,
		Checkbox,
		A,
		Card,
		Select,
		Img,
		Spinner,
		Footer,
		FooterLink,
		FooterCopyright,
		DarkMode,
		FooterLinkGroup,
		Toast
	} from 'flowbite-svelte';

	import logo_white from '$lib/assets/vector/default-monochrome-white.svg';
	import logo_black from '$lib/assets/vector/default-monochrome-black.svg';
	import Chat from '../components/chat.svelte';
	import { join as joinChat, onlineCount } from '$lib/chat';
	import { type ChatMessage, type ChatEvent, isChatEvent } from '$lib/message';
	import { server } from '$lib/config';

	let topics = [
		{ value: 'General', name: 'General' }
		// { value: 'Advise', name: 'Advise' },
		// { value: 'Confession', name: 'Confession' },
		// { value: 'Science and Technology', name: 'Science And Technology' },
		// { value: 'Random', name: 'Random' }
	];

	let selectedTopic = 'General';
	let handle: any = null;
	let messages: ChatMessage[] = [];
	let event: ChatEvent[] = [];
	let connected = false;
	let connecting = false;

	let wb: WebSocket | null = null;
	let toast_message = '';
	let show_toast = false;
	let online = 0;

	let join = async () => {
		connecting = true;
		try {
			let cid = await joinChat(handle);
			let url = new URL('websocket/' + cid, server.replace('http', 'ws'));
			wb = new WebSocket(url);
			wb.onmessage = ({ data }) => {
				let parsed = JSON.parse(data);
				if (isChatEvent(parsed)) {
					if (parsed.name == 'Joined') {
						online += 1;
					} else {
						online -= 1;
					}
				}
				messages = [parsed, ...messages];
			};
			wb.onclose = () => {
				connected = false;
			};
			wb.onerror = () => {
				toast('Communication error', 5);
				connected = false;
			};
			wb.onopen = async () => {
				online = await onlineCount(selectedTopic);
				if (online == -1) {
					toast('Failed to fetch online users count', 5);
				}
			};
			connected = true;
		} catch (err) {
			console.error(err);
			toast(err as string, 5);
		} finally {
			connecting = false;
		}
	};

	let leave = async () => {
		wb?.close();
		event = [];
		messages = [];
		connected = false;
		connecting = false;
	};

	let toast = (text: string, timeout: number) => {
		toast_message = text;
		show_toast = true;
		setTimeout(() => {
			show_toast = false;
		}, timeout * 1000);
	};
</script>

<Toast position="top-right" open={show_toast}>{toast_message}</Toast>
<main class="md:p-8 flex flex-col h-screen">
	<Heading tag="h2" class="ml-8 mt-8">Openchat</Heading>
	<Breadcrumb class="my-4 ml-8">
		<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
	</Breadcrumb>
	{#if !connected}
		<div class="grid place-items-center my-auto">
			<Card class="w-full">
				<div class="px-16 mb-8">
					<Img src={logo_black} alt="vector-logo-white" class="block dark:hidden" />
					<Img src={logo_white} alt="vector-logo-black" class="hidden dark:block" />
				</div>
				<form on:submit|preventDefault={join}>
					<div class="grid gap-6 mb-6">
						<div>
							<Label for="handle" class="mb-2">Chatroom handle</Label>
							<Input
								type="text"
								id="handle"
								required
								bind:value={handle}
								style="font-size: 16px;"
							/>
						</div>
						<div>
							<Label
								>Topic
								<Select class="mt-2" items={topics} bind:value={selectedTopic} />
							</Label>
						</div>
					</div>
					<Checkbox class="mb-6 space-x-1" required color="purple"
						>I agree with the <A href="/privacy_policy">privacy polices</A>.</Checkbox
					>
					<Button type="submit" disabled={connecting} color="primary" class="w-full">
						{#if connecting}
							<Spinner class="mr-3" size="4" color="white" />
						{/if}Proceed
						<svg
							aria-hidden="true"
							class="ml-2 -mr-1 w-5 h-5"
							fill="currentColor"
							viewBox="0 0 20 20"
							xmlns="http://www.w3.org/2000/svg"
							><path
								fill-rule="evenodd"
								d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
								clip-rule="evenodd"
							/></svg
						>
					</Button>
				</form>
			</Card>
		</div>
	{:else}
		<div class="flex flex-row mb-3 ml-8">
			<Heading tag="h5" class="flex-auto">Topic: {selectedTopic} has {online} online</Heading>
			<Button on:click={leave} color="red">Leave</Button>
		</div>

		<Chat
			{handle}
			{messages}
			{online}
			on:send={({ detail }) => {
				wb?.send(detail.text);
			}}
		/>
	{/if}
</main>
{#if !connected}
	<Footer class="mt-auto">
		<DarkMode class="text-lg">
			<svelte:fragment slot="lightIcon">Switch to Light theme</svelte:fragment>
			<svelte:fragment slot="darkIcon">Switch to Dark theme</svelte:fragment>
		</DarkMode>
		<FooterCopyright href="/" by="Openchat" year={2023} />
		<FooterLinkGroup
			ulClass="flex flex-wrap items-center mt-3 text-sm text-gray-500 dark:text-gray-400 sm:mt-0"
		>
			<FooterLink href="/about">About</FooterLink>
			<FooterLink href="/privacy_policy">Privacy Policy</FooterLink>
		</FooterLinkGroup>
	</Footer>
{/if}
