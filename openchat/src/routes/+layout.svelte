<script>
	import '../app.postcss';
	import { pwaInfo } from 'virtual:pwa-info';

	$: webManifestLink = pwaInfo ? pwaInfo.webManifest.linkTag : '';
</script>

<svelte:head>
	<title>Openchat - Chat with anyone, anywhere!</title>
	{@html webManifestLink}
</svelte:head>

<div class="min-h-screen">
	<slot />
</div>
