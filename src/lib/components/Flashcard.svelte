<script lang="ts">
    let { front, back, flipped = $bindable(false) } = $props();
</script>

<div class="card-container">
    <div class="card" class:flipped>
        <div class="card-face card-front">
            <div class="content">{front}</div>
        </div>
        <div class="card-face card-back">
            <div class="content">{back}</div>
        </div>
    </div>
</div>

<style>
    .card-container {
        width: 100%;
        height: 100%;
        perspective: 1000px;
        user-select: none;
        -webkit-tap-highlight-color: transparent;
        cursor: pointer;
    }

    .card {
        width: 100%;
        height: 100%;
        position: relative;
        transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1);
        transform-style: preserve-3d;
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
        border-radius: 20px;
    }

    .card.flipped {
        transform: rotateY(180deg);
    }

    .card-face {
        position: absolute;
        width: 100%;
        height: 100%;
        backface-visibility: hidden;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 20px;
        background: white;
        padding: 1.5rem;
        box-sizing: border-box;
        border: 1px solid rgba(0, 0, 0, 0.05);
    }

    .card-back {
        transform: rotateY(180deg);
        background: #f8f9fa;
    }

    .content {
        font-size: 2rem;
        font-weight: 700;
        text-align: center;
        color: #1a1a1a;
        word-break: break-word;
    }

    @media (prefers-color-scheme: dark) {
        .card-face {
            background: #2a2a2a;
            border-color: rgba(255, 255, 255, 0.1);
        }
        .card-back {
            background: #333;
        }
        .content {
            color: #f0f0f0;
        }
        .card {
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
        }
    }
</style>
