<script lang="ts">
    import { auth } from "$lib/stores/auth.svelte";

    let { alwaysVisible = false } = $props();

    function handleAuth() {
        if (auth.isLoggedIn) {
            auth.logout();
        } else {
            auth.login("User");
        }
    }
</script>

<div class="menu-content">
    <h2>Settings</h2>

    <div class="section">
        <div class="section-label">Account</div>
        {#if auth.isLoggedIn}
            <div class="user-info">
                Logged in as <strong>{auth.user?.name}</strong>
            </div>
        {/if}
        <button class="auth-btn" onclick={handleAuth}>
            {auth.isLoggedIn ? "Logout" : "Login"}
        </button>
    </div>

    <div class="section">
        <div class="section-label">About</div>
        <div class="info">CardStrike Flash v0.1</div>
    </div>
</div>

<style>
    .menu-content {
        flex: 1;
        padding: 1.5rem;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    h2 {
        margin: 0;
        font-size: 1.5rem;
        font-weight: 700;
    }

    .section {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .section-label {
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.06em;
        opacity: 0.5;
    }

    .user-info {
        font-size: 0.95rem;
        opacity: 0.8;
    }

    .auth-btn {
        width: 100%;
        padding: 0.9rem;
        border-radius: 12px;
        border: none;
        background: white;
        color: black;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: opacity 0.2s;
        -webkit-tap-highlight-color: transparent;
    }

    .auth-btn:active {
        opacity: 0.8;
    }

    .info {
        text-align: center;
        color: #888;
        font-size: 0.85rem;
    }
</style>
