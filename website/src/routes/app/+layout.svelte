<script lang="ts">
    import {onMount} from "svelte";
    import type {RealmDto} from "$lib/api/api";
    import Modal from "$lib/components/Modal.svelte";
		import ModalInput from '$lib/components/ModalInput.svelte';
		import { createRealm, type RealmCreationForm } from '$lib/api/realm';
		import Tooltip from '$lib/components/Tooltip.svelte';

    export let data;
    let { token, status } = data;
    let realms: RealmDto[] = status.realms;

    // modal conditions:
    let showCreateRealmModal: boolean = false;

    onMount(() => {
        console.log('User data:', status);
    })

    function handleCreateRealm() {
        showCreateRealmModal = true;
    }

		async function submitRealmCreation(data: RealmCreationForm) {
			let realmRes = await createRealm(token, data);
			realmRes.fold(
				(err) => {
					alert(`Failed to create realm: ${err}`);
				},
				(realm) => {
					realms = [...realms, realm]; // todo: remove this line when using real-time updates
				}
			)
		}
</script>

<div class="app-container">
    <div class="usable-space">
        <div class="sidebar">
						<div class="misc-buttons">
							<Tooltip text="Overview">
								<a href="/app/overview" style="text-decoration: none;" aria-label="Overview">
									<button class="overview-button" aria-label="Overview">
										<i class="fa-solid fa-gauge"></i>
									</button>
								</a>
							</Tooltip>
							<Tooltip text="Notifications">
								<a href="/app/notifications" style="text-decoration: none;" aria-label="Overview">
									<button class="overview-button" aria-label="Overview">
										<i class="fa-solid fa-bell"></i>
									</button>
								</a>
							</Tooltip>
							<Tooltip text="Search">
								<a href="/app/search" style="text-decoration: none;" aria-label="Overview">
									<button class="overview-button" aria-label="Overview">
										<i class="fa-solid fa-magnifying-glass"></i>
									</button>
								</a>
							</Tooltip>
							<Tooltip text="Settings">
								<a href="/app/search" style="text-decoration: none;" aria-label="Settings">
									<button class="overview-button" aria-label="Settings">
										<i class="fa-solid fa-user-gear"></i>
									</button>
								</a>
							</Tooltip>
						</div>
            <div class="realms">
                {#each realms as realm (realm.id)}
									<Tooltip text={realm.name}>
										<a href="/app/realm/{realm.id}" style="text-decoration: none;">
											<button class="realm-icon">
													{realm.name.charAt(0)}
											</button>
										</a>
									</Tooltip>
                {/each}
							  <Tooltip text="Create new realm">
									<button class="create-realm" onclick={handleCreateRealm}>
											+
									</button>
								</Tooltip>
                <Modal
									title="Create new realm"
									bind:condition={showCreateRealmModal}
									onSubmit={submitRealmCreation}
								>
                    <ModalInput
											name="name"
											type="text"
											text="Name"
											placeholder="My Studies"
										/>
										<ModalInput
											name="description"
											type="text"
											text="Description"
											placeholder="A really cool realm!"
										/>
                </Modal>
            </div>
        </div>
			<div class="page-content">
				<slot />
			</div>
    </div>
</div>

<style>
    .app-container {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100vw;
        height: 100vh;
        font-family: Rubik, sans-serif;
        color: var(--text);
        background-color: var(--background);
    }
    .usable-space {
        display: flex;
        flex-direction: row;
        width: 100%;
        height: 100%;
        border-radius: 12px;
    }
    .sidebar {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;
        width: 52px;
        height: 100%;
        border-right: 1px solid rgba(0, 0, 0, 0.1);
    }
		.misc-buttons {
				display: flex;
				flex-direction: column;
        margin-top: 8px;
				gap: 4px;
		}
		.misc-buttons button {
				display: flex;
				align-items: center;
				justify-content: center;
				cursor: pointer;
				color: #8c8888;
				border-radius: 50%;
				aspect-ratio: 1/1;
				width: 40px;
				font-size: 16px;
				border: none;
				background-color: transparent;
				transition: all 0.25s ease-in-out;
		}
		.misc-buttons button:hover {
        background-color: #e8e8e8;
    }
    .realms {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-top: 8px;
				border-top: 1px solid rgba(0, 0, 0, 0.1);
				padding-top: 16px;
        width: 85%;
				gap: 16px;
    }
    .create-realm {
        cursor: pointer;
        background-color: #d6d6d6;
        color: #8c8888;
        border: 1px solid rgba(0, 0, 0, 0.1);
        border-radius: 50%;
        aspect-ratio: 1/1;
        width: 40px;
        font-size: 24px;
        transition: all 0.2s ease-in-out;
    }
    .create-realm:hover {
        background-color: var(--primary);
        color: white;
        transform: scale(1.1);
    }
		.realm-icon {
				width: 38px;
				height: 38px;
				background-color: var(--primary);
				border-radius: 4px;
				display: flex;
				align-items: center;
				justify-content: center;
				color: white;
				font-weight: bold;
				border: none;
				cursor: pointer;
				transition: all 0.2s ease-in-out;
				text-transform: uppercase;
				font-size: 18px;
				box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
				text-decoration: none;
		}
		.realm-icon:hover {
				transform: scale(1.02);
				box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
				background-color: var(--accent);
				border-radius: 50%;
		}
		.page-content {
				flex-grow: 1;
		}
</style>