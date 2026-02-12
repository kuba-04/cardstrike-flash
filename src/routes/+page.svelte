<script lang="ts">
  import Flashcard from "$lib/components/Flashcard.svelte";
  import Menu from "$lib/components/Menu.svelte";
  import { auth } from "$lib/stores/auth.svelte";

  // State
  let cards = $state([
    { front: "Hello", back: "Cześć" },
    { front: "World", back: "Świat" },
    { front: "Apple", back: "Jabłko" },
    { front: "Flashcard", back: "Fiszka" },
  ]);
  let currentIndex = $state(0);
  let isMenuOpen = $state(false);
  let isFlipped = $state(false);

  // Gesture tracking
  let touchStart = { x: 0, y: 0 };
  let touchEnd = { x: 0, y: 0 };
  let cardOffset = $state({ x: 0, y: 0 });
  let isAnimating = $state(false);

  const SWIPE_THRESHOLD = 100;

  function handleTouchStart(e: TouchEvent) {
    if (isAnimating || isMenuOpen) return;
    touchStart.x = e.touches[0].clientX;
    touchStart.y = e.touches[0].clientY;
  }

  function handleTouchMove(e: TouchEvent) {
    if (isAnimating || isMenuOpen) return;
    touchEnd.x = e.touches[0].clientX;
    touchEnd.y = e.touches[0].clientY;

    // Only track X for left/right swipes
    cardOffset.x = touchEnd.x - touchStart.x;
  }

  async function handleTouchEnd() {
    if (isAnimating || isMenuOpen) return;

    const deltaX = touchEnd.x - touchStart.x;
    const deltaY = touchEnd.y - touchStart.y;

    // Up swipe for menu (deltaY is negative)
    if (deltaY < -100 && Math.abs(deltaX) < 50) {
      isMenuOpen = true;
      resetCard();
      return;
    }

    if (Math.abs(deltaX) > SWIPE_THRESHOLD) {
      if (deltaX > 0) {
        // Swipe Right - Known
        console.log("Known");
        await nextCard(1000);
      } else {
        // Swipe Left - Unknown
        console.log("Unknown");
        await nextCard(-1000);
      }
    } else {
      resetCard();
    }
  }

  function resetCard() {
    cardOffset.x = 0;
    cardOffset.y = 0;
  }

  async function nextCard(exitX: number) {
    isAnimating = true;
    cardOffset.x = exitX;

    // Wait for animation
    await new Promise((r) => setTimeout(r, 300));

    currentIndex = (currentIndex + 1) % cards.length;
    isFlipped = false;
    cardOffset.x = 0;
    isAnimating = false;
  }

  // Effect to "fetch" cards when logged in
  $effect(() => {
    if (auth.isLoggedIn) {
      // Mock fetch
      cards = [
        ...cards,
        { front: "Rust", back: "Rdza (the language)" },
        { front: "Tauri", back: "Tauri (the framework)" },
      ];
    } else {
      currentIndex = 0;
    }
  });
</script>

<main
  class="app-container"
  ontouchstart={handleTouchStart}
  ontouchmove={handleTouchMove}
  ontouchend={handleTouchEnd}
  onmousedown={(e) =>
    handleTouchStart({
      touches: [{ clientX: e.clientX, clientY: e.clientY }],
    } as any)}
  onmousemove={(e) =>
    e.buttons === 1 &&
    handleTouchMove({
      touches: [{ clientX: e.clientX, clientY: e.clientY }],
    } as any)}
  onmouseup={handleTouchEnd}
>
  {#if auth.isLoggedIn}
    <div
      class="card-wrapper"
      style="transform: translateX({cardOffset.x}px) rotate({cardOffset.x /
        20}deg); opacity: {1 - Math.abs(cardOffset.x) / 500}"
    >
      <Flashcard
        front={cards[currentIndex].front}
        back={cards[currentIndex].back}
        bind:flipped={isFlipped}
      />
    </div>

    <div class="swipe-hints">
      <span class="hint-left">Don't know 👈</span>
      <span class="hint-right">👉 Know it</span>
    </div>

    <div class="footer-hint">Pull up for Menu 🔼</div>
  {:else}
    <div class="login-screen">
      <h1>CardStrike Flash</h1>
      <p>Please login to see your flashcards</p>
      <button class="primary-btn" onclick={() => (isMenuOpen = true)}
        >Open Menu to Login</button
      >
    </div>
  {/if}

  <Menu bind:isOpen={isMenuOpen} />
</main>

<style>
  :root {
    --bg-color: #f0f2f5;
    --text-color: #1c1e21;
    overflow: hidden;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      --bg-color: #121212;
      --text-color: #e4e6eb;
    }
  }

  .app-container {
    width: 100vw;
    height: 100vh;
    background: var(--bg-color);
    color: var(--text-color);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    position: relative;
    touch-action: none; /* Prevent browser default scroll/refresh */
  }

  .card-wrapper {
    transition:
      transform 0.2s ease-out,
      opacity 0.2s ease-out;
    will-change: transform, opacity;
  }

  .swipe-hints {
    position: absolute;
    bottom: 15vh;
    width: 100%;
    display: flex;
    justify-content: space-between;
    padding: 0 2rem;
    box-sizing: border-box;
    opacity: 0.4;
    font-weight: 500;
  }

  .footer-hint {
    position: absolute;
    bottom: 2rem;
    font-size: 0.9rem;
    opacity: 0.6;
    animation: bounce 2s infinite;
  }

  @keyframes bounce {
    0%,
    20%,
    50%,
    80%,
    100% {
      transform: translateY(0);
    }
    40% {
      transform: translateY(-10px);
    }
    60% {
      transform: translateY(-5px);
    }
  }

  .login-screen {
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
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      Helvetica, Arial, sans-serif;
  }
</style>
