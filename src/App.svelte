<script>
    import CreateCharacter from "./CreateCharacter.svelte";
    import {Button} from "carbon-components-svelte";
    import "carbon-components-svelte/css/g90.css";

    let showCharacterCreator = false;

    function startNewGame() {
        showCharacterCreator = true;
    }
</script>

<style>
    .new-game-button {
        display: grid;
        place-items: center;
    }
</style>

{#if !showCharacterCreator}
    <div class="new-game-button">
        <Button on:click={startNewGame}>New Game</Button>
    </div>
{/if}

{#if showCharacterCreator}
    <CreateCharacter></CreateCharacter>
{/if}