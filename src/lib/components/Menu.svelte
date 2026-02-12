<script lang="ts">
    import { auth } from "$lib/stores/auth.svelte";

    let { isOpen = $bindable(false) } = $props();

    function toggleMenu() {
        isOpen = !isOpen;
    }

    function handleAuth() {
        if (auth.isLoggedIn) {
            auth.logout();
        } else {
            auth.login("User");
        }
        isOpen = false;
    }
</script>

{#if isOpen}
    <div class="overlay" onclick={toggleMenu}></div>
{/if}

<div class="menu" class:open={isOpen}>
    <div class="handle" onclick={toggleMenu}></div>
    <div class="content">
        <h2>Menu</h2>
        <button class="auth-btn" onclick={handleAuth}>
            {auth.isLoggedIn ? "Logout" : "Login"}
        </button>
        <div class="info">
            <p>CardStrike Flash v0.1</p>
        </div>
    </div>
</div>

<style>
    .overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.3);
        backdrop-filter: blur(4px);
        z-index: 99;
    }

    .menu {
        position: fixed;
        bottom: -100%;
        left: 0;
        width: 100%;
        background: #ffffff;
        border-radius: 24px 24px 0 0;
        transition: bottom 0.4s cubic-bezier(0.4, 0, 0.2, 1);
        z-index: 100;
        padding-bottom: env(safe-area-inset-bottom);
        box-shadow: 0 -10px 30px rgba(0, 0, 0, 0.1);
    }

    .menu.open {
        bottom: 0;
    }

    .handle {
        width: 40px;
        height: 5px;
        background: #ddd;
        border-radius: 3px;
        margin: 12px auto;
        cursor: pointer;
    }

    .content {
        padding: 1rem 2rem 2rem;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    h2 {
        margin: 0;
        font-size: 1.5rem;
        font-weight: 700;
    }

    .auth-btn {
        width: 100%;
        padding: 1rem;
        border-radius: 12px;
        border: none;
        background: #007aff;
        color: white;
        font-size: 1.1rem;
        font-weight: 600;
        cursor: pointer;
        transition: opacity 0.2s;
    }

    .auth-btn:active {
        opacity: 0.8;
    }

    .info {
        text-align: center;
        color: #888;
        font-size: 0.9rem;
    }

    @media (prefers-color-scheme: dark) {
        .menu {
            background: #1c1c1e;
        }
        .handle {
            background: #444;
        }
        .auth-btn {
            background: #0a84ff;
        }
    }
</style>
