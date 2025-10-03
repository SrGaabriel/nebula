<script lang="ts">
    import {login} from "$lib/api/auth";

    let username: string = '';
    let password: string = '';

    async function handleLogin() {
        let result = await login(username, password);
        result.fold(
            (err) => {
                alert(`Login failed: ${err}`);
            },
            (data) => {
                document.cookie = `token=${data.token}; path=/;`;
                window.location.href = '/app';
            }
        )
    }
</script>

<span class="auth-title">Login</span>
<input
    type="text"
    placeholder="Username"
    class="auth-input"
    bind:value={username} />
<input
    type="password"
    placeholder="Password"
    class="auth-input"
    bind:value={password} />
<button class="auth-button" on:click={handleLogin}>
    Submit
</button>