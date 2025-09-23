<script lang="ts">
    let { children } = $props();
</script>

<div class="page">
    <div class="card">
        <div class="card-content">
            {@render children?.()}
        </div>
    </div>
</div>

<style>
    .page {
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #d6d6d6;
        width: 100vw;
        height: 100vh;
    }
    .card {
        display: flex;
        height: 500px;
        width: 400px;
        border-radius: 12px;
        background-color: rgba(255, 255, 255, 0.98);
        box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
    }
    .card-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin: auto;
        width: 90%;
        height: 90%;
        font-family: Rubik, sans-serif;
    }
</style>