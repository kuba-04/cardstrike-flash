<script lang="ts">
  import { auth } from "$lib/stores/auth.svelte";

  interface Props {
    onLoginSuccess?: () => void;
  }
  let { onLoginSuccess }: Props = $props();

  function handleAuth() {
    if (auth.isLoggedIn) {
      auth.logout();
    } else {
      auth.login("User", "");
      onLoginSuccess?.();
    }
  }
</script>

<div class="flex-1 p-6 flex flex-col gap-6">
  <h2 class="m-0 text-2xl font-bold">Settings</h2>

  <div class="flex flex-col gap-3">
    <div class="text-xs font-semibold uppercase tracking-widest opacity-50">
      Account
    </div>
    {#if auth.isLoggedIn}
      <div class="text-[0.95rem] opacity-80">
        Logged in as <strong>{auth.user?.email}</strong>
      </div>
    {/if}
    <button
      class="w-full py-3.5 px-4 rounded-xl border-none bg-white dark:bg-gray-800 text-black dark:text-white text-base font-semibold cursor-pointer transition-opacity duration-200 [tap-highlight-color:transparent] active:opacity-80"
      onclick={handleAuth}
    >
      {auth.isLoggedIn ? "Logout" : "Login"}
    </button>
  </div>

  <div class="flex flex-col gap-3">
    <div class="text-xs font-semibold uppercase tracking-widest opacity-50">
      About
    </div>
    <div class="text-center text-gray-500 dark:text-gray-400 text-sm">
      CardStrike Flash v0.1
    </div>
  </div>
</div>
