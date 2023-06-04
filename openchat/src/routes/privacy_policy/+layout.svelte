<script>
	import '../../app.postcss';
	import { Footer, FooterCopyright, FooterLinkGroup, FooterLink, DarkMode } from 'flowbite-svelte';
</script>

<!-- src/routes/+layout.svelte -->

<svelte:head>
	<title>Openchat - Privacy Policies</title>
</svelte:head>

<div class="p-8 min-h-screen">
	<slot />
</div>

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
