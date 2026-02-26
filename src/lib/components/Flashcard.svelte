<script lang="ts">
    let { front, back, flipped = $bindable(false) } = $props();
</script>

<div
  class="w-full h-full [perspective:1000px] select-none cursor-pointer [tap-highlight-color:transparent]"
>
  <div
    class="w-full h-full relative transition-transform duration-500 ease-[cubic-bezier(0.4,0,0.2,1)] [transform-style:preserve-3d] shadow-lg rounded-[20px] {flipped
      ? '[transform:rotateY(180deg)]'
      : ''}"
  >
    <div
      class="absolute inset-0 [backface-visibility:hidden] flex items-center justify-center rounded-[20px] bg-white dark:bg-[#2a2a2a] p-6 box-border border border-black/5 dark:border-white/10"
    >
      <div
        class="text-3xl font-bold text-center text-gray-900 dark:text-[#f0f0f0] break-words"
      >
        {front}
      </div>
    </div>
    <div
      class="absolute inset-0 [backface-visibility:hidden] flex items-center justify-center rounded-[20px] bg-[#f8f9fa] dark:bg-[#333] p-6 box-border border border-black/5 dark:border-white/10 [transform:rotateY(180deg)]"
    >
      <div
        class="text-3xl font-bold text-center text-gray-900 dark:text-[#f0f0f0] break-words"
      >
        {back}
      </div>
    </div>
  </div>
</div>
