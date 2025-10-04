<script>
	export let data;
	let { realm } = data;
</script>

<div class="realm-app-container">
	<div class="header">
		<div class="header-center">
			<button class="header-button">
				<a href="/app/realm/{realm.id}/overview" style="text-decoration: none; color: inherit;">
					Overview
				</a>
			</button>
			<button class="header-button">
				<a href="/app/realm/{realm.id}/calendar" style="text-decoration: none; color: inherit;">
					Calendar
				</a>
			</button>
			<button class="header-button">
				<a href="/app/realm/{realm.id}/tasks" style="text-decoration: none; color: inherit;">
					Tasks
				</a>
			</button>
			<button class="header-button">
				<a href="/app/realm/{realm.id}/notes" style="text-decoration: none; color: inherit;">
					Notes
				</a>
			</button>
			<button class="header-button">
				<a href="/app/realm/{realm.id}/files" style="text-decoration: none; color: inherit;">
					Files
				</a>
			</button>
			<button class="header-button">
				<a href="/app/realm/{realm.id}/settings" style="text-decoration: none; color: inherit;">
					Settings
				</a>
			</button>
		</div>
	</div>
	<div class="content">
		<slot />
	</div>
</div>

<style>
		.realm-app-container {
				height: 100%;
				width: 100%;
				display: flex;
				flex-direction: column;
				color: var(--text);
		}
		.header {
				margin-top: 8px;
				width: 100%;
				display: flex;
				align-items: center;
				justify-content: space-around;
				padding: 0 20px;
				box-sizing: border-box;
		}
		.header-center {
				display: flex;
				align-items: center;
				gap: 21px;
				background-color: #fafafa;
				border-radius: 16px;
				box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
				padding: 1px 8px;
        border: 1px solid #e0e0e0;
		}
		.header-button {
				background-color: transparent;
				border: none;
				font-size: 13px;
				cursor: pointer;
				padding: 8px 12px;
				border-radius: 6px;
				font-weight: 600;
				color: #494747;
				transition: background-color 0.2s ease-in-out;
		}
		.content {
				flex-grow: 1;
		}
</style>