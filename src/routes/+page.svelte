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
  const scoreBtnClasses: Record<number, string> = {
    0: "bg-score-0 text-gray-900",
    1: "bg-score-1 text-gray-900",
    2: "bg-score-2 text-gray-900",
    3: "bg-score-3 text-gray-900",
    4: "bg-score-4 text-gray-900",
    5: "bg-score-5 text-gray-900",
  };

  function handleScore(score: number) {
    if (isExiting) return;
    console.log(`Card "${cards[currentIndex].front}" scored: ${score}`);
    isFlipped = false;
    isExiting = true;

    setTimeout(() => {
      currentIndex = (currentIndex + 1) % cards.length;
      isExiting = false;
    }, 250);
  }

  function flipCard() {
    if (!isExiting) {
      isFlipped = !isFlipped;
    }
  }

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

<main
  class="w-screen h-screen h-dvh bg-[#f0f2f5] dark:bg-[#121212] text-[#1c1e21] dark:text-[#e4e6eb] flex flex-col relative overflow-hidden"
>
  {#if activeTab === "cards"}
    {#if auth.isLoggedIn}
      <div class="flex-1 flex flex-col items-center justify-center p-4 min-h-0">
        <div class="text-sm font-semibold opacity-50 mb-3 tracking-wide">
          {currentIndex + 1} / {cards.length}
        </div>
        <div
          role="button"
          tabindex="0"
          class="w-full max-w-[400px] flex-1 max-h-[55vh] transition-all duration-250 ease-out cursor-pointer [tap-highlight-color:transparent] {isExiting
            ? 'scale-90 opacity-0'
            : ''}"
          onclick={flipCard}
          onkeydown={(e) => e.key === 'Enter' && flipCard()}
        >
          <Flashcard
            front={cards[currentIndex].front}
            back={cards[currentIndex].back}
            bind:flipped={isFlipped}
          />
        </div>
        <div class="mt-3 text-base opacity-40 font-medium">
          {isFlipped ? "Translation" : "Word to learn"}
        </div>
      </div>

      <div class="px-5 pt-3 pb-0">
        <div class="text-center text-base font-semibold opacity-60 mb-2">
          Do you know it?
        </div>
        <div class="flex gap-2 justify-center">
          {#each scores as score}
            <button
              class="flex-1 max-w-[60px] aspect-square rounded-[14px] border-none text-xl font-bold cursor-pointer transition-all duration-150 ease-out text-white [tap-highlight-color:transparent] active:scale-90 disabled:opacity-40 disabled:pointer-events-none {scoreBtnClasses[
                score
              ]}"
              onclick={() => handleScore(score)}
              disabled={isExiting}
            >
              {score}
            </button>
          {/each}
        </div>
        <div class="flex justify-between text-[0.7rem] opacity-40 mt-1.5 px-1">
          <span>No clue</span>
          <span>Perfect</span>
        </div>
      </div>
    {:else}
      <div
        class="flex-1 flex flex-col items-center justify-center text-center p-8"
      >
        <h1
          class="text-4xl mb-4 bg-gradient-to-br from-[#007aff] to-[#5856d6] bg-clip-text text-transparent"
        >
          CardStrike Flash
        </h1>
        <p>Please login to see your flashcards</p>
        <button
          class="mt-8 py-4 px-8 rounded-xl border-none bg-[#007aff] text-white text-lg font-semibold cursor-pointer [tap-highlight-color:transparent]"
          onclick={() => (activeTab = "menu")}
        >
          Go to Menu to Login
        </button>
      </div>
    {/if}
  {:else}
    <Menu onLoginSuccess={() => (activeTab = "cards")} />
  {/if}

  <nav
    class="mt-4 flex bg-white dark:bg-[#1c1c1e] border-t border-black/5 dark:border-white/[0.08] pb-[env(safe-area-inset-bottom)]"
  >
    <button
      class="flex-1 flex flex-col items-center gap-0.5 mt-4 py-4 border-none bg-transparent text-[#1c1e21] dark:text-[#e4e6eb] opacity-40 cursor-pointer transition-opacity duration-200 [tap-highlight-color:transparent] {activeTab ===
      'cards'
        ? '!opacity-100'
        : ''}"
      onclick={() => (activeTab = "cards")}
    >
      <span class="text-base font-semibold tracking-wide">Cards</span>
    </button>
    <button
      class="flex-1 flex flex-col items-center gap-0.5 mt-4 py-4 border-none bg-transparent text-[#1c1e21] dark:text-[#e4e6eb] opacity-40 cursor-pointer transition-opacity duration-200 [tap-highlight-color:transparent] {activeTab ===
      'menu'
        ? '!opacity-100'
        : ''}"
      onclick={() => (activeTab = "menu")}
    >
      <span class="text-base font-semibold tracking-wide">Menu</span>
    </button>
  </nav>
</main>
