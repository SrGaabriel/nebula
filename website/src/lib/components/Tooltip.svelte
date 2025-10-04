<!-- Tooltip.svelte -->
<script>
	import { crossfade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';

	const [send, receive] = crossfade({
		duration: d => Math.sqrt(d * 200),

		fallback() {
			return {
				duration: 150,
				easing: cubicOut,
				css: t => `
					transform: scale(${t});
					opacity: ${t}
				`
			};
		}
	});

	export let text = "";
	let visible = false;
</script>

<div class="tooltip-wrapper" on:mouseenter={() => visible = true} on:mouseleave={() => visible = false}>
	<slot />

	{#if visible}
		<div class="tooltip" in:receive={{ key: text }} out:send={{ key: text }}>
			{text}
		</div>
	{/if}
</div>

<style>
    .tooltip-wrapper {
        position: relative;
        display: inline-block;
    }

    .tooltip {
        position: absolute;
        bottom: 50%;
        left: 100%;
        background: black;
        color: white;
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        font-size: 0.8rem;
        white-space: nowrap;
        pointer-events: none;
        opacity: 0.9;
    }
</style>
