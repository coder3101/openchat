<script>
	import { Breadcrumb, BreadcrumbItem, Heading, P } from 'flowbite-svelte';
</script>

<main class="flex flex-col">
	<Heading tag="h2">About Us</Heading>
	<Breadcrumb class="my-4">
		<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
		<BreadcrumbItem href="/about">About</BreadcrumbItem>
	</Breadcrumb>

	<section class="my-5">
		<Heading tag="h3">Our Mission</Heading>
		<P class="mb-3" color="text-gray-500 dark:text-gray-400" weight="light"
			>Welcome to our chat room application! We provide a unique and completely ephemeral chat
			experience in a single chat room, without any sign-up or account creation required. Our
			platform is designed to be simple, user-friendly, and privacy-focused, ensuring that your
			conversations remain secure and confidential.</P
		>
		<P class="mb-3" color="text-gray-500 dark:text-gray-400" weight="light">
			With our application, you can join a chat room and start chatting with anyone in real-time,
			without the need to exchange personal information or create an account. Once you leave the
			chat room, your conversation is completely erased, providing you with a completely ephemeral
			chat experience that ensures your privacy and security.
		</P>
		<P class="mb-3" color="text-gray-500 dark:text-gray-400" weight="light">
			Our platform is perfect for anyone who values their privacy and wants to enjoy a completely
			free and anonymous chat experience. Whether you want to connect with friends, meet new people,
			or simply chat with like-minded individuals, our chat room application has everything you need
			to get started.
		</P>
	</section>
</main>
