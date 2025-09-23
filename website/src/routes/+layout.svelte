<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}

<style>
    @import url('https://fonts.googleapis.com/css2?family=Rubik:ital,wght@0,300..900;1,300..900&display=swap');

    @media (prefers-color-scheme: light) {
        :root {
            --text: #0a1320;
            --background: #ffffff;
            --primary: #4580e5;
            --secondary: #59cafa;
            --accent: #60e5dc;
        }
    }
    @media (prefers-color-scheme: dark) {
        :root {
            --text: #e0e9f5;
            --background: #000000;
            --primary: #2668d9;
            --secondary: #0577a8;
            --accent: #1a9e95;
        }
    }
</style>