<script lang="ts">
    import {onMount} from "svelte";
    import type {RealmDto, UserDto} from "$lib/api/api";
    import Modal from "$lib/components/Modal.svelte";
		import ModalInput from '$lib/components/ModalInput.svelte';
		import { createRealm, type RealmCreationForm } from '$lib/api/realm';
		import { replaceState } from '$app/navigation';

    export let data;
    let { token, status, activeRealm } = data;
    let user: UserDto = status.self;
    let realms: RealmDto[] = status.realms;

    // modal conditions:
    let showCreateRealmModal: boolean = false;

    onMount(() => {
        console.log('User data:', status);
    })

    function handleCreateRealm() {
        showCreateRealmModal = true;
    }

		function selectRealm(realm: RealmDto) {
			activeRealm = realm;
			// eslint-disable-next-line svelte/no-navigation-without-resolve
			replaceState(`/app/realms/${realm.id}`, {});
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
            <div class="realms">
                {#each realms as realm (realm.id)}
                    <button class="realm-icon" title={realm.name} onclick={() => selectRealm(realm)}>
                        {realm.name.charAt(0).toUpperCase()}
                    </button>
                {/each}
                <button class="create-realm" onclick={handleCreateRealm}>
                    +
                </button>
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
				<p>Realm: {activeRealm?.name}</p>
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
        width: 64px;
        height: 100%;
        border-right: 1px solid rgba(0, 0, 0, 0.1);
    }
    .realms {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-top: 16px;
				gap: 12px;
        width: 100%;
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
        margin: 8px;
				width: 48px;
				height: 48px;
				background-color: var(--primary);
				border-radius: 8px;
				display: flex;
				align-items: center;
				justify-content: center;
				color: white;
				font-weight: bold;
				border: none;
				cursor: pointer;
				transition: all 0.2s ease-in-out;
		}
		.realm-icon:hover {
				transform: scale(1.02);
				box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
				background-color: var(--accent);
				border-radius: 50%;
		}
</style>