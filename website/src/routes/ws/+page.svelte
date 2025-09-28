<script lang="ts">
    let token: string = '';

    function connectToWebSocket() {
        const socket = new WebSocket('ws://localhost:3030/ws', ['Token', token]);

        socket.addEventListener('open', (event) => {
            console.log('WebSocket connection established with token:', token);
            socket.send('Hello Server!');
        });

        socket.addEventListener('message', (event) => {
            console.log('Message from server ', event.data);
        });

        socket.addEventListener('close', (event) => {
            console.log('WebSocket connection closed');
        });

        socket.addEventListener('error', (error) => {
            console.error('WebSocket error: ', error);
        });
    }


</script>

<button on:click={connectToWebSocket}>
    Connect to WebSocket
</button>
<input type="text" bind:value={token} placeholder="Enter your token" />