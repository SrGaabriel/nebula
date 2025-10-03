<script lang="ts">
    import {onMount} from "svelte";
		let formElement: HTMLFormElement;

    let { title,  condition = $bindable(), onSubmit = undefined, onClose = undefined, children} = $props();

    onMount(() => {
        document.addEventListener('keydown', (event) => {
            if (event.key === 'Escape' && condition) {
                close();
            }
            if (event.key === 'Enter' && condition) {
                submit();
            }
        });
    })

    function close() {
        if (onClose) {
					onClose();
        }
				condition = false;
    }

    function submit() {
        if (onSubmit) {
					const data = Object.fromEntries(new FormData(formElement));
					onSubmit(data);
        }
				condition = false;
    }
</script>

{#if condition}
    <div class="modal-background">
        <div class="modal-box">
            <div class="modal-title-container">
                {title}
            </div>
            <div class="modal-content">
							<form bind:this={formElement}>
								{@render children()}
							</form>
            </div>
            <div class="modal-options">
                <button class="close-button" onclick={close}>Close</button>
                <button class="submit-button" onclick={submit}>Submit</button>
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-background {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        animation: opacityFadeIn 0.4s ease-in-out forwards;
    }
    .modal-box {
        background-color: white;
        padding: 20px;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        width: 400px;
        max-width: 90%;
        max-height: 90%;
        overflow-y: auto;
        min-height: 160px;
        display: flex;
        flex-direction: column;
        animation: popUp 0.3s ease-in-out forwards;
    }
    .modal-title-container {
        border-bottom: 1px solid #eee;
        padding-bottom: 10px;
        margin-bottom: 20px;
        font-size: 1em;
        font-weight: bold;
        font-family: 'Rubik', sans-serif;
    }
    .modal-options {
        display: flex;
        margin-top: auto;
        gap: 10px;
        justify-content: flex-end;
    }
    .modal-options button {
        padding: 6px 11px;
        border: none;
        border-radius: 8px;
        cursor: pointer;
        font-family: 'Rubik', sans-serif;
        font-weight: 500;
        transition: all 0.3s ease-in-out;
    }
    .submit-button {
        background-color: var(--primary);
        color: white;
    }
    .close-button:hover {
        color: #ffffff;
        background-color: #e47474;
    }

    @keyframes popUp {
        0% {
            transform: scale(0.4);
            opacity: 0;
        }
        100% {
            transform: scale(1);
            opacity: 1;
        }
    }
    @keyframes opacityFadeIn {
        0% {
            opacity: 0;
        }
        100% {
            opacity: 1;
        }
    }
</style>