<script lang="ts">
    import JSONbig from "json-bigint";
    let token: string = '';
    let realm_id: string = '';
    let webSocket: WebSocket;
    let messages: HTMLUListElement;

    function connectToWebSocket() {
        webSocket = new WebSocket('ws://localhost:3030/ws?token=' + token);
        messages = document.getElementById('messages') as HTMLUListElement;

        webSocket.addEventListener('open', (event) => {
            console.log('WebSocket connection established with token:', token);

            const message = {
                code: 3,
                data: {
                    token: token
                }
            };
            webSocket.send(JSON.stringify(message));
        });

        webSocket.addEventListener('message', (event) => {
            console.log('Message from server ', event.data);
            messages.innerHTML += `<li>${event.data}</li>`;
        });

        webSocket.addEventListener('close', (event) => {
            console.log('WebSocket connection closed');
        });

        webSocket.addEventListener('error', (error) => {
            console.error('WebSocket error: ', JSON.stringify(error));
        });
    }

    function subscribeToRealm() {
        if (webSocket && webSocket.readyState === WebSocket.OPEN) {
            const message = {
                code: 5,
                data: {
                    realm_id: BigInt(realm_id)
                }
            };
            webSocket.send(JSONbig.stringify(message));
            console.log('Subscribed to realm:', realm_id);
        } else {
            console.error('WebSocket is not connected.');
        }
    }
</script>

<input type="text" bind:value={token} placeholder="Enter your token" />
<input type="text" bind:value={realm_id} placeholder="Enter your realm ID" />
<button on:click={connectToWebSocket}>
    Connect to WebSocket
</button>
<button on:click={subscribeToRealm}>
    Subscribe to Realm
</button>

<h2>Messages:</h2>
<ul id="messages"></ul>