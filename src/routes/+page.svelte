<script lang="ts">
  import Flashcard from "$lib/components/Flashcard.svelte";
  import Menu from "$lib/components/Menu.svelte";
  import { auth } from "$lib/stores/auth.svelte";
  import { untrack } from "svelte";

  // State
  let cards = $state([
    { front: "Hello", back: "Cześć" },
    { front: "World", back: "Świat" },
    { front: "Apple", back: "Jabłko" },
    { front: "Flashcard", back: "Fiszka" },
  ]);
  let currentIndex = $state(0);
  let isFlipped = $state(false);
  let activeTab = $state<"cards" | "menu">("cards");
  let isExiting = $state(false);

  const scores = [0, 1, 2, 3, 4, 5];

  function handleScore(score: number) {
    if (isExiting) return;
    console.log(`Card "${cards[currentIndex].front}" scored: ${score}`);
    isExiting = true;

    // Brief exit animation, then advance
    setTimeout(() => {
      currentIndex = (currentIndex + 1) % cards.length;
      isFlipped = false;
      isExiting = false;
    }, 250);
  }

  function flipCard() {
    if (!isExiting) {
      isFlipped = !isFlipped;
    }
  }

  // Effect to "fetch" cards when logged in
  $effect(() => {
    if (auth.isLoggedIn) {
      const current = untrack(() => cards);
      cards = [
        ...current,
        { front: "Rust", back: "Rdza (the language)" },
        { front: "Tauri", back: "Tauri (the framework)" },
      ];
    } else {
      currentIndex = 0;
    }
  });
</script>

<main class="app-container">
  {#if activeTab === "cards"}
    {#if auth.isLoggedIn}
      <div class="card-area">
        <div class="card-counter">
          {currentIndex + 1} / {cards.length}
        </div>
        <div class="card-wrapper" class:exiting={isExiting} onclick={flipCard}>
          <Flashcard
            front={cards[currentIndex].front}
            back={cards[currentIndex].back}
            bind:flipped={isFlipped}
          />
        </div>
        <div class="tap-hint">
          {isFlipped ? "Translation" : "Word to learn"}
        </div>
      </div>

      <div class="score-section">
        <div class="score-label">Do you know it?</div>
        <div class="score-buttons">
          {#each scores as score}
            <button
              class="score-btn score-{score}"
              onclick={() => handleScore(score)}
              disabled={isExiting}
            >
              {score}
            </button>
          {/each}
        </div>
        <div class="score-labels">
          <span>No clue</span>
          <span>Perfect</span>
        </div>
      </div>
    {:else}
      <div class="login-screen">
        <h1>CardStrike Flash</h1>
        <p>Please login to see your flashcards</p>
        <button class="primary-btn" onclick={() => (activeTab = "menu")}
          >Go to Menu to Login</button
        >
      </div>
    {/if}
  {:else}
    <Menu />
  {/if}

  <nav class="tab-bar">
    <button
      class="tab"
      class:active={activeTab === "cards"}
      onclick={() => (activeTab = "cards")}
    >
      <span class="tab-label">Cards</span>
    </button>
    <button
      class="tab"
      class:active={activeTab === "menu"}
      onclick={() => (activeTab = "menu")}
    >
      <span class="tab-label">Menu</span>
    </button>
  </nav>
</main>

<style>
  :root {
    --bg-color: #f0f2f5;
    --text-color: #1c1e21;
    --card-bg: #ffffff;
    --surface: #f8f9fa;
    --border-subtle: rgba(0, 0, 0, 0.06);
    --tab-bg: #ffffff;
    overflow: hidden;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      --bg-color: #121212;
      --text-color: #e4e6eb;
      --card-bg: #2a2a2a;
      --surface: #1c1c1e;
      --border-subtle: rgba(255, 255, 255, 0.08);
      --tab-bg: #1c1c1e;
    }
  }

  .app-container {
    width: 100vw;
    height: 100vh;
    height: 100dvh;
    background: var(--bg-color);
    color: var(--text-color);
    display: flex;
    flex-direction: column;
    position: relative;
    overflow: hidden;
  }

  /* Card area */
  .card-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    min-height: 0;
  }

  .card-counter {
    font-size: 0.85rem;
    font-weight: 600;
    opacity: 0.5;
    margin-bottom: 0.75rem;
    letter-spacing: 0.05em;
  }

  .card-wrapper {
    width: 100%;
    max-width: 400px;
    flex: 1;
    max-height: 55vh;
    transition:
      transform 0.25s ease,
      opacity 0.25s ease;
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
  }

  .card-wrapper.exiting {
    transform: scale(0.9);
    opacity: 0;
  }

  .tap-hint {
    margin-top: 0.75rem;
    font-size: 1rem;
    opacity: 0.4;
    font-weight: 500;
  }

  /* Score section */
  .score-section {
    padding: 0.75rem 1.25rem;
    padding-bottom: 0;
  }

  .score-label {
    text-align: center;
    font-size: 1rem;
    font-weight: 600;
    opacity: 0.6;
    margin-bottom: 0.5rem;
  }

  .score-buttons {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
  }

  .score-btn {
    flex: 1;
    max-width: 60px;
    aspect-ratio: 1;
    border-radius: 14px;
    border: none;
    font-size: 1.2rem;
    font-weight: 700;
    cursor: pointer;
    transition:
      transform 0.15s ease,
      opacity 0.15s ease;
    color: white;
    -webkit-tap-highlight-color: transparent;
  }

  .score-btn:active {
    transform: scale(0.9);
  }

  .score-btn:disabled {
    opacity: 0.4;
    pointer-events: none;
  }

  .score-0 {
    background: #f8a9a5;
    color: #1a1a1a;
  }
  .score-1 {
    background: #eebf6f;
    color: #1a1a1a;
  }
  .score-2 {
    background: #f8e783;
    color: #1a1a1a;
  }
  .score-3 {
    background: #e3f69d;
    color: #1a1a1a;
  }
  .score-4 {
    background: #a7f1b9;
    color: #1a1a1a;
  }
  .score-5 {
    background: #7bb6f4;
    color: #1a1a1a;
  }

  .score-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.7rem;
    opacity: 0.4;
    margin-top: 0.35rem;
    padding: 0 0.25rem;
  }

  /* Tab bar */
  .tab-bar {
    margin-top: 1rem;
    display: flex;
    background: var(--tab-bg);
    border-top: 1px solid var(--border-subtle);
    padding-bottom: env(safe-area-inset-bottom);
  }

  .tab {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.2rem;
    margin-top: 1rem;
    padding: 1rem 0;
    border: none;
    background: none;
    color: var(--text-color);
    opacity: 0.4;
    cursor: pointer;
    transition: opacity 0.2s;
    -webkit-tap-highlight-color: transparent;
  }

  .tab.active {
    opacity: 1;
  }

  .tab-icon {
    font-size: 1.3rem;
  }

  .tab-label {
    font-size: 1rem;
    font-weight: 600;
    letter-spacing: 0.02em;
  }

  /* Login screen */
  .login-screen {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 2rem;
  }

  h1 {
    font-size: 2.5rem;
    margin-bottom: 1rem;
    background: linear-gradient(45deg, #007aff, #5856d6);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .primary-btn {
    margin-top: 2rem;
    padding: 1rem 2rem;
    border-radius: 12px;
    border: none;
    background: #007aff;
    color: white;
    font-size: 1.1rem;
    font-weight: 600;
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      Helvetica, Arial, sans-serif;
  }
</style>
