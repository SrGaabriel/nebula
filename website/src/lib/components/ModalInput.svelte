<script lang="ts">
	let { name, text, type, placeholder } = $props();
</script>

<div class="modal-input-container">
	<span class="modal-text">{text}</span>
	<input name={name} type={type} placeholder={placeholder} class="modal-input" />
</div>

<style>
		.modal-input-container {
				display: flex;
				flex-direction: column;
				margin-bottom: 16px;
		}
		.modal-text {
				font-size: 11px;
				margin-bottom: 4px;
				color: #919191;
				font-weight: 400;
		}
		.modal-input {
				display: flex;
				flex-direction: row;
				align-items: center;
				justify-content: flex-start;
				padding: 8px 12px;
				border: 1px solid #ececec;
				border-radius: 6px;
				font-size: 13px;
				color: var(--text);
				transition: border-color 0.2s ease-in-out;
				height: 16px;
		}
		.modal-input:focus {
				border-color: var(--primary);
				outline: none;
		}
</style>